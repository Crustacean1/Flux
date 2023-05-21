use atlas::{
    components::{
        camera::{Camera, Frustrum},
        controller::{CameraControllerSystem, Controller},
        skybox_renderer::{SkyboxRenderer, SkyboxSystem},
    },
    entity_manager::{ComponentIterator, EntityManager, EntityManagerTrait},
    event_bus::{swap_event_buffers, EventReader, EventReaderTrait, EventSender},
    game_root::GameError,
    graphics::{
        graphics_context::{self, ContextEvent, GraphicsContext},
        mesh::Mesh,
        vertices::base_vertices::{TriangleIndex, Vertex3PT},
    },
    resource_manager::{
        root_resource_manager::RootResourceManager, scene_resource_manager::SceneResourceManager,
        ResourceManager,
    },
    scene::{Scene, SceneEvent},
};
use glam::Vec3;

use crate::game_objects::camera_controller::UserCameraController;

pub struct FirstScene {
    cam_id: usize,
    skybox_system: SkyboxSystem,
    camera_controller_system: CameraControllerSystem,
    entity_manager: EntityManager,
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
                .read_inputs(&self.event_reader, self.entity_manager.iter());

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
    pub fn new(
        root_resource_manager: &mut RootResourceManager,
        graphics_context: &mut GraphicsContext,
    ) -> Result<Box<dyn Scene>, GameError> {
        let mut entity_manager = EntityManager::new();
        let mut resource_manager = SceneResourceManager::build()?;

        let (skybox_system, camera_controller_system, event_sender, event_reader) =
            Self::create_systems(root_resource_manager)?;

        let space_box_textures = [
            "bkg1_top",
            "bkg1_left",
            "bkg1_front",
            "bkg1_right",
            "bkg1_back",
            "bkg1_bot",
        ];

        let space_box_textures: Vec<_> = space_box_textures
            .iter()
            .map(|texture| resource_manager.get(texture).expect("").res)
            .collect();

        let space_box = SkyboxRenderer::new(10.0, &space_box_textures);
        entity_manager.add_entity(space_box);

        let (width, height) = graphics_context.dimensions();

        let camera = Camera::new_persp(
            Frustrum::centered_frustrum(width as f32, height as f32, 0.1, 100.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        );

        let cam_id = entity_manager.add_entity((
            Box::new(UserCameraController::new()) as Box<dyn Controller>,
            camera,
        ));

        graphics_context.cursor_lock(true);

        Ok(Box::new(FirstScene {
            camera_controller_system,
            skybox_system,
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
        root_resource_manager: &mut RootResourceManager,
    ) -> Result<
        (
            SkyboxSystem,
            CameraControllerSystem,
            EventSender,
            EventReader,
        ),
        GameError,
    > {
        let skybox_system = SkyboxSystem::new(root_resource_manager.get("basic_skybox")?.res);
        let camera_controller_system = CameraControllerSystem::new();
        let event_sender = EventSender::new();
        let event_reader = EventReader::new();
        Ok((
            skybox_system,
            camera_controller_system,
            event_sender,
            event_reader,
        ))
    }

    fn render(&self) {
        if let Some(camera) = self.entity_manager.get_camera(self.cam_id) {
            self.skybox_system
                .render(camera, self.entity_manager.iter());
        }
    }
}
