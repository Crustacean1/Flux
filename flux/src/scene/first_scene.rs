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
    game_entities::{bullet::BulletEntity, player_ship::PlayerShip, ui_label::UiLabel, GameEntity},
    game_root::GameError,
    graphics::graphics_context::{ContextEvent, GraphicsContext},
    resource_manager::{font::Font, scene_resource_manager::SceneResourceManager, ResourceManager},
    scene::{Scene, SceneEvent},
    systems::{
        bullet_renderer::BulletRenderer,
        bullet_system::update_bullets,
        collider_renderer::CollisionRenderer,
        collision_system::CollisionSystem,
        hud_refresher::HudRefresher,
        particle_system::update_particles,
        physical_simulation::PhysicalSimulation,
        player_controller::{GameEvent, PlayerController},
        text_update::{update_text, TextChangeEvent},
        trail_renderer::{self, TrailRenderer},
    },
};
use glam::Vec3;

use crate::game_objects::asteroids::asteroids;

pub struct FirstScene {
    ui_camera: Camera,
    entity_manager: EntityManager,
    resource_manager: SceneResourceManager,

    trail_renderer: TrailRenderer,
    collision_renderer: CollisionRenderer,
    hud_refresher: HudRefresher,
    bullet_renderer: BulletRenderer,
    particle_renderer: ParticleRenderer,
    sprite_renderer: SpriteRendererSystem,
    text_renderer: TextRendererSystem,
    mesh_renderer: MeshRendererSystem,
    skybox_renderer: SkyboxRendererSystem,
    player_controller: PlayerController,
    physical_simulation: PhysicalSimulation,

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

        let (mut now, mut prev, mut prev_phys) = (Instant::now(), Instant::now(), Instant::now());

        let mut physics_delta: u128 = 0;

        let mut fps = 0.0;
        let mut physics_fps = 0.0;

        loop {
            let delta = now.elapsed().as_nanos();
            physics_delta += delta;
            now = Instant::now();

            fps = fps * 0.9 + 0.1 * (1_000_000_000.0 / delta as f32);
            self.event_sender.write(TextChangeEvent::TextChange(
                fps_counter,
                format!("FPS: {}", fps),
            ));

            self.event_sender.write(TextChangeEvent::TextChange(
                physics_counter,
                format!("Physix: {}", physics_fps),
            ));

            self.render(graphics_context);

            update_text(&mut self.entity_manager, &mut self.event_reader);

            graphics_context.display();

            if physics_delta > self.physical_simulation.delta() {
                physics_delta -= self.physical_simulation.delta();
                physics_fps = physics_fps * 0.9
                    + 0.1 * (1_000_000_000.0 / prev_phys.elapsed().as_nanos() as f32);
                prev_phys = Instant::now();

                self.hud_refresher.update(&mut self.entity_manager);
                update_bullets(
                    &mut self.entity_manager,
                    &mut self.event_sender,
                    self.physical_simulation.delta(),
                );

                //CollisionSystem::resolve_collisions(&self.entity_manager);
                self.physical_simulation
                    .integrate_movement(&mut self.entity_manager);
                update_particles(&mut self.entity_manager, self.physical_simulation.delta());

                self.player_controller.control(
                    self.physical_simulation.delta(),
                    &mut self.entity_manager,
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

        let (
            bullet_renderer,
            mesh_renderer,
            skybox_renderer,
            player_controller,
            physical_simulation,
            text_renderer,
            sprite_renderer,
            particle_renderer,
            collision_renderer,
            trail_renderer,
            event_sender,
            event_reader,
        ) = Self::create_systems(&mut resource_manager)?;

        let (width, height) = graphics_context.dimensions();

        let hud_refresher =
            asteroids(&mut entity_manager, &mut resource_manager, graphics_context)?;

        let ui_camera = Camera::new(
            Frustrum::orthogonal(width as f32, height as f32),
            Vec3::new(0.0, 0.0, 0.0),
        );

        graphics_context.cursor_lock(true);

        Ok(Box::new(FirstScene {
            hud_refresher,
            ui_camera,
            entity_manager,
            resource_manager,
            collision_renderer,
            trail_renderer,
            bullet_renderer,
            mesh_renderer,
            skybox_renderer,
            particle_renderer,
            text_renderer,
            sprite_renderer,
            player_controller,
            physical_simulation,
            event_sender,
            event_reader,
        }))
    }

    fn poll_events(&mut self, graphics_context: &mut GraphicsContext) {
        graphics_context.poll_events(&mut self.event_sender);
    }

    fn process_events(&mut self, graphics_context: &mut GraphicsContext) -> Option<SceneEvent> {
        self.event_reader.read().map(|events| {
            events.for_each(|event| match event {
                GameEvent::ShootPlasmaBullet(transform, bullet) => {
                    self.entity_manager.add_at(bullet, transform);
                }
                GameEvent::RemoveBullet(entity) => {
                    self.entity_manager.remove::<BulletEntity>(entity)
                }
            })
        });

        self.event_reader
            .read()?
            .fold(None, |action, event| match event {
                ContextEvent::Resized(width, height) => {
                    graphics_context.set_viewport(width, height);
                    action
                }
                ContextEvent::Close => Some(SceneEvent::Exit),
            })
    }

    fn create_systems(
        resource_manager: &mut SceneResourceManager,
    ) -> Result<
        (
            BulletRenderer,
            MeshRendererSystem,
            SkyboxRendererSystem,
            PlayerController,
            PhysicalSimulation,
            TextRendererSystem,
            SpriteRendererSystem,
            ParticleRenderer,
            CollisionRenderer,
            TrailRenderer,
            EventSender,
            EventReader,
        ),
        GameError,
    > {
        let text_shader = resource_manager.get("basic").res;
        let phong_shader = resource_manager.get("phong").res;
        let skybox_shader = resource_manager.get("basic").res;
        let particle_shader = resource_manager.get("flat").res;
        let sprite_shader = resource_manager.get("basic").res;
        let bullet_shader = resource_manager.get("plasma").res;
        let collision_shader = resource_manager.get("collision").res;
        let trail_shader = resource_manager.get("trail").res;

        let bullet_renderer = BulletRenderer::new(bullet_shader);
        let mesh_renderer = MeshRendererSystem::new(phong_shader);
        let skybox_renderer = SkyboxRendererSystem::new(skybox_shader);
        let player_controller = PlayerController::new();
        let physical_simulation = PhysicalSimulation::new(1.0 / 120.0);
        let text_system = TextRendererSystem::new(text_shader);
        let particle_renderer = ParticleRenderer::new(particle_shader);
        let sprite_renderer = SpriteRendererSystem::new(sprite_shader);
        let collision_renderer = CollisionRenderer::new(collision_shader);
        let trail_renderer = TrailRenderer::new(trail_shader);

        let (event_sender, event_reader) = create_event_queue();

        Ok((
            bullet_renderer,
            mesh_renderer,
            skybox_renderer,
            player_controller,
            physical_simulation,
            text_system,
            sprite_renderer,
            particle_renderer,
            collision_renderer,
            trail_renderer,
            event_sender,
            event_reader,
        ))
    }

    fn render(&mut self, context: &mut GraphicsContext) {
        let camera_kit = self
            .entity_manager
            .iter()
            .map(|player: &GameEntity<PlayerShip>| {
                (
                    &player.transform,
                    &player.entity.camera,
                    &player.entity.physical_body,
                )
            })
            .next();

        if let Some((camera_transform, camera, body)) = camera_kit {
            context.depth_write(false);
            self.skybox_renderer
                .render(&self.entity_manager, camera, camera_transform);
            context.depth_write(true);

            self.mesh_renderer
                .render(&self.entity_manager, camera, camera_transform);

            context.depth_write(false);
            self.particle_renderer
                .render(&self.entity_manager, camera, camera_transform);

            self.bullet_renderer
                .render_bullets(&self.entity_manager, camera, camera_transform);

            /*self.collision_renderer.render(&self.entity_manager, camera, camera_transform);

            self.trail_renderer.render(
                body,
                &self.entity_manager,
                camera,
                camera_transform,
            );*/

            context.depth_write(true);

            self.text_renderer
                .render(&self.entity_manager, &self.ui_camera);

            self.sprite_renderer
                .render(&self.entity_manager, &self.ui_camera);
        }
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
