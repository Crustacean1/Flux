use std::collections::VecDeque;

use atlas::{
    components::{
        camera::{Camera, Frustrum},
        shape_renderer::ShapeRendererSystem,
    },
    game_root::GameError,
    graphics::{
        graphics_context::{GraphicsContext, UserEvent},
        shaders::{ShaderProgram, UiShader},
    },
    logger::Logger,
    resource_manager::{
        root_resource_manager::RootResourceManager, scene_resource_manager::SceneResourceManager,
        ResourceManager,
    },
    scene::{Scene, SceneAction},
};

use crate::component_manager::{ComponentAggregator, ComponentManager};

use self::menu::main_menu;

mod menu;

enum SceneEvent {
    NewScene(&'static str),
    Exit,
}

pub struct MainMenuScene {
    scene_events: VecDeque<SceneEvent>,
    camera: Camera,
    component_manager: ComponentAggregator,
    resource_manager: SceneResourceManager,
    shape_rendering_system: ShapeRendererSystem,
}

impl Scene for MainMenuScene {
    fn run(
        &mut self,
        logger: std::rc::Rc<dyn Logger>,
        graphics_context: &mut GraphicsContext,
    ) -> SceneAction {
        loop {
            self.handle_user_events(graphics_context);

            graphics_context.display();

            let shapes = self.component_manager.components_mut();
            self.shape_rendering_system.render(shapes, &self.camera);

            if let Some(scene_action) = self.handle_scene_events() {
                return scene_action;
            }
        }
    }
}

impl MainMenuScene {
    fn handle_user_events(&mut self, graphics_context: &mut GraphicsContext) {
        graphics_context.get_events().for_each(|event| match event {
            UserEvent::Close => self.scene_events.push_back(SceneEvent::Exit),
            _ => {}
        });
    }

    fn handle_scene_events(&mut self) -> Option<SceneAction> {
        while let Some(event) = self.scene_events.pop_front() {
            match event {
                SceneEvent::NewScene(scene_id) => return Some(SceneAction::NewScene(scene_id)),
                SceneEvent::Exit => return Some(SceneAction::Exit),
            }
        }
        None
    }

    pub fn new(
        root_resource_manager: &mut RootResourceManager,
        graphics_context: &GraphicsContext,
    ) -> Result<Box<dyn Scene>, GameError> {
        let ui_shader: ShaderProgram<UiShader> = root_resource_manager.get("basic_ui")?.res;
        let shape_rendering_system = ShapeRendererSystem::new(ui_shader);

        let mut component_manager = ComponentAggregator::new();
        let mut resource_manager = SceneResourceManager::build(graphics_context)?;

        main_menu(&mut component_manager, &mut resource_manager)?;

        let (width, height) = graphics_context.dimensions();
        let (width, height) = (width as f32 / 2.0, height as f32 / 2.0);

        let main_scene = MainMenuScene {
            resource_manager,
            scene_events: VecDeque::new(),
            camera: Camera::new(
                Frustrum::new(-width, width, -height, height, 1.0, 10.0),
                glam::Vec3::new(0.0, 0.0, -1.0),
                glam::Vec3::new(0.0, 0.0, 1.0),
            ),
            component_manager,
            shape_rendering_system,
        };

        Ok(Box::new(main_scene))
    }
}
