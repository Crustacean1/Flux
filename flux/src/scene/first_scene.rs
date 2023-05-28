use atlas::{
    components::{
        camera::{Camera, Frustrum},
        controller::{CameraControllerSystem, Controller},
        mesh_renderer::MeshRendererSystem,
        skybox_renderer::SkyboxSystem,
        text_renderer::TextRendererSystem,
    },
    entity_manager::{ComponentIteratorGenerator, EntityManager, EntityManagerTrait},
    event_bus::{swap_event_buffers, EventReader, EventReaderTrait, EventSender},
    game_root::GameError,
    graphics::{
        graphics_context::{ContextEvent, GraphicsContext},
        shaders::{text_shader::TextShader, MeshShader, ShaderProgram, SkyboxShader},
    },
    resource_manager::{scene_resource_manager::SceneResourceManager, ResourceManager},
    scene::{Scene, SceneEvent},
};
use glam::Vec3;

use crate::game_objects::{
    asteroids::asteroids, camera_controller::UserCameraController, skybox::skybox,
};

pub struct FirstScene {
    cam_id: usize,
    skybox_system: SkyboxSystem,
    mesh_system: MeshRendererSystem,
    camera_controller_system: CameraControllerSystem,
    ui_camera: Camera,
    entity_manager: EntityManager,
    text_system: TextRendererSystem,
    event_reader: EventReader,
    event_sender: EventSender,
}

impl Scene for FirstScene {
    fn run(
        &mut self,
        graphics_context: &mut atlas::graphics::graphics_context::GraphicsContext,
    ) -> atlas::scene::SceneEvent {
        loop {
            self.render();

            self.camera_controller_system
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
            skybox_system,
            mesh_system,
            camera_controller_system,
            text_system,
            event_sender,
            event_reader,
        ) = Self::create_systems(&mut resource_manager)?;

        let (width, height) = graphics_context.dimensions();

        skybox(&mut entity_manager, &mut resource_manager);
        asteroids(&mut entity_manager, &mut resource_manager)?;

        let camera = Camera::new_persp(
            Frustrum::centered_frustrum(width as f32, height as f32, 0.1, 100.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        );

        let camera_controller = Box::new(UserCameraController::new()) as Box<dyn Controller>;
        let cam_id = entity_manager.add_entity((camera_controller, camera));
        let ui_camera = Camera::new_ortho(Frustrum::ui_frustrum(width as f32, height as f32));

        graphics_context.cursor_lock(true);

        Ok(Box::new(FirstScene {
            camera_controller_system,
            ui_camera,
            skybox_system,
            mesh_system,
            text_system,
            event_reader,
            event_sender,
            entity_manager,
            cam_id,
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
            SkyboxSystem,
            MeshRendererSystem,
            CameraControllerSystem,
            TextRendererSystem,
            EventSender,
            EventReader,
        ),
        GameError,
    > {
        let sky_shader: ShaderProgram<SkyboxShader> = resource_manager.get("basic").res;
        let mesh_shader: ShaderProgram<MeshShader> = resource_manager.get("basic").res;
        let text_shader: ShaderProgram<TextShader> = resource_manager.get("basic").res;

        let skybox_system = SkyboxSystem::new(sky_shader.clone());
        let mesh_system = MeshRendererSystem::new(mesh_shader);

        let camera_controller_system = CameraControllerSystem::new();
        let text_system = TextRendererSystem::new(text_shader);

        let event_sender = EventSender::new();
        let event_reader = EventReader::new();

        Ok((
            skybox_system,
            mesh_system,
            camera_controller_system,
            text_system,
            event_sender,
            event_reader,
        ))
    }

    fn render(&mut self) {
        if let Some(camera) = self.entity_manager.get_camera(self.cam_id) {
            //self.skybox_system
            //.render(camera, self.entity_manager.iter());
            self.mesh_system.render(&self.entity_manager, camera);
            self.text_system
                .render(&self.entity_manager, &self.ui_camera)
        }
    }
}
