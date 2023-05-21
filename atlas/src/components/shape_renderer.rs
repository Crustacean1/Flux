use std::ptr;

use glad_gl::gl;

use crate::graphics::{
    material::TextureMaterial,
    mesh::Mesh,
    shaders::{ShaderProgram, UiShader},
    vertices::base_vertices::{TriangleIndex, Vertex2PT},
};

use super::{camera::Camera, transform::Transform};

pub struct ShapeRenderer {
    mesh: Mesh<Vertex2PT, TriangleIndex>,
    material: TextureMaterial,
}

impl ShapeRenderer {
    pub fn quad((width, height): (f32, f32), material: TextureMaterial) -> ShapeRenderer {
        ShapeRenderer {
            mesh: Mesh::gen_quad(width, height),
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

pub type ShapeAndTransform<'a> = (&'a Transform, &'a ShapeRenderer);

impl ShapeRendererSystem {
    pub fn render<'a>(
        &self,
        shapes: &[(usize, *const Transform, *const ShapeRenderer)],
        camera: &Camera,
    ) {
        unsafe {
            let vp = camera.pv_mat();
            shapes.iter().for_each(|(_, transform, shape)| {
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
