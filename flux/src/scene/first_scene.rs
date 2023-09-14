use std::time::Instant;

use atlas::{
    components::{
        camera::{Camera, Frustrum},
        mesh_renderer::MeshRendererSystem,
        particle_renderer::ParticleRenderer,
        skybox_renderer::SkyboxRendererSystem,
        sprite_renderer::SpriteRendererSystem,
        text_renderer::{TextRenderer, TextRendererSystem},
        transform::Transform,
    },
    entity_manager::EntityManager,
    event_bus::{create_event_queue, EventReader, EventSender},
    game_entities::{
        bullet::BulletEntity, hud::update_hud, player_ship::PlayerShip, ui_label::UiLabel,
        GameEntity,
    },
    game_root::GameError,
    graphics::graphics_context::{ContextEvent, GraphicsContext},
    resource_manager::{font::Font, scene_resource_manager::SceneResourceManager, ResourceManager},
    scene::{Scene, SceneEvent},
    systems::{
        asteroid_detonator::detonate_asteroids,
        bullet_detonator::process_bullet_events,
        bullet_renderer::BulletRenderer,
        bullet_system::update_bullets,
        collider_renderer::CollisionRenderer,
        collision_system::CollisionSystem,
        health_renderer::HealthRendererSystem,
        particle_system::update_particles,
        physical_simulation::PhysicalSimulation,
        player_controller::{GameEvent, PlayerController},
        text_update::{update_text, TextChangeEvent},
        trail_renderer::TrailRenderer, player_follower::follow_player,
    },
};
use glam::{Quat, Vec3};

use crate::game_objects::asteroids::asteroids;

pub struct FirstScene {
    entity_manager: EntityManager,
    resource_manager: SceneResourceManager,

    render_fn: Box<dyn FnMut(f32, &GraphicsContext, &mut EntityManager, &EventReader)>,
    physical_fn: Box<
        dyn FnMut(
            f32,
            f32,
            &mut EntityManager,
            &mut SceneResourceManager,
            &mut EventReader,
            &mut EventSender,
        ),
    >,

    event_reader: EventReader,
    event_sender: EventSender,
}

impl Scene for FirstScene {
    fn run(
        &mut self,
        graphics_context: &mut atlas::graphics::graphics_context::GraphicsContext,
    ) -> SceneEvent {
        let fps_counter = self.create_label(Vec3::new(50.0, 50.0, 0.0));
        let physics_counter = self.create_label(Vec3::new(50.0, 100.0, 0.0));

        let (mut now, mut prev_phys) = (Instant::now(), Instant::now());

        let mut physics_delta: u128 = 0;

        let mut fps = 0.0;
        let mut physics_fps = 0.0;
        const PHYSICS_DELTA: u128 = 1_000_000_000 / 128;

        let mut time = now.elapsed().as_nanos();

        loop {
            let current = now.elapsed().as_nanos();
            let delta = current - time;
            time = current;

            physics_delta += delta;

            fps = fps * 0.9 + 0.1 * (1_000_000_000.0 / delta as f32);

            self.event_sender.write(TextChangeEvent::TextChange(
                fps_counter,
                format!("FPS: {}", fps),
            ));

            self.event_sender.write(TextChangeEvent::TextChange(
                physics_counter,
                format!("Physix: {}", physics_fps),
            ));

            (self.render_fn)(
                time as f32 / 1_000_000_000.0,
                &graphics_context,
                &mut self.entity_manager,
                &self.event_reader,
            );

            update_text(&mut self.entity_manager, &mut self.event_reader);

            graphics_context.display();

            while physics_delta > PHYSICS_DELTA {
                physics_delta -= PHYSICS_DELTA;
                physics_fps = physics_fps * 0.9
                    + 0.1 * (1_000_000_000.0 / prev_phys.elapsed().as_nanos() as f32);

                (self.physical_fn)(
                    time as f32 / 1_000_000_000.0,
                    1.0 / 120.0,
                    &mut self.entity_manager,
                    &mut self.resource_manager,
                    &mut self.event_reader,
                    &mut self.event_sender,
                );
            }

            self.poll_events(graphics_context);

            if let Some(result) = self.process_events(graphics_context) {
                return result;
            }
        }
    }
}

impl FirstScene {
    pub fn new(graphics_context: &mut GraphicsContext) -> Result<Box<dyn Scene>, GameError> {
        let mut entity_manager = EntityManager::new();
        let mut resource_manager = SceneResourceManager::build("first")?;

        let render_fn = Box::new(Self::create_renderer(
            &mut resource_manager,
            graphics_context,
        ));
        let physical_fn = Box::new(Self::create_physics());

        asteroids(&mut entity_manager, &mut resource_manager, graphics_context)?;

        graphics_context.cursor_lock(true);

        let (event_sender, event_reader) = create_event_queue();

        Ok(Box::new(FirstScene {
            entity_manager,
            resource_manager,
            event_sender,
            event_reader,
            render_fn,
            physical_fn,
        }))
    }

    fn create_physics() -> impl FnMut(
        f32,
        f32,
        &mut EntityManager,
        &mut SceneResourceManager,
        &mut EventReader,
        &mut EventSender,
    ) {
        let mut player_controller = PlayerController::new();
        let mut physical_simulation = PhysicalSimulation::new(1.0 / 120.0);

        move |time: f32,
              delta: f32,
              entity_manager: &mut EntityManager,
              resource_manager: &mut SceneResourceManager,
              event_reader: &mut EventReader,
              event_sender: &mut EventSender| {
            CollisionSystem::resolve_collisions(time, delta, event_sender, entity_manager);

            physical_simulation.integrate_movement(entity_manager);

            update_bullets(entity_manager, event_sender, delta);
            update_hud(entity_manager, event_sender);

            process_bullet_events(
                event_reader,
                event_sender,
                resource_manager,
                entity_manager,
                delta,
            );

            detonate_asteroids(entity_manager, resource_manager, event_reader);

            update_particles(entity_manager, delta);


            player_controller.control(
                physical_simulation.delta(),
                entity_manager,
                event_reader,
                event_sender,
            );
        }
    }

    fn create_renderer(
        res_man: &mut SceneResourceManager,
        graphics_context: &GraphicsContext,
    ) -> impl FnMut(f32, &GraphicsContext, &mut EntityManager, &EventReader) {
        let mut bullet_renderer = BulletRenderer::new(res_man.get("plasma").res);
        let mesh_renderer = MeshRendererSystem::new(res_man.get("phong").res);
        let skybox_renderer = SkyboxRendererSystem::new(res_man.get("basic").res);
        let mut text_renderer = TextRendererSystem::new(res_man.get("basic").res);
        let particle_renderer = ParticleRenderer::new(res_man.get("flat").res);
        let sprite_renderer = SpriteRendererSystem::new(res_man.get("basic").res);
        let mut collision_renderer = CollisionRenderer::new(res_man.get("collision").res);
        let mut trail_renderer = TrailRenderer::new(res_man.get("trail").res);
        let health_renderer = HealthRendererSystem::new(res_man.get("bar").res);

        let (width, height) = graphics_context.dimensions();

        let ui_camera = Camera::new(
            Frustrum::orthogonal(width as f32, height as f32),
            Vec3::new(0.0, 0.0, 0.0),
            Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 0.0),
        );

        move |time: f32,
              graphics_context: &GraphicsContext,
              entity_manager: &mut EntityManager,
              event_reader: &EventReader| {
            let mut context = graphics_context.new_context();
            follow_player(entity_manager);

            let camera_kit = entity_manager
                .iter()
                .map(|player: &GameEntity<PlayerShip>| {
                    (
                        &player.transform,
                        &player.entity.camera,
                        &player.entity.physical_body,
                    )
                })
                .next();

            if let Some((camera_transform, camera, _body)) = camera_kit {
                graphics_context.depth_write(false);
                skybox_renderer.render(&mut context, entity_manager, camera, camera_transform);
                graphics_context.depth_write(true);

                mesh_renderer.render(&mut context, entity_manager, camera, camera_transform);

                graphics_context.depth_write(false);
                particle_renderer.render(&mut context, entity_manager, camera, camera_transform);

                bullet_renderer.render_bullets(
                    &mut context,
                    entity_manager,
                    camera,
                    camera_transform,
                );
                collision_renderer.render(
                    &mut context,
                    time,
                    entity_manager,
                    camera,
                    camera_transform,
                );

                let player = entity_manager.iter::<PlayerShip>().next().unwrap();

                trail_renderer.render(
                    &mut context,
                    event_reader,
                    &player.entity.physical_body,
                    &entity_manager,
                    camera,
                    camera_transform,
                );

                graphics_context.depth_write(true);

                text_renderer.render(&mut context, entity_manager, &ui_camera);

                sprite_renderer.render(&mut context, entity_manager, &ui_camera);
                health_renderer.render(&mut context, entity_manager, &ui_camera);
            }
        }
    }

    fn poll_events(&mut self, graphics_context: &mut GraphicsContext) {
        graphics_context.poll_events(&mut self.event_sender);
    }

    fn process_events(&mut self, graphics_context: &mut GraphicsContext) -> Option<SceneEvent> {
        self.event_reader.read(|event| match event {
            GameEvent::ShootPlasmaBullet(transform, bullet) => {
                self.entity_manager.add_at(bullet, transform);
            }
            GameEvent::RemoveBullet(entity) => self.entity_manager.remove::<BulletEntity>(entity),
        });

        let mut action = None;
        self.event_reader.read(|event| match event {
            ContextEvent::Resized(width, height) => {
                graphics_context.set_viewport(width, height);
            }
            ContextEvent::Close => action = Some(SceneEvent::Exit),
        });
        action
    }

    fn create_label(&mut self, position: Vec3) -> usize {
        let font: Font = self.resource_manager.get("main").res;
        self.entity_manager.add_at(
            UiLabel {
                renderer: TextRenderer::new(Transform::new(), "", font),
            },
            Transform::pos(position),
        )
    }
}
