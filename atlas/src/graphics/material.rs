use std::{mem, path::PathBuf};

use glad_gl::gl;
use image::GenericImageView;

use crate::game_root::GameError;

pub trait Material {}

#[derive(Clone)]
pub enum ChannelLayout {
    Rgb,
    Rgba,
}

impl ChannelLayout {
    pub fn from(path: &PathBuf) -> Option<ChannelLayout> {
        let extension = path.extension()?;
        let extension = extension.to_str()?;
        match extension {
            "jpg" => Some(ChannelLayout::Rgb),
            "jpeg" => Some(ChannelLayout::Rgb),
            "png" => Some(ChannelLayout::Rgba),
            _ => None,
        }
    }

    pub fn into_gl(&self) -> u32 {
        match self {
            ChannelLayout::Rgb => gl::RGB,
            ChannelLayout::Rgba => gl::RGBA,
        }
    }
}

#[derive(Clone)]
pub struct TextureMaterial {
    texture_id: u32,
    dimensions: (u32, u32),
    channel_layout: ChannelLayout,
}

impl Material for TextureMaterial {}

impl TextureMaterial {
    pub fn from_color((r, g, b): (f32, f32, f32)) -> Self {
        let color_texture: [u8; 6] = [
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            0 as u8,
            0 as u8,
            0 as u8,
        ];

        let dimensions = (2, 1);
        let channel_layout = ChannelLayout::Rgb;

        let texture_id = Self::create_texture();
        Self::load_texture(&color_texture, &channel_layout, dimensions);

        TextureMaterial {
            texture_id,
            dimensions,
            channel_layout: ChannelLayout::Rgb,
        }
    }

    pub fn from_file(path: &PathBuf) -> Result<TextureMaterial, GameError> {
        let Ok(img) = image::open(path.to_str().unwrap()) else {return Err(GameError::new(&format!("Failed to open texture:\n {:?}", path)))};
        let Some(channel_layout) = ChannelLayout::from(path) else {return Err(GameError::new(&format!("Failed to read extension: '{:?}'", path)))};

        let img_data: Result<&[u8], GameError> = match channel_layout {
            ChannelLayout::Rgb => {
                let Some(data) = img.as_rgb8() else {return Err(GameError::new(&format!("Failed to read data from texture: '{:?}'", path)))};
                Ok(data as &[u8])
            }
            ChannelLayout::Rgba => {
                let Some(data) = img.as_rgba8() else {return Err(GameError::new(&format!("Failed to read data from texture: '{:?}'", path)))};
                Ok(data as &[u8])
            }
        };

        let img_data = img_data?;
        let texture_id = Self::create_texture();
        Self::load_texture(img_data, &channel_layout, img.dimensions());

        Ok(TextureMaterial {
            texture_id,
            dimensions: (0, 0),
            channel_layout,
        })
    }

    pub fn texture(&self) -> u32 {
        self.texture_id
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
            gl::ActiveTexture(gl::TEXTURE0 + self.texture_id);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    fn load_texture(data: &[u8], channel_layout: &ChannelLayout, dimensions: (u32, u32)) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                channel_layout.into_gl() as i32,
                dimensions.0 as i32,
                dimensions.1 as i32,
                0,
                channel_layout.into_gl(),
                gl::UNSIGNED_BYTE,
                mem::transmute(data.as_ptr()),
            );
        }
    }
}
