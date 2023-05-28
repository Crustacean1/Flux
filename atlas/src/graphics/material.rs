use std::path::PathBuf;

use crate::game_root::GameError;

use super::{
    shaders::{MeshShader, ShaderProgram, SkyboxShader, UiShader},
    texture::Texture,
};

pub trait Material {
    type Shader: Clone;
    fn bind(&self, shader: &ShaderProgram<Self::Shader>);
}

#[derive(Clone)]
pub struct TextureMaterial {
    pub diffuse: Texture,
}

impl Default for TextureMaterial {
    fn default() -> Self {
        Self {
            diffuse: Texture::from_color((0.2, 0.2, 0.2)),
        }
    }
}

impl Material for TextureMaterial {
    type Shader = MeshShader;
    fn bind(&self, shader: &ShaderProgram<MeshShader>) {
        self.diffuse.bind();
        shader.bind_diffuse(self.diffuse.texture());
    }
}

impl TextureMaterial {
    pub fn load(textures: &Vec<PathBuf>) -> Result<Self, GameError> {
        Ok(TextureMaterial {
            diffuse: load_named_texture("diffuse", textures)?,
        })
    }
}

#[derive(Clone)]
pub struct UiMaterial {
    tex: Texture,
}

impl Default for UiMaterial {
    fn default() -> Self {
        Self {
            tex: Texture::from_color((1.0, 0.0, 1.0)),
        }
    }
}

impl Material for UiMaterial {
    type Shader = UiShader;
    fn bind(&self, shader: &ShaderProgram<UiShader>) {
        self.tex.bind();
        shader.bind_texture(self.tex.texture());
    }
}

impl UiMaterial {
    pub fn load(textures: &Vec<PathBuf>) -> Result<Self, GameError> {
        Ok(UiMaterial {
            tex: load_named_texture("texture", textures)?,
        })
    }
}

fn load_named_texture(name: &str, textures: &Vec<PathBuf>) -> Result<Texture, GameError> {
    if let Some(tex) = textures.iter().find(|t| {
        t.file_stem().map_or(false, |name| {
            name.to_str().map_or(false, |name| name == "diffuse")
        })
    }) {
        Ok(Texture::from_file(tex)?)
    } else {
        Err(GameError::new("Diffuse texture not found"))
    }
}
