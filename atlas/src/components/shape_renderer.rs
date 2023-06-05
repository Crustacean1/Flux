use std::ptr;

use glad_gl::gl;

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    graphics::{
        material::{sprite_material::SpriteMaterial, Material},
        primitive::Primitive,
        shaders::{ui_shader::SpriteShader, ShaderProgram},
    },
};

use super::{camera::Camera, transform::Transform};

pub struct ShapeRenderer {
    mesh: Primitive,
    material: SpriteMaterial,
}

impl ShapeRenderer {
    pub fn quad((width, height): (f32, f32), material: SpriteMaterial) -> ShapeRenderer {
        ShapeRenderer {
            mesh: Primitive::quad(width, height),
            material,
        }
    }
}

pub struct ShapeRendererSystem {
    shader: ShaderProgram<SpriteShader>,
}

impl ShapeRendererSystem {
    pub fn new(shader: ShaderProgram<SpriteShader>) -> Self {
        ShapeRendererSystem { shader }
    }
}

impl ShapeRendererSystem {
    pub fn render(&self, entity_manager: &EntityManager, camera: &Camera) {
        unsafe {
            let vp = camera.projection_mat();
            entity_manager
                .iter()
                .for_each(|(transform, shape): (&Transform, &ShapeRenderer)| {
                    shape.material.bind();
                    shape.mesh.bind();

                    let mvp = vp * transform.model();
                    self.shader.bind_projection_view_model(&mvp.to_cols_array());

                    gl::DrawElements(
                        shape.mesh.primitive_type(),
                        shape.mesh.count() as i32,
                        gl::UNSIGNED_INT,
                        ptr::null(),
                    );
                });
        }
    }
}
