use std::mem;

use glad_gl::gl;

use crate::{game_root::GameError, graphics::texture::Texture};

use super::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct TextShader {
    projection_view_model_uniform: i32,
}

impl ShaderProgram<TextShader> {
    pub fn load_character(&self) {
        unsafe {
            gl::UseProgram(self.shader_id);
            let mat_texture =
                gl::GetUniformLocation(self.shader_id, mem::transmute("character\0".as_ptr()));
            match mat_texture {
                -1 => {
                    println!("Failed to load uniform: 'character'");
                }
                _ => {
                    gl::Uniform1i(mat_texture, 0);
                }
            }
        }
    }

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
