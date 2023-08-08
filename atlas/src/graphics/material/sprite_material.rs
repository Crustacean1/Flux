use std::path::PathBuf;

use glad_gl::gl;

use crate::{game_root::GameError, graphics::texture::Texture, resource_manager::ResourceLoader};

use super::Material;

#[derive(Clone)]
pub struct SpriteMaterial {
    pub tex: Texture,
}

impl ResourceLoader for SpriteMaterial {
    type Resource = SpriteMaterial;

    fn is_resource(path: &PathBuf) -> bool {
        path.extension().map_or(false, |e| e == "sprite")
    }

    fn load_resource(contents: &[PathBuf]) -> Result<Self::Resource, GameError> {
        contents
            .iter()
            .next()
            .map(|tex| SpriteMaterial {
                tex: Texture::from_file(tex).unwrap(),
            })
            .ok_or(GameError::new("Failed to load sprite "))
    }
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            tex: Texture::from_color((1.0, 0.0, 1.0)),
        }
    }
}

impl Material for SpriteMaterial {
    fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            self.tex.bind();
        }
    }
}
