use std::collections::VecDeque;

use atlas::{
    graphics::graphics_context::{GraphicsContext, UserEvent},
    logger::Logger,
    scene::{Scene, SceneAction},
};

use self::menu::Menu;

mod menu;

enum SceneEvent {
    NewScene(&'static str),
    Exit,
}

pub struct MainMenuScene {
    scene_events: VecDeque<SceneEvent>,
    // Components
    menus: Vec<Menu>,
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
        Box::new(MainMenuScene {
            scene_events: VecDeque::new(),
            menus: Self::create_menus(),
        })
    }
}

impl MainMenuScene {
    fn create_menus() -> Vec<Menu> {
        vec![]
    }
}
