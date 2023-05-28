use std::ptr;

use glad_gl::gl::{self, PrimitiveRestartIndex};

use super::{
    material::{Material, TextureMaterial},
    primitive::Primitive,
    shaders::ShaderProgram,
};

#[derive(Clone)]
pub struct Mesh<Q: Clone, T: Material<Shader = Q>> {
    pub primitives: Vec<(T, Primitive)>,
}

impl<Q: Clone, T: Material<Shader = Q>> Mesh<Q, T> {
    pub fn render(&self, shader: &ShaderProgram<Q>) {
        self.primitives
            .iter()
            .for_each(|(material, primitive)| unsafe {
                primitive.bind();
                //material.bind(&shader);
                gl::DrawElements(
                    primitive.primitive_type(),
                    primitive.count() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            });
    }
}

impl<Q: Clone, T: Material<Shader = Q> + Default> Default for Mesh<Q, T> {
    fn default() -> Self {
        let primitive = Primitive::sphere(1.0, 10);
        let mat = T::default();
        return Mesh {
            primitives: vec![(mat, primitive)],
        };
    }
}
