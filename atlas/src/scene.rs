use std::rc::Rc;

use crate::{graphics::graphics_context::GraphicsContext, logger::Logger};

#[derive(Clone, Copy)]
pub enum SceneAction {
    NewScene(&'static str),
    RestartScene,
    Exit,
}

pub trait Scene {
    fn run(
        &mut self,
        logger: Rc<dyn Logger>,
        graphics_context: &mut GraphicsContext,
    ) -> SceneAction;
}

/*impl Scene {
    pub fn load(path: &str, logger: Rc<dyn Logger>) -> Result<Scene, ResourceError> {
        Ok(Scene {
            logger,
            action: SceneAction::None,
        })
    }

    pub fn run(&mut self, graphics_context: &mut GraphicsContext) -> SceneAction {
        loop {
            self.handle_user_events(graphics_context);
        }
    }

    fn handle_user_events(&mut self, graphics_context: &mut GraphicsContext) {
        graphics_context.get_events().for_each(|event| match event {
            UserEvent::Other => self.logger.log(LogMsg::Info("Unrecognized event")),
            UserEvent::Close => self.action = SceneAction::Exit,
            _ => {}
        });
    }
}*/
