use std::ptr;

use glad_gl::gl;
use glam::{Vec2, Vec3};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    graphics::{
        primitive::{MeshIndices, Primitive},
        shaders::{text_shader::TextShader, ShaderProgram},
    },
    resource_manager::font::Font,
};

use super::{camera::Camera, transform::Transform};

pub struct TextRendererSystem {
    text_quad: Primitive,
    shader_program: ShaderProgram<TextShader>,
}

impl TextRendererSystem {
    pub fn new(shader: ShaderProgram<TextShader>) -> Self {
        TextRendererSystem {
            text_quad: Primitive::new(
                &[0.; 16],
                &[2, 2],
                &mut MeshIndices::Triangles(vec![0, 1, 2, 2, 3, 0]),
            ),
            shader_program: shader,
        }
    }
}

pub struct TextRenderer {
    text: String,
    primitive: Primitive,
    font: Font,
}

impl TextRenderer {
    pub fn new(text: &str, font: Font) -> Self {
        let mut primitive = Primitive::new(&[], &[2, 2], &mut MeshIndices::Triangles(vec![]));
        font.render(text, &mut primitive);

        TextRenderer {
            text: String::from(text),
            primitive,
            font,
        }
    }

    pub fn primitive(&self) -> &Primitive {
        &self.primitive
    }
}

impl TextRendererSystem {
    pub fn render(&mut self, entity_manager: &EntityManager, camera: &Camera) {
        self.shader_program.bind();
        entity_manager.iter().for_each(
            |(transform, text_renderer): (&Transform, &TextRenderer)| unsafe {
                let projection_view_model = camera.projection_view_mat() * transform.model();
                let primitive = text_renderer.primitive();

                self.shader_program.bind();
                primitive.bind();
                text_renderer.font.bind();
                self.shader_program
                    .bind_projection_view_model(&projection_view_model.to_cols_array());

                gl::DrawElements(
                    gl::TRIANGLES,
                    primitive.count() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            },
        );
    }
}
