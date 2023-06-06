use crate::game_root::GameError;

use super::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct TextShader {
    projection_view_model_uniform: i32,
}

impl ShaderProgram<TextShader> {
    pub fn bind_projection_view_model(&mut self, projection_view_model: &[f32; 16]) {
        Self::load_mat(
            &self,
            projection_view_model,
            self.shader.projection_view_model_uniform,
        )
    }
}

impl Shader<TextShader> for TextShader {
    fn build(shader_id: u32) -> Result<TextShader, GameError> {
        let projection_view_model_uniform =
            ShaderProgram::<Self>::get_location(shader_id, "projection_view_model\0")?;
        Ok(Self {
            projection_view_model_uniform,
        })
    }
}
