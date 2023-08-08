use std::mem;

use glad_gl::gl;

use super::{
    material::Material,
    shaders::{Shader, ShaderDefinition},
};

pub trait GenericMesh {
    fn vao(&self) -> u32;
}

pub struct Context {}

pub struct ShaderContext<'a, T: Shader> {
    pub context: &'a mut Context,
    pub shader: &'a T,
}

pub struct MaterialContext<'a, T: Shader, Q: Material> {
    pub context: &'a mut Context,
    pub shader: &'a T,
    pub material: &'a Q,
}

impl Context {
    pub fn use_shader<'a, T: ShaderDefinition + 'a>(
        &'a mut self,
        shader_def: &'a T,
        shader_fn: impl FnOnce(&mut ShaderContext<T::Shader>),
    ) {
        unsafe {
            let shader = shader_def.create_shader();
            gl::UseProgram(shader.shader_id());
            let mut shader_context = ShaderContext {
                context: self,
                shader: &shader,
            };
            shader_fn(&mut shader_context);
        }
    }
}

impl<'a, T: Shader> ShaderContext<'a, T> {
    pub fn use_material<Q: Material + 'a>(
        &self,
        material: &'a Q,
        material_fn: impl FnOnce(&mut MaterialContext<T, Q>),
    ) {
        let oneself = to_mut(self); // TODO: Fix it

        material.bind();
        let mut material_context = MaterialContext {
            context: oneself.context,
            shader: oneself.shader,
            material,
        };
        material_fn(&mut material_context);
    }
}

fn to_mut<T>(val: &T) -> &mut T {
    unsafe {
        let ptr: *const T = val;
        mem::transmute(ptr)
    }
}
