use glad_gl::gl;

use super::shaders::Shader;

pub trait GenericMesh {
    fn vao(&self) -> u32;
}

pub struct Context {}

impl Context {
    pub fn shader_pass<S: Shader>(&mut self, shader: &S) -> S::ShaderPass {
        unsafe {
            gl::UseProgram(shader.shader_id());
            S::new_pass(shader)
        }
    }
}
