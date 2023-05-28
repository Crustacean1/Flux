use glad_gl::gl;
use glam::Vec3;

use crate::game_root::GameError;
use std::{marker::PhantomData, mem, path::PathBuf};

use super::lights::LightColor;

pub trait Shader<T: Clone> {
    fn new(shader_id: u32) -> T;
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
    shader: T,
}

impl<T: Clone + Shader<T>> ShaderProgram<T> {
    pub fn new(shader_id: u32) -> Self {
        let mut shader = Self {
            shader_id,
            shader: T::new(shader_id),
        };
        shader
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.shader_id);
        }
    }

    pub fn load_projection_view_model(&self, mat: &[f32; 16]) {
        self.load_mat(mat, "projection_view_model\0");
    }

    pub fn load_view_model(&self, mat: &[f32; 16]) {
        self.load_mat(mat, "view_model\0");
    }

    pub fn load_view(&self, mat: &[f32; 16]) {
        self.load_mat(mat, "view\0");
    }

    fn load_mat(&self, mat: &[f32; 16], name: &str) {
        unsafe {
            gl::UseProgram(self.shader_id);
            let mvp = gl::GetUniformLocation(self.shader_id, mem::transmute(name.as_ptr()));
            gl::UniformMatrix4fv(mvp, 1, gl::FALSE, mat.as_ptr());
        }
    }
}

#[derive(Clone)]
pub struct UiShader;

impl Shader<UiShader> for UiShader {
    fn new(shader_id: u32) -> UiShader {
        Self {}
    }
}

impl ShaderProgram<UiShader> {
    pub fn bind_texture(&self, texture_id: u32) {
        unsafe {
            gl::UseProgram(self.shader_id);
            let mat_texture =
                gl::GetUniformLocation(self.shader_id, mem::transmute("mat_texture\0".as_ptr()));
            match mat_texture {
                -1 => {
                    println!("Failed to load uniform: 'mat_texture'");
                }
                _ => {
                    gl::Uniform1i(mat_texture, texture_id as i32);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct SkyboxShader;

impl Shader<SkyboxShader> for SkyboxShader {
    fn new(shader_id: u32) -> SkyboxShader {
        Self {}
    }
}

impl ShaderProgram<SkyboxShader> {
    pub fn bind_diffuse(&self, diffuse: u32) {
        unsafe {
            gl::UseProgram(self.shader_id);
            let mat_texture =
                gl::GetUniformLocation(self.shader_id, mem::transmute("mat_texture\0".as_ptr()));
            match mat_texture {
                -1 => {
                    println!("Failed to load uniform: 'mat_texture'");
                }
                _ => {
                    gl::Uniform1i(mat_texture, diffuse as i32);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct MeshShader {
    directional_light_count_id: i32,
    directional_light_ids: [[i32; 4]; 5],
    directional_light_count: u32,
}

impl Shader<MeshShader> for MeshShader {
    fn new(shader_id: u32) -> MeshShader {
        let mut directional_light_ids: [[i32; 4]; 5] = [[0; 4]; 5];
        let properties = ["ambient", "diffuse", "specular", "dir"];

        unsafe {
            gl::UseProgram(shader_id);

            directional_light_ids
                .iter_mut()
                .enumerate()
                .for_each(|(i, directional_light_id)| {
                    directional_light_id
                        .iter_mut()
                        .enumerate()
                        .for_each(|(j, property_id)| {
                            let name = format!("directional_lights[{}].{}\0", i, properties[j]);
                            *property_id =
                                gl::GetUniformLocation(shader_id, mem::transmute(name.as_ptr()));
                            if *property_id == -1 {
                                println!("Couldn't find uniform: {} ", name);
                            }
                        });
                });

            MeshShader {
                directional_light_ids,
                directional_light_count: 0,
                directional_light_count_id: 0,
            }
        }
    }
}

impl ShaderProgram<MeshShader> {
    pub fn bind_diffuse(&self, diffuse: u32) {
        unsafe {
            gl::UseProgram(self.shader_id);
            let mat_texture =
                gl::GetUniformLocation(self.shader_id, mem::transmute("mat_texture\0".as_ptr()));
            match mat_texture {
                -1 => {
                    println!("Failed to load uniform: 'mat_texture'");
                }
                _ => {
                    gl::Uniform1i(mat_texture, diffuse as i32);
                }
            }
        }
    }

    pub fn add_point_light(&self, pos: &Vec3, color: &LightColor) {
        todo!()
    }
    pub fn add_directional_light(&mut self, dir: &Vec3, color: &LightColor) {
        unsafe {
            let i = self.shader.directional_light_count as usize;
            self.shader.directional_light_count += 1;

            gl::UseProgram(self.shader_id);
            gl::Uniform3f(
                self.shader.directional_light_ids[i][0],
                color.ambient.x,
                color.ambient.y,
                color.ambient.z,
            );
            gl::Uniform3f(
                self.shader.directional_light_ids[i][1],
                color.diffuse.x,
                color.diffuse.y,
                color.diffuse.z,
            );
            gl::Uniform3f(
                self.shader.directional_light_ids[i][2],
                color.specular.x,
                color.specular.y,
                color.specular.z,
            );
            gl::Uniform3f(self.shader.directional_light_ids[i][3], dir.x, dir.y, dir.z);
            gl::Uniform1ui(
                self.shader.directional_light_count_id,
                self.shader.directional_light_count,
            );
        }
    }

    pub fn reset_directional_lights(&mut self) {
        unsafe {
            self.shader.directional_light_count = 0;
            gl::Uniform1ui(
                self.shader.directional_light_count_id,
                self.shader.directional_light_count,
            );
        }
    }
}

impl<T: Clone> Default for ShaderProgram<T> {
    fn default() -> Self {
        todo!()
    }
}
