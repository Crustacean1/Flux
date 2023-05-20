use crate::graphics::graphics_context::GraphicsContext;

#[derive(Clone, Copy)]
pub enum SceneEvent {
    NewScene(&'static str),
    RestartScene,
    Exit,
}

pub trait Scene {
    fn run(&mut self, graphics_context: &mut GraphicsContext) -> SceneEvent;
}
