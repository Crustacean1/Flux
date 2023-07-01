use glad_gl::gl;
use glam::Mat4;

use super::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct BulletShader {
    projection_uniform: i32,
    view_uniform: i32,
    texture_uniform: i32,
}

impl Shader<BulletShader> for BulletShader {
    fn build(shader_id: u32) -> Result<BulletShader, crate::game_root::GameError> {
        let projection_uniform = ShaderProgram::<Self>::get_location(shader_id, "projection\0")?;
        let view_uniform = ShaderProgram::<Self>::get_location(shader_id, "view\0")?;
        let texture_uniform = ShaderProgram::<Self>::get_location(shader_id, "sprite\0")?;

        Ok(Self {
            projection_uniform,
            texture_uniform,
            view_uniform,
        })
    }
}

impl ShaderProgram<BulletShader> {
    pub fn bind_projection_view(&self, projection: &Mat4, view: &Mat4) {
        self.load_mat(&projection.to_cols_array(), self.shader.projection_uniform);
        self.load_mat(&view.to_cols_array(), self.shader.view_uniform);
    }

    pub fn bind_texture(&self, texture: i32) {
        unsafe {
            gl::Uniform1i(texture, self.shader.texture_uniform);
        }
    }
}
