use std::{
    any::{self, Any},
    collections::HashMap,
    env, fs,
    path::PathBuf,
};

use crate::{
    game_root::GameError,
    graphics::{
        material::{
            bullet_material::BulletMaterial, particle_material::ParticleMaterial,
            phong_material::PhongMaterial, skybox_material::SkyboxMaterial,
            sprite_material::SpriteMaterial,
        },
        model::Model,
        shaders::{
            bullet_shader::BulletShaderDefinition, flat_shader::FlatShaderDefinition,
            health_shader::HealthShaderDefinition, mesh_shader::MeshShaderDefinition,
            particle_shader::ParticleShaderDefinition, skybox_shader::SkyboxShaderDefinition,
            sprite_shader::SpriteShaderDefinition, text_shader::TextShaderDefinition,
        },
    },
};

use super::{
    font::Font, indexer::index_resources, resource::Resource, ResourceLoader, ResourceManager,
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
        println!("Indexing resources in '{}'", root);
        let resource_index = index_resources(&Self::root_path(root)?);

        let mut res_man = SceneResourceManager {
            resources: Vec::new(),
        };

        res_man.build_resource::<Model>(&resource_index);
        res_man.build_resource::<Font>(&resource_index);

        //Shaders
        res_man.build_resource::<SpriteShaderDefinition>(&resource_index);
        res_man.build_resource::<SkyboxShaderDefinition>(&resource_index);
        res_man.build_resource::<MeshShaderDefinition>(&resource_index);
        res_man.build_resource::<FlatShaderDefinition>(&resource_index);
        res_man.build_resource::<TextShaderDefinition>(&resource_index);
        res_man.build_resource::<BulletShaderDefinition>(&resource_index);
        res_man.build_resource::<ParticleShaderDefinition>(&resource_index);
        res_man.build_resource::<HealthShaderDefinition>(&resource_index);

        //Materials
        res_man.build_resource::<SpriteMaterial>(&resource_index);
        res_man.build_resource::<PhongMaterial>(&resource_index);
        res_man.build_resource::<SkyboxMaterial>(&resource_index);
        res_man.build_resource::<ParticleMaterial>(&resource_index);
        res_man.build_resource::<BulletMaterial>(&resource_index);

        Ok(res_man)
    }

    pub fn build_resource<T: ResourceLoader + 'static>(
        &mut self,
        index: &[(String, String, PathBuf)],
    ) {
        index
            .iter()
            .filter(|(_, _, dir)| T::is_resource(dir))
            .for_each(|(res_id, _, dir)| {
                let dir_content = fs::read_dir(dir).map(|dir| -> Vec<_> {
                    dir.filter_map(|entry| entry.map(|entry| entry.path()).ok())
                        .collect()
                });

                if let Ok(dir_content) = dir_content {
                    match T::load_resource(&dir_content) {
                        Ok(resource) => self.register(res_id, resource),
                        Err(e) => println!("Failed to load resource '{}':\n{}", res_id, e),
                    }
                }
            })
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
