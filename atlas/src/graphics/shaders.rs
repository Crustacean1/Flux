pub mod mesh_shader;
pub mod skybox_shader;
pub mod text_shader;
pub mod ui_shader;

use glad_gl::gl;

use crate::game_root::GameError;
use std::{ffi::c_char, marker::PhantomData, mem, path::PathBuf};

use super::lights::LightColor;

pub trait Shader<T: Clone> {
    fn build(shader_id: u32) -> Result<T, GameError>;
}

impl From<(ShaderType, String)> for GameError {
    fn from((shader_type, error): (ShaderType, String)) -> Self {
        GameError::new(&format!(
            "Shader '{:?}' compilation failed:\n{}",
            shader_type, error
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
}

impl ShaderType {
    pub fn to_gl(&self) -> u32 {
        match self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
        }
    }
}

#[derive(Clone)]
pub struct ShaderProgram<T: Clone> {
    shader_id: u32,
    pub shader: T,
}

impl<T: Clone + Shader<T>> ShaderProgram<T> {
    pub fn build(shader_id: u32) -> Result<Self, GameError> {
        let shader = Self {
            shader_id,
            shader: T::build(shader_id)?,
        };
        Ok(shader)
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.shader_id);
        }
    }

    fn load_mat(&self, mat: &[f32; 16], location: i32) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
        }
    }

    fn get_location(shader_id: u32, name: &str) -> Result<i32, GameError> {
        unsafe {
            match gl::GetUniformLocation(shader_id, mem::transmute(name.as_ptr())) {
                -1 => GameError::err(format!(
                    "Shader: '{}' uniform '{}' not found\n",
                    shader_id, name
                )),
                location => Ok(location),
            }
        }
    }
}


impl<T: Clone> Default for ShaderProgram<T> {
    fn default() -> Self {
        todo!()
    }
}
