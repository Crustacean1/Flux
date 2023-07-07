pub mod font;
pub mod indexer;
pub mod material;
pub mod mesh;
pub mod resource;
pub mod scene_manager;
pub mod scene_resource_manager;

use std::path::PathBuf;

use resource::Resource;

use crate::game_root::GameError;

pub trait ResourceManager<T: Clone> {
    fn get(&mut self, res_id: &str) -> Resource<T>;
    fn register(&mut self, res_id: &str, resource: T);
}

pub trait ResourceLoader {
    type Resource;
    fn is_resource(path: &PathBuf) -> bool;
    fn load_resource(contents: &[PathBuf]) -> Result<Self::Resource, GameError>;
}
