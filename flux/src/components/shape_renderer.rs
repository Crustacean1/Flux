use std::ptr;

use glad_gl::gl;

use atlas::graphics::{
    material::TextureMaterial,
    mesh::Mesh,
    vertices::base_vertices::{TriangleIndex, Vertex2P},
};

pub struct ShapeRenderer {
    mesh: Mesh<Vertex2P, TriangleIndex>,
    material: TextureMaterial,
}

impl ShapeRenderer {
    pub fn quad() -> ShapeRenderer {
        ShapeRenderer {
            mesh: Mesh::gen_quad(5.0, 5.0),
            material: TextureMaterial::from_color(0.0, 1.0, 0.0),
        }
    }
}

pub struct ShapeRendererSystem {
    //shader: Shader<Vertex2P, TextureMaterial>,
}

impl ShapeRendererSystem {
    pub fn render(shapes: &[ShapeRenderer]) {
        unsafe {
            shapes.iter().for_each(|shape| {
                shape.mesh.bind();
                gl::DrawElements(
                    shape.mesh.primitive_type(),
                    shape.mesh.count(),
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            });
        }
    }
}
