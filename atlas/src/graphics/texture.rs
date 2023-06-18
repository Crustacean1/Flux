use core::slice;
use std::{mem, path::PathBuf};

use glad_gl::gl;
use image::GenericImageView;

use crate::game_root::GameError;

#[derive(Clone, Copy)]
pub enum ChannelLayout {
    R8,
    Rgb8,
    Rgb16,
    Rgba8,
}

impl ChannelLayout {
    pub fn gl_internal_format(&self) -> u32 {
        match self {
            ChannelLayout::R8 => gl::RED,
            ChannelLayout::Rgb8 => gl::RGB,
            ChannelLayout::Rgb16 => gl::RGB16UI,
            ChannelLayout::Rgba8 => gl::RGBA,
        }
    }

    pub fn gl_format(&self) -> u32 {
        match self {
            ChannelLayout::R8 => gl::RED,
            ChannelLayout::Rgb8 => gl::RGB,
            ChannelLayout::Rgb16 => gl::RGB,
            ChannelLayout::Rgba8 => gl::RGBA,
        }
    }

    pub fn gl_type(&self) -> u32 {
        match self {
            ChannelLayout::R8 => gl::UNSIGNED_BYTE,
            ChannelLayout::Rgb8 => gl::UNSIGNED_BYTE,
            ChannelLayout::Rgb16 => gl::UNSIGNED_BYTE,
            ChannelLayout::Rgba8 => gl::UNSIGNED_BYTE,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Texture {
    texture_id: u32,
    dimensions: (u32, u32),
    channel_layout: ChannelLayout,
}

impl Texture {
    pub fn from_color((r, g, b): (f32, f32, f32)) -> Self {
        let color_texture: [u8; 3] = [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8];

        let dimensions = (1, 1);
        let channel_layout = ChannelLayout::Rgb8;

        let texture_id = Self::create_texture();
        Self::load_texture(&color_texture, channel_layout, dimensions);

        Texture {
            texture_id,
            dimensions,
            channel_layout,
        }
    }

    pub fn from_file(path: &PathBuf) -> Result<Texture, GameError> {
        let Ok(img) = image::open(path.to_str().unwrap()) else {return Err(GameError::new(&format!("Failed to open texture:\n {:?}", path)))};

        let channel_layout = match img.color() {
            image::ColorType::Rgb8 => ChannelLayout::Rgb8,
            image::ColorType::Rgb16 => ChannelLayout::Rgb16,
            image::ColorType::Rgba8 => ChannelLayout::Rgba8,
            _ => {
                return Err(GameError::new(&format!(
                    "Failed to read extension: '{:?}' at '{:?}'",
                    img.color(),
                    path
                )))
            }
        };

        let img_data: Result<&[u8], GameError> = match channel_layout {
            ChannelLayout::R8 => {
                todo!()
            }
            ChannelLayout::Rgb8 => {
                let Some(data) = img.as_rgb8() else {return Err(GameError::new(&format!("Failed to read data from rgb texture: '{:?}'", path)))};
                Ok(data as &[u8])
            }
            ChannelLayout::Rgb16 => {
                todo!()
            }
            ChannelLayout::Rgba8 => {
                let Some(data) = img.as_rgba8() else {return Err(GameError::new(&format!("Failed to read data from rgba texture: '{:?}'", path)))};
                Ok(data as &[u8])
            }
        };

        let img_data = img_data?;
        let texture_id = Self::create_texture();
        Self::load_texture(img_data, channel_layout, img.dimensions());

        Ok(Texture {
            texture_id,
            dimensions: img.dimensions(),
            channel_layout,
        })
    }

    pub fn from_buff(
        buffer: &[u8],
        layout: ChannelLayout,
        dimensions: (u32, u32),
    ) -> Result<Texture, GameError> {
        let tex_id = Self::create_texture();
        Self::load_texture(buffer, layout, dimensions);
        Ok(Texture {
            texture_id: tex_id,
            channel_layout: layout,
            dimensions,
        })
    }

    pub fn texture(&self) -> u32 {
        self.texture_id
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    fn create_texture() -> u32 {
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        texture_id
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    fn load_texture(data: &[u8], channel_layout: ChannelLayout, dimensions: (u32, u32)) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                channel_layout.gl_internal_format() as i32,
                dimensions.0 as i32,
                dimensions.1 as i32,
                0,
                channel_layout.gl_format(),
                channel_layout.gl_type(),
                mem::transmute(data.as_ptr()),
            );
        }
    }
}
