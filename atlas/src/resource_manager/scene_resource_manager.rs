use std::{collections::HashMap, env, path::PathBuf};

use crate::{
    game_root::GameError,
    graphics::{
        material::TextureMaterial,
        mesh::Primitive,
        vertices::base_vertices::{TriangleIndex, Vertex3PT},
    },
};

use super::{
    indexer::index_resources, mesh::collect_mesh, resource::Resource, texture::collect_textures,
    ResourceManager,
};

pub enum LazyResource<T> {
    Unloaded(PathBuf),
    Loaded(T),
}

pub struct SceneResourceManager {
    textures: HashMap<String, LazyResource<Resource<TextureMaterial>>>,
    meshes: HashMap<String, Resource<Primitive<Vertex3PT, TriangleIndex>>>,
}

impl SceneResourceManager {
    pub fn build() -> Result<Self, GameError> {
        let resource_groups = index_resources(&Self::root_path()?)?;
        let textures = collect_textures(&resource_groups);
        let meshes = collect_mesh(&resource_groups);

        Ok(SceneResourceManager { textures, meshes })
    }

    fn root_path() -> Result<PathBuf, GameError> {
        let mut game_dir = env::current_exe()?;
        game_dir.pop();
        game_dir.pop();
        game_dir.pop();
        game_dir.push("flux");
        game_dir.push("assets");
        Ok(game_dir)
    }
}

impl ResourceManager<TextureMaterial> for SceneResourceManager {
    fn get(&mut self, res_id: &str) -> Result<Resource<TextureMaterial>, GameError> {
        if let Some(lazy_material) = self.textures.get_mut(res_id) {
            match lazy_material {
                LazyResource::Loaded(material) => Ok(material.clone()),
                LazyResource::Unloaded(filepath) => {
                    let new_material = TextureMaterial::from_file(filepath)?;
                    let new_material_resource = Resource::new(
                        String::from(filepath.file_stem().unwrap().to_str().unwrap()),
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

impl ResourceManager<Primitive<Vertex3PT, TriangleIndex>> for SceneResourceManager {
    fn get(
        &mut self,
        res_id: &str,
    ) -> Result<Resource<Primitive<Vertex3PT, TriangleIndex>>, GameError> {
        let Some(a) = self.meshes.get(res_id) else {return Err(GameError::new("No mesh with id: {}"))};
        Ok(a.clone())
    }
}
