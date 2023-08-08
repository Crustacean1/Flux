use std::time::Instant;

use atlas::{
    components::{
        button_trigger::ButtonTriggerSystem,
        camera::{Camera, Frustrum},
        sprite_renderer::SpriteRendererSystem,
    },
    entity_manager::EntityManager,
    event_bus::{create_event_queue, EventReader, EventSender},
    game_root::GameError,
    graphics::{
        graphics_context::{ContextEvent, GraphicsContext},
        shaders::sprite_shader::{SpriteShader, SpriteShaderDefinition},
    },
    resource_manager::{scene_resource_manager::SceneResourceManager, ResourceManager},
    scene::{Scene, SceneEvent},
};
use glam::Vec3;

use crate::game_objects::menu::create_main_menu;

pub struct MainMenuScene {
    camera: Camera,
    entity_manager: EntityManager,
    resource_manager: SceneResourceManager,
    shape_rendering_system: SpriteRendererSystem,
    button_system: ButtonTriggerSystem,

    event_sender: EventSender,
    event_reader: EventReader,
}

impl Scene for MainMenuScene {
    fn run(&mut self, graphics_context: &mut GraphicsContext) -> SceneEvent {
        let (mut now, mut prev) = (Instant::now(), Instant::now());
        let mut context = graphics_context.new_context();

        loop {
            prev = now;
            now = Instant::now();

            self.poll_events(graphics_context);

            graphics_context.display();

            self.button_system.check_buttons(
                &mut self.entity_manager,
                &mut self.event_reader,
                &mut self.event_sender,
            );

            self.shape_rendering_system.render(
                &mut context,
                &mut self.entity_manager,
                &self.camera,
            );

            if let Some(scene_action) = self.get_scene_action() {
                return scene_action;
            }
        }
    }
}

impl MainMenuScene {
    fn poll_events(&mut self, graphics_context: &mut GraphicsContext) {
        graphics_context.poll_events(&mut self.event_sender);

        self.event_reader.read().map(|event| {
            event.for_each(|event| match event {
                ContextEvent::Resized(width, height) => {
                    graphics_context.set_viewport(width, height);
                    /*self.camera
                    .new(Frustrum::orthogonal(*width as f32, *height as f32));*/
                }
                _ => {}
            })
        });
    }

    fn get_scene_action(&mut self) -> Option<SceneEvent> {
        self.event_reader
            .read()?
            .fold(None, |action, event| match event {
                SceneEvent::NewScene(new_scene) => Some(SceneEvent::NewScene(new_scene)),
                SceneEvent::Exit => Some(SceneEvent::Exit),
                _ => None,
            })
    }

    pub fn new(graphics_context: &mut GraphicsContext) -> Result<Box<dyn Scene>, GameError> {
        let mut entity_manager = EntityManager::new();
        let mut resource_manager = SceneResourceManager::build("main")?;

        let ui_shader: SpriteShaderDefinition = resource_manager.get("basic_ui").res;
        let shape_rendering_system = SpriteRendererSystem::new(ui_shader);

        create_main_menu(
            &mut entity_manager,
            &mut resource_manager,
            graphics_context.dimensions(),
        )?;

        let (width, height) = graphics_context.dimensions();

        let (event_sender, event_reader) = create_event_queue();

        let main_scene = MainMenuScene {
            camera: Camera::new(
                Frustrum::orthogonal(width as f32, height as f32),
                Vec3::new(0.0, 0.0, 0.0),
            ),
            entity_manager,
            resource_manager,
            shape_rendering_system,
            button_system: ButtonTriggerSystem::new(),
            event_sender,
            event_reader,
        };

        Ok(Box::new(main_scene))
    }
}
