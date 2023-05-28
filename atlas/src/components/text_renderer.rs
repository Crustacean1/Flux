use std::ptr::{self, null};

use glad_gl::gl;
use glam::{Vec2, Vec3, Vec4};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    graphics::{
        primitive::Primitive,
        shaders::{text_shader::TextShader, ShaderProgram},
    },
    resource_manager::font::Font,
};

use super::{camera::Camera, transform::Transform};

pub struct TextRendererSystem {
    text_quad: Primitive,
    shader: ShaderProgram<TextShader>,
}

impl TextRendererSystem {
    pub fn new(shader: ShaderProgram<TextShader>) -> Self {
        TextRendererSystem {
            text_quad: Primitive::quad(10., 10.),
            shader,
        }
    }
}

pub struct TextRenderer {
    pub text: String,
    pub font: Font,
}

impl TextRendererSystem {
    pub fn render(&self, entity_manager: &EntityManager, camera: &Camera) {
        entity_manager.iter().for_each(
            |(transform, text_renderer): (&Transform, &TextRenderer)| {
                let mut position = Vec2::new(transform.position.x, transform.position.y);

                text_renderer.text.bytes().for_each(|char| unsafe {
                    let pvm = camera.projection_view_mat()
                        * glam::Mat4::from_translation(Vec3::new(position.x, position.y, 0.0));
                    self.shader.load_projection_view_model(&pvm.to_cols_array());
                    self.shader
                        .load_character(text_renderer.font.characters[char as usize].texture);
                    position.x += text_renderer.font.characters[char as usize].advance;
                    self.text_quad.bind();

                    gl::DrawElements(
                        self.text_quad.primitive_type(),
                        self.text_quad.count() as i32,
                        gl::UNSIGNED_INT,
                        ptr::null(),
                    )
                });
            },
        );
    }
}
