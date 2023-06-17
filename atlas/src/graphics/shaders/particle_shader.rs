use super::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct ParticleShader {
    projection_uniform: i32,
    view_uniform: i32,
}

impl Shader<ParticleShader> for ParticleShader {
    fn build(shader_id: u32) -> Result<ParticleShader, crate::game_root::GameError> {
        let view_uniform = ShaderProgram::<Self>::get_location(shader_id, "view\0")?;
        let projection_uniform = ShaderProgram::<Self>::get_location(shader_id, "projection\0")?;
        Ok(Self {
            projection_uniform,
            view_uniform,
        })
    }
}

impl ShaderProgram<ParticleShader> {
    pub fn bind_projection_view(&self, projection: &[f32; 16], view: &[f32; 16]) {
        self.load_mat(projection, self.shader.projection_uniform);
        self.load_mat(view, self.shader.view_uniform);
    }
}
