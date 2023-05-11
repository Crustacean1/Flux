use std::collections::VecDeque;

use atlas::{
    graphics::graphics_context::{GraphicsContext, UserEvent},
    logger::Logger,
    scene::{Scene, SceneAction, Stage},
};

use crate::components::{
    shape_renderer::{ShapeRenderer, ShapeRendererSystem},
    ConcreteComponentAllocator,
};

mod menu;

enum SceneEvent {
    NewScene(&'static str),
    Exit,
}

pub struct MainMenuScene {
    scene_events: VecDeque<SceneEvent>,
    component_allocator: ConcreteComponentAllocator,
    shape_renderer_system: ShapeRendererSystem,
    stage: Stage,
    // Components
    //shapes: Allocator<ShapeRenderer>
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

            self.shape_renderer_system.render();

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

    pub fn new() -> Box<dyn Scene> {
        let mut main_scene = MainMenuScene {
            scene_events: VecDeque::new(),
            component_allocator: ConcreteComponentAllocator::new(),
            shape_renderer_system: ShapeRendererSystem {},
            stage: Stage::new(),
        };

        let menu = main_scene.stage.add_entity();

        menu.add_component::<ConcreteComponentAllocator, ShapeRenderer>(
            &mut main_scene.component_allocator,
            ShapeRenderer::quad(),
        );

        Box::new(main_scene)
    }
}
