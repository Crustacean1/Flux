use std::ptr;

use glad_gl::gl;

use crate::graphics::{
    material::TextureMaterial,
    mesh::Mesh,
    shaders::{ShaderProgram, UiShader},
    vertices::base_vertices::{TriangleIndex, Vertex2PT},
};

use super::{camera::Camera, Component};

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

impl ShapeRendererSystem {
    pub fn render<'a>(
        &self,
        shapes: impl Iterator<Item = &'a mut Component<ShapeRenderer>>,
        camera: &Camera,
    ) {
        unsafe {
            self.shader.load_mvp(&camera.vp_mat());
            shapes.for_each(|Component::<ShapeRenderer> { component, .. }| {
                component.mesh.bind();
                self.shader.bind_material(&component.material);

                gl::DrawElements(
                    component.mesh.primitive_type(),
                    component.mesh.count() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            });
        }
    }
}
