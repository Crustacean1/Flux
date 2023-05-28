use std::ptr;

use glad_gl::gl;

use crate::{
    entity_manager::{self, ComponentIteratorGenerator, EntityManager},
    graphics::{
        material::{Material, UiMaterial},
        primitive::Primitive,
        shaders::{ShaderProgram, UiShader},
    },
};

use super::{camera::Camera, transform::Transform};

pub struct ShapeRenderer {
    mesh: Primitive,
    material: UiMaterial,
}

impl ShapeRenderer {
    pub fn quad((width, height): (f32, f32), material: UiMaterial) -> ShapeRenderer {
        ShapeRenderer {
            mesh: Primitive::quad(width, height),
            material,
        }
    }
}

pub struct ShapeRendererSystem {
    shader: ShaderProgram<UiShader>,
}

impl ShapeRendererSystem {
    pub fn new(shader: ShaderProgram<UiShader>) -> Self {
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
                    let mvp = vp * transform.model();
                    self.shader.load_projection_view_model(&mvp.to_cols_array());
                    shape.material.bind(&self.shader);
                    shape.mesh.bind();

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
