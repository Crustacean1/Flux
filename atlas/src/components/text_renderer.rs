use std::ptr::{self};

use glad_gl::gl;
use glam::{Mat4, Vec2, Vec3};

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
    shader: ShaderProgram<TextShader>,
}

impl TextRendererSystem {
    pub fn new(shader: ShaderProgram<TextShader>) -> Self {
        TextRendererSystem {
            text_quad: Primitive::new(
                &[0.; 16],
                &[2, 2],
                &mut MeshIndices::Triangles(vec![0, 1, 2, 2, 3, 0]),
            ),
            shader,
        }
    }
}

pub struct TextRenderer {
    pub text: String,
    pub font: Font,
}

impl TextRendererSystem {
    pub fn render(&mut self, entity_manager: &EntityManager, camera: &Camera) {
        entity_manager.iter().for_each(
            |(transform, text_renderer): (&Transform, &TextRenderer)| {
                let mut position = Vec2::new(transform.position.x, transform.position.y);

                text_renderer.text.bytes().for_each(|char| unsafe {
                    let character = text_renderer.font.characters[char as usize];
                    let (width, height) = (character.size.x, character.size.y);
                    let (x_bearing, y_bearing) = (character.bearing.x, character.bearing.y);

                    self.text_quad
                        .reload(&[
                            x_bearing,
                            -y_bearing,
                            0.,
                            0.,
                            x_bearing + width,
                            -y_bearing,
                            1.,
                            0.,
                            x_bearing + width,
                            height - y_bearing,
                            1.,
                            1.,
                            x_bearing,
                            height - y_bearing,
                            0.,
                            1.,
                        ])
                        .unwrap();

                    let pvm = camera.projection_view_mat()
                        * transform.model()
                        * glam::Mat4::from_translation(Vec3::new(position.x, position.y, 0.0));

                    self.shader.load_projection_view_model(&pvm.to_cols_array());
                    character.bind(&self.shader);

                    position.x += (character.advance >> 6) as f32;
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
