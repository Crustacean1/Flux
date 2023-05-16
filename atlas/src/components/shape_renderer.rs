use std::ptr;

use glad_gl::gl;

use crate::graphics::{
    material::TextureMaterial,
    mesh::Mesh,
    shaders::{ShaderProgram, UiShader},
    vertices::base_vertices::{TriangleIndex, Vertex2PT},
};

use super::{camera::Camera, transform::Transform, Component};

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

pub type ShapeAndTransform<'a> = (&'a Component<ShapeRenderer>, &'a Component<Transform>);

impl ShapeRendererSystem {
    pub fn render<'a>(&self, shapes: &[ShapeAndTransform], camera: &Camera) {
        unsafe {
            self.shader.load_mvp(&camera.vp_mat());
            shapes.iter().for_each(
                |(
                    Component::<ShapeRenderer> {
                        component: shape, ..
                    },
                    Component::<Transform> {
                        component: transform,
                        ..
                    },
                )| {
                    shape.mesh.bind();
                    self.shader.bind_material(&shape.material);

                    gl::DrawElements(
                        shape.mesh.primitive_type(),
                        shape.mesh.count() as i32,
                        gl::UNSIGNED_INT,
                        ptr::null(),
                    );
                },
            );
        }
    }
}
