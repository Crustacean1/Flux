use std::mem;

use glad_gl::gl;
use glam::Vec3;

use crate::{game_root::GameError, graphics::lights::LightColor};

use super::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct MeshShader {
    projection_view_model_uniform: i32,
    view_model_uniform: i32,
    directional_light_count_uniform: i32,
    directional_light_uniforms: [DirectionalLightUniform; 4],
    material_uniform: MaterialUniform,
}

#[derive(Clone, Copy, Debug)]
struct DirectionalLightUniform {
    direction: i32,
    ambient: i32,
    diffuse: i32,
    specular: i32,
}

#[derive(Clone, Copy, Debug)]
struct MaterialUniform {
    diffuse: i32,
}

impl Shader<MeshShader> for MeshShader {
    fn build(shader_id: u32) -> Result<MeshShader, GameError> {
        unsafe {
            gl::UseProgram(shader_id);

            let projection_view_model_uniform =
                ShaderProgram::<Self>::get_location(shader_id, "projection_view_model\0")?;

            let view_model_uniform =
                ShaderProgram::<Self>::get_location(shader_id, "view_model\0")?;

            let directional_light_count_uniform =
                ShaderProgram::<Self>::get_location(shader_id, "directional_light_count\0")?;

            let directional_lights: Result<[DirectionalLightUniform; 4], _> = (0..5)
                .filter_map(|i| {
                    let instance = format!("directional_lights[{}]", i);

                    let direction = ShaderProgram::<Self>::get_location(
                        shader_id,
                        &format!("{}.direction\0", instance),
                    )
                    .ok()?;
                    let ambient = ShaderProgram::<Self>::get_location(
                        shader_id,
                        &format!("{}.ambient\0", instance),
                    )
                    .ok()?;
                    let diffuse = ShaderProgram::<Self>::get_location(
                        shader_id,
                        &format!("{}.diffuse\0", instance),
                    )
                    .ok()?;
                    let specular = ShaderProgram::<Self>::get_location(
                        shader_id,
                        &format!("{}.specular\0", instance),
                    )
                    .ok()?;
                    Some(DirectionalLightUniform {
                        direction,
                        ambient,
                        diffuse,
                        specular,
                    })
                })
                .collect::<Vec<_>>()
                .try_into();

            let material_uniform = MaterialUniform {
                diffuse: ShaderProgram::<Self>::get_location(
                    shader_id,
                    &format!("material.diffuse\0"),
                )?,
            };

            let Ok(directional_light_uniforms) = directional_lights else {
                return GameError::err(format!("Failed to find directional_lights in shader"));
            };

            Ok(MeshShader {
                directional_light_uniforms,
                material_uniform,
                directional_light_count_uniform,
                projection_view_model_uniform,
                view_model_uniform,
            })
        }
    }
}

impl ShaderProgram<MeshShader> {
    pub fn bind_diffuse(&self, diffuse: u32) {
        unsafe {
            gl::UseProgram(self.shader_id);
            gl::Uniform1i(self.shader.material_uniform.diffuse, diffuse as i32);
        }
    }

    pub fn bind_projection_view_model(&self, projection_view_model: &[f32; 16]) {
        Self::load_mat(
            &self,
            projection_view_model,
            self.shader.projection_view_model_uniform,
        )
    }

    pub fn bind_view_model(&self, view_model: &[f32; 16]) {
        Self::load_mat(&self, view_model, self.shader.view_model_uniform)
    }

    pub fn bind_directional_light(&self, i: usize, dir: &Vec3, color: &LightColor) {
        unsafe {
            gl::Uniform3f(
                self.shader.directional_light_uniforms[i].ambient,
                color.ambient.x,
                color.ambient.y,
                color.ambient.z,
            );
            gl::Uniform3f(
                self.shader.directional_light_uniforms[i].diffuse,
                color.diffuse.x,
                color.diffuse.y,
                color.diffuse.z,
            );
            gl::Uniform3f(
                self.shader.directional_light_uniforms[i].specular,
                color.specular.x,
                color.specular.y,
                color.specular.z,
            );
            gl::Uniform3f(
                self.shader.directional_light_uniforms[i].direction,
                dir.x,
                dir.y,
                dir.z,
            );
        }
    }

    pub fn bind_directional_light_count(&self, count: u32) {
        unsafe {
            gl::Uniform1i(self.shader.directional_light_count_uniform, count as i32);
        }
    }
}
