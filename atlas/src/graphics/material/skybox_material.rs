use std::path::PathBuf;

use glad_gl::gl;

use crate::{
    game_root::GameError,
    graphics::{material::load_named_texture, texture::Texture},
    resource_manager::ResourceLoader,
};

use super::Material;

#[derive(Clone, Copy)]
pub struct SkyboxMaterial {
    sides: [Texture; 6],
}

impl Default for SkyboxMaterial {
    fn default() -> Self {
        todo!()
    }
}

impl Material for SkyboxMaterial {
    fn bind(&self) {
        unsafe {
            self.sides.iter().enumerate().for_each(|(i, side)| {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                side.bind();
            });
        }
    }
}

impl SkyboxMaterial {
    pub fn get_side_sampler(&self, i: usize) -> i32 {
        (gl::TEXTURE0 + i as u32) as i32
    }
}

impl ResourceLoader for SkyboxMaterial {
    type Resource = SkyboxMaterial;

    fn is_resource(path: &PathBuf) -> bool {
        path.extension().map_or(false, |e| e == "skybox")
    }

    fn load_resource(contents: &[PathBuf]) -> Result<Self::Resource, GameError> {
        let sides = ["front", "back", "bot", "top", "left", "right"];

        let sides: Vec<_> = sides
            .iter()
            .filter_map(|side| load_named_texture(side, contents).ok())
            .collect();

        let sides: [_; 6] = sides
            .try_into()
            .map_err(|_e| GameError::new(&format!("Failed to load textures of skybox",)))?;

        Ok(Self { sides })
    }
}
