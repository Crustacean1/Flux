use std::path::PathBuf;

use glad_gl::gl;

use crate::{game_root::GameError, graphics::texture::Texture, resource_manager::ResourceLoader};

use super::{load_named_texture, Material};

#[derive(Clone)]
pub struct PhongMaterial {
    pub diffuse: Texture,
}

impl Default for PhongMaterial {
    fn default() -> Self {
        Self {
            diffuse: Texture::from_color((0.2, 0.2, 0.2)),
        }
    }
}

impl Material for PhongMaterial {
    fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            self.diffuse.bind();
        }
    }
}

impl PhongMaterial {
    pub fn load(textures: &[PathBuf]) -> Result<Self, GameError> {
        Ok(PhongMaterial {
            diffuse: load_named_texture("diffuse", textures)?,
        })
    }
}

impl ResourceLoader for PhongMaterial {
    type Resource = PhongMaterial;

    fn is_resource(path: &PathBuf) -> bool {
        path.extension().map_or(false, |e| e == "mat")
    }

    fn load_resource(contents: &[PathBuf]) -> Result<Self::Resource, GameError> {
        Ok(PhongMaterial {
            diffuse: load_named_texture("diffuse", contents)?,
        })
    }
}
