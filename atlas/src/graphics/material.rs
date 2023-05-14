use std::{mem, path::PathBuf};

use glad_gl::gl;
use image::GenericImageView;

use crate::game_root::GameError;

pub trait Material {}

#[derive(Clone)]
pub struct TextureMaterial {
    texture_id: u32,
    dimensions: (u32, u32),
}

impl Material for TextureMaterial {}

impl TextureMaterial {
    pub fn from_color((r, g, b): (f32, f32, f32), free_texture_unit: u32) -> Self {
        let color_texture: [u8; 6] = [
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            0 as u8,
            0 as u8,
            0 as u8,
        ];
        let dimensions = (2, 1);

        let texture_id = Self::create_texture(free_texture_unit);
        Self::load_texture(&color_texture, dimensions);

        TextureMaterial {
            texture_id,
            dimensions,
        }
    }

    pub fn from_file(path: &PathBuf, free_texture_unit: u32) -> Result<TextureMaterial, GameError> {
        let Ok(img) = image::open(path.to_str().unwrap()) else {return Err(GameError::new(&format!("Failed to open texture: {:?}", path)))};
        let Some(img_data) = img.as_rgb8() else {return Err(GameError::new(&format!("Failed to access image data: {:?}", path)))};

        let texture_id = Self::create_texture(free_texture_unit);

        println!("Dimensions: {:?}", img.dimensions());

        Self::load_texture(img_data, img.dimensions());

        Ok(TextureMaterial {
            texture_id,
            dimensions: (0, 0),
        })
    }

    pub fn texture(&self) -> u32 {
        self.texture_id
    }

    fn create_texture(free_unit: u32) -> u32 {
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::ActiveTexture(free_unit);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        texture_id
    }

    fn load_texture(data: &[u8], dimensions: (u32, u32)) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                dimensions.0 as i32,
                dimensions.1 as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                mem::transmute(data.as_ptr()),
            );
        }
    }
}
