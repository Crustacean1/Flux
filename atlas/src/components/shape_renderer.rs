use std::ptr;

use glad_gl::gl;

use crate::graphics::{
    material::TextureMaterial,
    mesh::Primitive,
    shaders::{ShaderProgram, UiShader},
    vertices::base_vertices::{TriangleIndex, Vertex2PT},
};

use super::{camera::Camera, transform::Transform};

pub struct ShapeRenderer {
    mesh: Primitive<Vertex2PT, TriangleIndex>,
    material: TextureMaterial,
}

impl ShapeRenderer {
    pub fn quad((width, height): (f32, f32), material: TextureMaterial) -> ShapeRenderer {
        ShapeRenderer {
            mesh: Primitive::gen_quad(width, height),
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
    pub fn render(
        &self,
        shapes: &[((usize, *const Transform), *const ShapeRenderer)],
        camera: &Camera,
    ) {
        unsafe {
            let vp = camera.projection_view_mat();
            shapes.iter().for_each(|((_, transform), shape)| {
                let transform = &**transform;
                let shape = &**shape;

                let mvp = vp * transform.model();
                self.shader.load_mvp(&mvp.to_cols_array());
                shape.material.bind();
                shape.mesh.bind();
                self.shader.bind_material(&shape.material);

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
