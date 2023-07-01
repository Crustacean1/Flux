pub trait Renderer {
    type Shader;
    fn render(&self, shader: &Self::Shader);
}

pub trait RenderPass {
    fn pre_rendering();
    fn run();
}
