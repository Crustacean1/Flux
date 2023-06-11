use std::mem;

use glad_gl::gl;

use crate::game_root::GameError;

use super::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct SkyboxShader {
    billboard_uniform: i32,
    projection_uniform: i32,
    view_uniform: i32,
}

impl Shader<SkyboxShader> for SkyboxShader {
    fn build(shader_id: u32) -> Result<SkyboxShader, GameError> {
        let billboard_uniform = ShaderProgram::<Self>::get_location(shader_id, "billboard\0")?;
        let projection_view_uniform =
            ShaderProgram::<Self>::get_location(shader_id, "projection\0")?;
        let view_uniform = ShaderProgram::<Self>::get_location(shader_id, "view\0")?;
        Ok(Self {
            billboard_uniform,
            projection_uniform: projection_view_uniform,
            view_uniform,
        })
    }
}

impl ShaderProgram<SkyboxShader> {
    pub fn bind_billboard(&self, billboard: i32) {
        unsafe {
            gl::Uniform1i(self.shader.billboard_uniform, billboard);
        }
    }

    pub fn bind_projection_view(&self, projection: &[f32; 16], view: &[f32; 16]) {
        self.load_mat(projection, self.shader.projection_uniform);
        self.load_mat(view, self.shader.view_uniform);
    }
}
