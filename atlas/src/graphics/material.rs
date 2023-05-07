use std::mem;

use glad_gl::gl;

pub trait Material {}

pub struct TextureMaterial {
    texture_id: u32,
}

impl Material for TextureMaterial {}

impl TextureMaterial {
    pub fn from_color(r: f32, g: f32, b: f32) -> Self {
        let mut texture_id: u32 = 0;

        let (width, height) = (1, 1);
        let color_texture: [u8; 3] = [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8];

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width,
                height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                mem::transmute(color_texture.as_ptr()),
            );
        }
        TextureMaterial { texture_id }
    }
}
