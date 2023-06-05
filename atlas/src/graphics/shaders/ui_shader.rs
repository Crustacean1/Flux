use glad_gl::gl;

use crate::game_root::GameError;

use super::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct SpriteShader {
    projection_view_model_uniform: i32,
    sprite_texuture_uniform: i32,
}

impl Shader<SpriteShader> for SpriteShader {
    fn build(shader_id: u32) -> Result<SpriteShader, GameError> {
        let projection_view_model_uniform =
            ShaderProgram::<Self>::get_location(shader_id, "projection_view_model\0")?;
        let sprite_texuture_uniform =
            ShaderProgram::<Self>::get_location(shader_id, "sprite_texture\0")?;

        Ok(Self {
            projection_view_model_uniform,
            sprite_texuture_uniform,
        })
    }
}

impl ShaderProgram<SpriteShader> {
    pub fn bind_texture(&self, texture_id: u32) {
        unsafe {
            gl::UseProgram(self.shader_id);
            gl::Uniform1i(self.shader.sprite_texuture_uniform, texture_id as i32);
        }
    }

    pub fn bind_projection_view_model(&self, projection_view_model: &[f32; 16]) {
        Self::load_mat(
            &self,
            projection_view_model,
            self.shader.projection_view_model_uniform,
        )
    }
}
