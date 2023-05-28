use crate::{
    entity_manager::ComponentIterator, graphics::primitive::Primitive, resource_manager::font::Font,
};

use super::transform::Transform;

pub struct TextRendererSystem {
    text_quad: Primitive,
    //shader: TextShader,
}

pub struct TextRenderer {
    text: String,
    font: Font,
}

impl TextRendererSystem {
    pub fn render(renderers: ComponentIterator<(&Transform, &TextRenderer)>) {}
}
