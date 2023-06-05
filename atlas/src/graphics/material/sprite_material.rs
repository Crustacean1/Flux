use std::path::PathBuf;

use crate::{
    game_root::GameError,
    graphics::{
        shaders::{ui_shader::SpriteShader, ShaderProgram},
        texture::Texture,
    },
};

use super::{load_named_texture, Material};

#[derive(Clone)]
pub struct SpriteMaterial {
    tex: Texture,
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            tex: Texture::from_color((1.0, 0.0, 1.0)),
        }
    }
}

impl Material for SpriteMaterial {
    type Shader = SpriteShader;
    fn bind(&self) {
        self.tex.bind();
    }
}

impl SpriteMaterial {
    pub fn load(textures: &Vec<PathBuf>) -> Result<Self, GameError> {
        Ok(SpriteMaterial {
            tex: load_named_texture("texture", textures)?,
        })
    }
}
