use atlas::{
    components::{
        camera::{Camera, Frustrum},
        controller::{CameraControllerSystem, Controller},
        mesh_renderer::MeshRendererSystem,
        skybox_renderer::SkyboxRendererSystem,
        text_renderer::TextRendererSystem,
    },
    entity_manager::{EntityManager, EntityManagerTrait},
    event_bus::{swap_event_buffers, EventReader, EventReaderTrait, EventSender},
    game_root::GameError,
    graphics::graphics_context::{ContextEvent, GraphicsContext},
    resource_manager::{scene_resource_manager::SceneResourceManager, ResourceManager},
    scene::{Scene, SceneEvent},
};
use glam::Vec3;

use crate::game_objects::{asteroids::asteroids, camera_controller::UserCameraController};

pub struct FirstScene {
    cam_id: usize,
    ui_camera: Camera,
    entity_manager: EntityManager,

    camera_controller: CameraControllerSystem,
    text_renderer: TextRendererSystem,
    mesh_renderer: MeshRendererSystem,
    skybox_renderer: SkyboxRendererSystem,
    event_reader: EventReader,
    event_sender: EventSender,
}

impl Scene for FirstScene {
    fn run(
        &mut self,
        graphics_context: &mut atlas::graphics::graphics_context::GraphicsContext,
    ) -> atlas::scene::SceneEvent {
        loop {
            self.render(graphics_context);

            self.camera_controller
                .read_inputs(&self.event_reader, &mut self.entity_manager);

            graphics_context.display();

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
            camera_controller,
            text_renderer,
            event_sender,
            event_reader,
        ) = Self::create_systems(&mut resource_manager)?;

        let (width, height) = graphics_context.dimensions();

        asteroids(&mut entity_manager, &mut resource_manager)?;

        let camera = Camera::new_persp(
            Frustrum::centered_frustrum(width as f32, height as f32, 0.1, 100.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        );

        let controller = Box::new(UserCameraController::new()) as Box<dyn Controller>;
        let cam_id = entity_manager.add_entity((controller, camera));
        let ui_camera = Camera::new_ortho(Frustrum::ui_frustrum(width as f32, height as f32));

        graphics_context.cursor_lock(true);

        Ok(Box::new(FirstScene {
            ui_camera,
            entity_manager,
            cam_id,
            mesh_renderer,
            skybox_renderer,
            text_renderer,
            camera_controller,
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
            CameraControllerSystem,
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
        let camera_controller_system = CameraControllerSystem::new();
        let text_system = TextRendererSystem::new(text_shader);

        let event_sender = EventSender::new();
        let event_reader = EventReader::new();

        Ok((
            mesh_renderer,
            skybox_renderer,
            camera_controller_system,
            text_system,
            event_sender,
            event_reader,
        ))
    }

    fn render(&mut self, context: &mut GraphicsContext) {
        if let Some(camera) = self.entity_manager.get_camera(self.cam_id) {
            context.depth_write(false);
            self.skybox_renderer.render(camera, &self.entity_manager);
            context.depth_write(true);
            self.mesh_renderer.render(&self.entity_manager, camera);
            self.text_renderer
                .render(&self.entity_manager, &self.ui_camera);
        }
    }
}
