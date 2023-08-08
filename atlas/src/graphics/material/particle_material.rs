use std::path::PathBuf;

use glad_gl::gl;

use crate::{game_root::GameError, graphics::texture::Texture, resource_manager::ResourceLoader};

use super::Material;

#[derive(Clone)]
pub struct ParticleMaterial {
    pub texture: Texture,
}

impl ResourceLoader for ParticleMaterial {
    type Resource = ParticleMaterial;

    fn is_resource(path: &PathBuf) -> bool {
        path.extension().map_or(false, |e| e == "particle")
    }

    fn load_resource(contents: &[PathBuf]) -> Result<Self::Resource, GameError> {
        let texture = contents
            .iter()
            .next()
            .ok_or(GameError::new("No texture found for particle material"))?;

        Ok(Self {
            texture: Texture::from_file(texture)?,
        })
    }
}

impl Default for ParticleMaterial {
    fn default() -> Self {
        Self {
            texture: Texture::from_color((1.0, 0.0, 1.0)),
        }
    }
}

impl Material for ParticleMaterial {
    fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            self.texture.bind();
        }
    }
}
