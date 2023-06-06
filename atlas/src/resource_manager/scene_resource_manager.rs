use std::{
    any::{self, Any},
    collections::HashMap,
    env,
    path::PathBuf,
};

use freetype::Library;

use crate::game_root::GameError;

use super::{
    font::load_font, indexer::index_resources, material::load_mat, mesh::load_mesh,
    resource::Resource, shader::load_shader, ResourceManager,
};

pub enum LazyResource<T> {
    Unloaded(PathBuf),
    Loaded(T),
}

trait ResourceCollectionTrait<T: Default + Clone> {
    fn get(&mut self, res_id: &str) -> Option<&T>;
    fn add(&mut self, res_id: &str, resource: &T);
}

struct ResourceCollection<T> {
    resources: HashMap<String, T>,
}

impl<T: Default + Clone> ResourceCollection<T> {
    pub fn new(res_id: &str, resource: &T) -> Self {
        let mut res_col = ResourceCollection {
            resources: HashMap::new(),
        };
        res_col.add(res_id, resource);
        res_col
    }
}

impl<T: Default + Clone> ResourceCollectionTrait<T> for ResourceCollection<T> {
    fn get(&mut self, res_id: &str) -> Option<&T> {
        self.resources.get(res_id)
    }

    fn add(&mut self, res_id: &str, resource: &T) {
        self.resources
            .insert(String::from(res_id), resource.clone());
    }
}

pub struct SceneResourceManager {
    resources: Vec<Box<dyn Any>>,
}

impl SceneResourceManager {
    pub fn build(root: &str) -> Result<Self, GameError> {
        let resource_index = index_resources(&Self::root_path(root)?);

        let resources = Vec::new();
        let mut freetype_lib = Library::init().unwrap();
        let mut res_man = SceneResourceManager { resources };

        resource_index.iter().for_each(|(res_id, ext, dir)| {
            load_font(res_id, ext, dir, &mut freetype_lib, &mut res_man)
        });

        resource_index
            .iter()
            .for_each(|(res_id, ext, dir)| load_shader(res_id, ext, dir, &mut res_man));

        resource_index
            .iter()
            .for_each(|(res_id, ext, dir)| load_mat(res_id, ext, dir, &mut res_man));

        resource_index
            .iter()
            .filter(|(_, ext, _)| ext == "mesh")
            .for_each(|(res_id, _, dir)| load_mesh(res_id, dir, &mut res_man));

        Ok(res_man)
    }

    fn root_path(root: &str) -> Result<PathBuf, GameError> {
        if let Ok(mut game_dir) = env::current_exe() {
            game_dir.pop();
            game_dir.pop();
            game_dir.pop();
            game_dir.push("flux");
            game_dir.push("assets");
            game_dir.push(root);
            Ok(game_dir)
        } else {
            Err(GameError::new("Failed to read asset path"))
        }
    }
}

impl<T: Default + Clone + 'static> ResourceManager<T> for SceneResourceManager {
    fn get(&mut self, res_id: &str) -> Resource<T> {
        let resource: T = self
            .resources
            .iter_mut()
            .find_map(|resources| {
                Some(
                    resources
                        .downcast_mut::<ResourceCollection<T>>()?
                        .get(res_id)?
                        .clone(),
                )
            })
            .unwrap_or_else(|| {
                println!("Resource: '{}' not found", res_id);
                T::default()
            });

        Resource::new(res_id, resource)
    }

    fn register(&mut self, res_id: &str, resource: T) {
        println!(
            "Registering resource '{}' : '{}'",
            res_id,
            any::type_name::<T>()
        );
        if self
            .resources
            .iter_mut()
            .find_map(
                |resources| match resources.downcast_mut::<ResourceCollection<T>>() {
                    Some(resources) => {
                        resources.add(&res_id, &resource);
                        Some(())
                    }
                    _ => None,
                },
            )
            .is_none()
        {
            let collection = Box::new(ResourceCollection::<T>::new(res_id, &resource));
            self.resources.push(collection);
        }
    }
}
