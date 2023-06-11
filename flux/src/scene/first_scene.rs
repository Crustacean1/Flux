use std::time::Instant;

use atlas::{
    components::{
        camera::{Camera, Frustrum},
        mesh_renderer::MeshRendererSystem,
        physical_body::PhysicalBody,
        skybox_renderer::SkyboxRendererSystem,
        text_renderer::TextRendererSystem,
        transform::Transform,
    },
    entity_manager::EntityManager,
    event_bus::{swap_event_buffers, EventReader, EventReaderTrait, EventSender},
    game_entities::{player_ship::PlayerShip, GameEntity},
    game_root::GameError,
    graphics::graphics_context::{ContextEvent, GraphicsContext},
    resource_manager::{scene_resource_manager::SceneResourceManager, ResourceManager},
    scene::{Scene, SceneEvent},
    systems::{physical_simulation::PhysicalSimulation, player_controller::PlayerController},
};
use glam::{Quat, Vec3};

use crate::game_objects::asteroids::asteroids;

pub struct FirstScene {
    ui_camera: Camera,
    entity_manager: EntityManager,

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
    ) -> atlas::scene::SceneEvent {
        let (mut now, mut prev) = (Instant::now(), Instant::now());
        loop {
            now = Instant::now();

            self.render(graphics_context);

            self.player_controller
                .control(&mut self.entity_manager, &self.event_reader);
            graphics_context.display();

            if (now - prev).as_nanos() > self.physical_simulation.delta() {
                self.physical_simulation
                    .integrate_movement(&mut self.entity_manager);
                prev = now;
            }

            self.poll_events(graphics_context);
            swap_event_buffers(&mut self.event_reader, &mut self.event_sender);

            if let Some(action) = self.process_events(graphics_context) {
                return action;
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
            event_sender,
            event_reader,
        ) = Self::create_systems(&mut resource_manager)?;

        let (width, height) = graphics_context.dimensions();

        asteroids(&mut entity_manager, &mut resource_manager)?;

        let camera = Camera::new(
            Frustrum::perspective(width as f32, height as f32, 0.1, 100.0),
            Vec3::new(0.0, 1.0, 3.0),
        );

        entity_manager.add_at(
            PlayerShip {
                camera,
                physical_body: PhysicalBody::new(10., 10.),
                mesh: resource_manager.get("spaceship3").res,
            },
            Transform {
                position: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                rotation: Quat::IDENTITY,
            },
        );

        let ui_camera = Camera::new(
            Frustrum::orthogonal(width as f32, height as f32),
            Vec3::new(0.0, 0.0, 0.0),
        );

        graphics_context.cursor_lock(true);

        Ok(Box::new(FirstScene {
            ui_camera,
            entity_manager,
            mesh_renderer,
            skybox_renderer,
            text_renderer,
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
        self.event_reader
            .read()
            .iter()
            .fold(None, |action, event| match event {
                ContextEvent::Resized(width, height) => {
                    graphics_context.set_viewport(*width, *height);
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
            EventSender,
            EventReader,
        ),
        GameError,
    > {
        let text_shader = resource_manager.get("basic").res;
        let phong_shader = resource_manager.get("phong").res;
        let skybox_shader = resource_manager.get("basic").res;

        let mesh_renderer = MeshRendererSystem::new(phong_shader);
        let skybox_renderer = SkyboxRendererSystem::new(skybox_shader);
        let player_controller = PlayerController::new();
        let physical_simulation = PhysicalSimulation::new(1.0 / 120.0);
        let text_system = TextRendererSystem::new(text_shader);

        let event_sender = EventSender::new();
        let event_reader = EventReader::new();

        Ok((
            mesh_renderer,
            skybox_renderer,
            player_controller,
            physical_simulation,
            text_system,
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
}
