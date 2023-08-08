use std::path::PathBuf;

use glad_gl::gl;

use crate::{game_root::GameError, graphics::texture::Texture, resource_manager::ResourceLoader};

use super::Material;

#[derive(Clone)]
pub struct BulletMaterial {
    pub texture: Texture,
}

impl ResourceLoader for BulletMaterial {
    type Resource = BulletMaterial;

    fn is_resource(path: &PathBuf) -> bool {
        path.extension().map_or(false, |e| e == "bullet")
    }

    fn load_resource(contents: &[PathBuf]) -> Result<Self::Resource, GameError> {
        let texture = contents
            .iter()
            .next()
            .ok_or(GameError::new("No texture found for bullet material"))?;

        Ok(Self {
            texture: Texture::from_file(texture)?,
        })
    }
}

impl Default for BulletMaterial {
    fn default() -> Self {
        Self {
            texture: Texture::from_color((1.0, 0.0, 1.0)),
        }
    }
}

impl Material for BulletMaterial {
    fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            self.texture.bind();
        }
    }
}

impl BulletMaterial {
    pub fn load(textures: &Vec<PathBuf>) -> Result<Self, GameError> {
        let texture = textures
            .iter()
            .next()
            .ok_or(GameError::new("No texture found for bullet material"))?;

        Ok(Self {
            texture: Texture::from_file(texture)?,
        })
    }
}
