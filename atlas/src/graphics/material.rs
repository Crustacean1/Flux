pub mod particle_material;
pub mod phong_material;
pub mod skybox_material;
pub mod sprite_material;

use std::path::PathBuf;

use crate::game_root::GameError;

use super::{shaders::ShaderProgram, texture::Texture};

pub trait Material {
    type Shader: Clone;
    fn bind(&self, shader: &ShaderProgram<Self::Shader>);
}

fn load_named_texture(name: &str, textures: &Vec<PathBuf>) -> Result<Texture, GameError> {
    if let Some(tex) = textures.iter().find(|t| {
        t.file_stem().map_or(false, |texture_name| {
            texture_name
                .to_str()
                .map_or(false, |texture_name| texture_name == name)
        })
    }) {
        Ok(Texture::from_file(tex)?)
    } else {
        GameError::err(format!("Texture '{}' not found", name))
    }
}
