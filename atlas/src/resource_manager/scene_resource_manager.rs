use std::{collections::HashMap, env, path::PathBuf};

use glad_gl::gl;

use crate::{
    game_root::GameError,
    graphics::{
        graphics_context::{self, GraphicsContext},
        material::TextureMaterial,
    },
};

use super::{
    indexer::{crawl_dirs, index_resources},
    resource::Resource,
    texture::index_textures,
    ResourceManager,
};

pub enum LazyResource<T> {
    Unloaded(PathBuf),
    Loaded(T),
}

pub struct SceneResourceManager {
    texture_unit_count: u32,
    next_texture_unit: u32,

    textures: HashMap<String, LazyResource<Resource<TextureMaterial>>>,
}

impl SceneResourceManager {
    pub fn build(graphics_context: &GraphicsContext) -> Result<Self, GameError> {
        let resource_groups = index_resources(&Self::root_path()?)?;
        let textures = index_textures(resource_groups);

        Ok(SceneResourceManager {
            texture_unit_count: graphics_context.texture_unit_count(),
            next_texture_unit: gl::TEXTURE0,
            textures,
        })
    }

    fn root_path() -> Result<PathBuf, GameError> {
        let mut game_dir = env::current_exe()?;
        game_dir.pop();
        game_dir.pop();
        game_dir.pop();
        game_dir.push("flux");
        Ok(game_dir)
    }
}

impl ResourceManager<TextureMaterial> for SceneResourceManager {
    fn get(&mut self, res_id: &str) -> Result<Resource<TextureMaterial>, GameError> {
        if let Some(lazy_material) = self.textures.get_mut(res_id) {
            match lazy_material {
                LazyResource::Loaded(material) => Ok(material.clone()),
                LazyResource::Unloaded(filepath) => {
                    let texture_unit = if self.next_texture_unit == self.texture_unit_count {
                        return Err(GameError::new(&format!("Exhausted texture units")));
                    } else {
                        self.next_texture_unit += 1;
                        self.next_texture_unit - 1
                    };

                    let new_material = TextureMaterial::from_file(filepath, texture_unit)?;
                    let new_material_resource = Resource::new(
                        filepath.file_stem().unwrap().to_str().unwrap(),
                        new_material,
                    );

                    *lazy_material = LazyResource::Loaded(new_material_resource.clone());
                    Ok(new_material_resource)
                }
            }
        } else {
            Err(GameError::new(&format!("No material with id: {}", res_id)))
        }
    }
}
