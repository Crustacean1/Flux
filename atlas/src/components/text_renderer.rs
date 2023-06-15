use glad_gl::gl;
use std::ptr;

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::ui_label::UiLabel,
    graphics::{
        primitive::Primitive,
        shaders::{text_shader::TextShader, ShaderProgram},
        vertices::layouts::{P2TVertex, PTVertex, TriangleGeometry},
    },
    resource_manager::font::Font,
};

use super::{camera::Camera, transform::Transform};

pub struct TextRendererSystem {
    shader_program: ShaderProgram<TextShader>,
}

impl TextRendererSystem {
    pub fn new(shader: ShaderProgram<TextShader>) -> Self {
        TextRendererSystem {
            shader_program: shader,
        }
    }
}

pub struct TextRenderer {
    text: String,
    primitive: Primitive<P2TVertex, TriangleGeometry>,
    font: Font,
}

impl TextRenderer {
    pub fn new(text: &str, font: Font) -> Self {
        let mut primitive = Primitive::triangles(&vec![], &vec![]);
        font.render(text, &mut primitive);

        TextRenderer {
            text: String::from(text),
            primitive,
            font,
        }
    }

    pub fn primitive(&self) -> &Primitive<P2TVertex, TriangleGeometry> {
        &self.primitive
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a TextRenderer)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a TextRenderer)> + 'a> {
        let labels = self
            .iter::<UiLabel>()
            .map(|label| (&label.transform, &label.entity.renderer));

        Box::new(labels)
    }
}

impl TextRendererSystem {
    pub fn render(&mut self, entity_manager: &EntityManager, camera: &Camera) {
        self.shader_program.bind();
        entity_manager.get_view().for_each(
            |(transform, text_renderer): (&Transform, &TextRenderer)| unsafe {
                let projection_view_model = camera.projection() * transform.model();
                let primitive = text_renderer.primitive();

                primitive.bind();
                text_renderer.font.bind();
                self.shader_program
                    .bind_projection_view_model(&projection_view_model.to_cols_array());

                gl::DrawElements(
                    primitive.primitive_type(),
                    primitive.count() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            },
        );
    }
}
