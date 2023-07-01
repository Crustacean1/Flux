use std::path::PathBuf;

use glad_gl::gl;

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
    pub tex: Texture,
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
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            self.tex.bind();
        }
    }
}

impl SpriteMaterial {
    pub fn load(textures: &Vec<PathBuf>) -> Result<Self, GameError> {
        textures
            .iter()
            .next()
            .map(|tex| SpriteMaterial {
                tex: Texture::from_file(tex).unwrap(),
            })
            .ok_or(GameError::new("Failed to load sprite "))
    }
}
