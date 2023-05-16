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
