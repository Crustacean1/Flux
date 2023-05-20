use std::{collections::VecDeque, time::Instant};

use atlas::{
    components::{
        button_trigger::ButtonTriggerSystem,
        camera::{Camera, Frustrum},
        shape_renderer::ShapeRendererSystem,
    },
    entity_manager::{ComponentIterator, EntityManager},
    event_bus::{swap_event_buffers, EventReader, EventReaderTrait, EventSender, EventSenderTrait},
    game_root::GameError,
    graphics::{
        graphics_context::{ContextEvent, GraphicsContext},
        shaders::{ShaderProgram, UiShader},
    },
    resource_manager::{
        root_resource_manager::RootResourceManager, scene_resource_manager::SceneResourceManager,
        ResourceManager,
    },
    scene::{Scene, SceneEvent},
};

use crate::game_objects::menu::create_main_menu;

pub struct MainMenuScene {
    camera: Camera,
    entity_manager: EntityManager,
    resource_manager: SceneResourceManager,
    shape_rendering_system: ShapeRendererSystem,
    button_system: ButtonTriggerSystem,

    event_sender: EventSender,
    event_reader: EventReader,
}

impl Scene for MainMenuScene {
    fn run(&mut self, graphics_context: &mut GraphicsContext) -> SceneEvent {
        let (mut now, mut prev) = (Instant::now(), Instant::now());
        loop {
            prev = now;
            now = Instant::now();

            self.poll_events(graphics_context);

            graphics_context.display();

            self.button_system.check_buttons(
                self.entity_manager.iter(),
                &self.event_reader,
                &mut self.event_sender,
            );

            self.shape_rendering_system
                .render(self.entity_manager.iter(), &self.camera);

            if let Some(scene_action) = self.get_scene_action() {
                return scene_action;
            }

            swap_event_buffers(&mut self.event_reader, &mut self.event_sender)
        }
    }
}

impl MainMenuScene {
    fn poll_events(&mut self, graphics_context: &mut GraphicsContext) {
        graphics_context.poll_events(&mut self.event_sender);

        self.event_reader
            .read()
            .iter()
            .for_each(|event| match event {
                ContextEvent::Resized(width, height) => {
                    graphics_context.set_viewport(*width, *height);
                    self.camera.ortho_from_dimensions((*width as f32, *height as f32));
                }
                ContextEvent::Close => self.event_sender.send(SceneEvent::Exit),
            })
    }

    fn get_scene_action(&self) -> Option<SceneEvent> {
        self.event_reader
            .read()
            .iter()
            .fold(None, |action, event| match event {
                SceneEvent::NewScene(new_scene) => Some(SceneEvent::NewScene(new_scene)),
                SceneEvent::Exit => Some(SceneEvent::Exit),
                _ => None,
            })
    }

    pub fn new(
        root_resource_manager: &mut RootResourceManager,
        graphics_context: &mut GraphicsContext,
    ) -> Result<Box<dyn Scene>, GameError> {
        let ui_shader: ShaderProgram<UiShader> = root_resource_manager.get("basic_ui")?.res;
        let shape_rendering_system = ShapeRendererSystem::new(ui_shader);

        let mut entity_manager = EntityManager::new();
        let mut resource_manager = SceneResourceManager::build()?;

        create_main_menu(
            &mut entity_manager,
            &mut resource_manager,
            graphics_context.dimensions(),
        )?;

        let (width, height) = graphics_context.dimensions();

        let main_scene = MainMenuScene {
            camera: Camera::ortho(
                Frustrum::from_size(width as f32, height as f32),
                glam::Vec3::new(0.0, 0.0, -1.0),
                glam::Vec3::new(0.0, 0.0, 1.0),
            ),
            entity_manager,
            resource_manager,
            shape_rendering_system,
            button_system: ButtonTriggerSystem::new(),
            event_sender: EventSender::new(),
            event_reader: EventReader::new(),
        };

        Ok(Box::new(main_scene))
    }
}
