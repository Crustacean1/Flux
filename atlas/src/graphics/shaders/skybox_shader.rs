use std::mem;

use glad_gl::gl;

use crate::game_root::GameError;

use super::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct SkyboxShader {
    billboard_uniforms: [i32; 6],
    projection_uniform: i32,
    view_uniform: i32,
}

impl Shader<SkyboxShader> for SkyboxShader {
    fn build(shader_id: u32) -> Result<SkyboxShader, GameError> {
        let billboard_uniforms: Vec<_> = (0..6)
            .filter_map(|i| {
                ShaderProgram::<Self>::get_location(shader_id, &format!("billboards[{}]\0", i)).ok()
            })
            .collect();
        let projection_view_uniform =
            ShaderProgram::<Self>::get_location(shader_id, "projection\0")?;
        let view_uniform = ShaderProgram::<Self>::get_location(shader_id, "view\0")?;

        Ok(Self {
            billboard_uniforms: billboard_uniforms
                .try_into()
                .map_err(|e| GameError::new("Billboard uniforms not found in skybox shader"))?,
            projection_uniform: projection_view_uniform,
            view_uniform,
        })
    }
}

impl ShaderProgram<SkyboxShader> {
    pub fn bind_billboard(&self, i: usize, billboard: i32) {
        unsafe {
            /*if i < self.shader.billboard_uniforms.len() {
                gl::Uniform1i(self.shader.billboard_uniforms[i], billboard);
            }*/

            gl::Uniform1i(self.shader.billboard_uniforms[i], billboard);
        }
    }

    pub fn bind_projection_view(&self, projection: &[f32; 16], view: &[f32; 16]) {
        self.load_mat(projection, self.shader.projection_uniform);
        self.load_mat(view, self.shader.view_uniform);
    }
}
