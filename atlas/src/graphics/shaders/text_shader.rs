use std::mem;

use glad_gl::gl;

use crate::graphics::texture::Texture;

use super::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct TextShader;

impl ShaderProgram<TextShader> {
    pub fn load_character(&self, character: Texture) {
        unsafe {
            gl::UseProgram(self.shader_id);
            let mat_texture =
                gl::GetUniformLocation(self.shader_id, mem::transmute("character\0".as_ptr()));
            match mat_texture {
                -1 => {
                    println!("Failed to load uniform: 'character'");
                }
                _ => {
                    gl::Uniform1i(mat_texture, character.texture() as i32);
                }
            }
        }
    }
}

impl Shader<TextShader> for TextShader {
    fn new(shader_id: u32) -> TextShader {
        Self {}
    }
}

