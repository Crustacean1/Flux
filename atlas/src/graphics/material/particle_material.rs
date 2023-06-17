use std::path::PathBuf;

use glad_gl::gl;

use crate::{
    game_root::GameError,
    graphics::{shaders::particle_shader::ParticleShader, texture::Texture},
};

use super::Material;

#[derive(Clone)]
pub struct ParticleMaterial {
    pub texture: Texture,
}

impl Default for ParticleMaterial {
    fn default() -> Self {
        Self {
            texture: Texture::from_color((1.0, 0.0, 1.0)),
        }
    }
}

impl Material for ParticleMaterial {
    type Shader = ParticleShader;

    fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            self.texture.bind();
        }
    }
}

impl ParticleMaterial {
    pub fn load(textures: &Vec<PathBuf>) -> Result<Self, GameError> {
        let texture = textures
            .iter()
            .next()
            .ok_or(GameError::new("No texture found for particle material"))?;

        Ok(Self {
            texture: Texture::from_file(texture)?,
        })
    }
}
