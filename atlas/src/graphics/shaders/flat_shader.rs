use glam::Mat4;

use super::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct FlatShader {
    projection_view_uniform: i32,
}

impl Shader<FlatShader> for FlatShader {
    fn build(shader_id: u32) -> Result<FlatShader, crate::game_root::GameError> {
        let projection_view_model_uniform =
            ShaderProgram::<Self>::get_location(shader_id, &format!("projection_view\0"))?;
        Ok(Self {
            projection_view_uniform: projection_view_model_uniform,
        })
    }
}

impl ShaderProgram<FlatShader> {
    pub fn bind_projection_view(&self, pvm: &Mat4) {
        self.load_mat(&pvm.to_cols_array(), self.shader.projection_view_uniform);
    }
}
