use std::path::PathBuf;

use glad_gl::gl;

use crate::{
    game_root::GameError,
    graphics::{
        shaders::{mesh_shader::MeshShader, ShaderProgram},
        texture::Texture,
    },
};

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
    pub fn load(textures: &Vec<PathBuf>) -> Result<Self, GameError> {
        Ok(PhongMaterial {
            diffuse: load_named_texture("diffuse", textures)?,
        })
    }
}
