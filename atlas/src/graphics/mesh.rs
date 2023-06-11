use std::ptr;

use glad_gl::gl;

use super::{
    material::Material,
    primitive::Primitive,
    shaders::ShaderProgram,
    vertices::layouts::{BufferElement, IndexGeometry},
};

#[derive(Clone)]
pub struct Mesh<Vertex: BufferElement, Geometry: IndexGeometry, Material> {
    pub primitives: Vec<(Material, Primitive<Vertex, Geometry>)>,
}

impl<Vertex: BufferElement, Geometry: IndexGeometry, Material> Mesh<Vertex, Geometry, Material> {
    pub fn render<Q: Clone>(&self, shader: &ShaderProgram<Q>) {
        self.primitives
            .iter()
            .for_each(|(material, primitive)| unsafe {
                primitive.bind();
                gl::DrawElements(
                    primitive.primitive_type(),
                    primitive.count() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            });
    }
}

impl<Vertex: BufferElement, Geometry: IndexGeometry, Material: Default> Default
    for Mesh<Vertex, Geometry, Material>
{
    fn default() -> Self {
        let primitive = Primitive::sphere(1.0, 5);
        let mat = Material::default();
        return Mesh {
            primitives: vec![(mat, primitive)],
        };
    }
}
