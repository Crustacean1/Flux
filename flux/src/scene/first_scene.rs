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
        particle_system::update_particles,
        physical_simulation::PhysicalSimulation,
        player_controller::PlayerController,
        text_update::{update_text, TextChangeEvent},
    },
};
use glam::Vec3;

use crate::game_objects::asteroids::asteroids;

pub enum GameEvent {
    ShootPlasmaBullet(BulletEntity),
    RemoveEntity(usize),
}

pub struct FirstScene {
    ui_camera: Camera,
    entity_manager: EntityManager,
    resource_manager: SceneResourceManager,

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
            prev = now;
            now = Instant::now();

            physics_delta += (now - prev).as_nanos();

            fps = fps * 0.9 + 0.1 * (1_000_000_000.0 / (now - prev).as_nanos() as f32);
            self.event_sender.write(TextChangeEvent::TextChange(
                fps_counter,
                format!("FPS: {}", fps),
            ));

            self.event_sender.write(TextChangeEvent::TextChange(
                physics_counter,
                format!("Physix: {}", physics_fps),
            ));

            self.render(graphics_context);

            update_particles(&mut self.entity_manager, (now - prev).as_nanos());
            update_text(&mut self.entity_manager, &mut self.event_reader);

            self.player_controller.control(
                &mut self.entity_manager,
                &mut self.event_reader,
                &mut self.event_sender,
            );
            graphics_context.display();

            if physics_delta > self.physical_simulation.delta() {
                physics_fps = physics_fps * 0.9
                    + 0.1 * (1_000_000_000.0 / prev_phys.elapsed().as_nanos() as f32);
                self.physical_simulation
                    .integrate_movement(&mut self.entity_manager);
                physics_delta -= self.physical_simulation.delta();
                prev_phys = Instant::now();
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
            mesh_renderer,
            skybox_renderer,
            player_controller,
            physical_simulation,
            text_renderer,
            sprite_renderer,
            particle_renderer,
            event_sender,
            event_reader,
        ) = Self::create_systems(&mut resource_manager)?;

        let (width, height) = graphics_context.dimensions();

        asteroids(&mut entity_manager, &mut resource_manager, graphics_context)?;

        let ui_camera = Camera::new(
            Frustrum::orthogonal(width as f32, height as f32),
            Vec3::new(0.0, 0.0, 0.0),
        );

        graphics_context.cursor_lock(true);

        Ok(Box::new(FirstScene {
            ui_camera,
            entity_manager,
            resource_manager,
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
                GameEvent::ShootPlasmaBullet(bullet) => {
                    self.entity_manager.add(bullet);
                }
                GameEvent::RemoveEntity(_) => {
                    //self.entity_manager.
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
            MeshRendererSystem,
            SkyboxRendererSystem,
            PlayerController,
            PhysicalSimulation,
            TextRendererSystem,
            SpriteRendererSystem,
            ParticleRenderer,
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

        let mesh_renderer = MeshRendererSystem::new(phong_shader);
        let skybox_renderer = SkyboxRendererSystem::new(skybox_shader);
        let player_controller = PlayerController::new();
        let physical_simulation = PhysicalSimulation::new(1.0 / 120.0);
        let text_system = TextRendererSystem::new(text_shader);
        let particle_renderer = ParticleRenderer::new(particle_shader);
        let sprite_renderer = SpriteRendererSystem::new(sprite_shader);

        let (event_sender, event_reader) = create_event_queue();

        Ok((
            mesh_renderer,
            skybox_renderer,
            player_controller,
            physical_simulation,
            text_system,
            sprite_renderer,
            particle_renderer,
            event_sender,
            event_reader,
        ))
    }

    fn render(&mut self, context: &mut GraphicsContext) {
        if let Some((camera_transform, camera)) = self.get_player_camera() {
            context.depth_write(false);
            self.skybox_renderer
                .render(&self.entity_manager, camera, camera_transform);
            context.depth_write(true);

            self.mesh_renderer
                .render(&self.entity_manager, camera, camera_transform);

            context.depth_write(false);
            self.particle_renderer
                .render(&self.entity_manager, camera, camera_transform);
            context.depth_write(true);

            self.sprite_renderer
                .render(&self.entity_manager, &self.ui_camera);

            self.text_renderer
                .render(&self.entity_manager, &self.ui_camera);
        }
    }

    fn get_player_camera(&self) -> Option<(&Transform, &Camera)> {
        self.entity_manager
            .iter()
            .map(|player: &GameEntity<PlayerShip>| (&player.transform, &player.entity.camera))
            .next()
    }

    fn create_label(&mut self, position: Vec3) -> usize {
        let font: Font = self.resource_manager.get("main").res;
        self.entity_manager.add_at(
            UiLabel {
                renderer: TextRenderer::new("", font),
            },
            Transform::pos(position),
        )
    }
}
