pub mod font;
pub mod indexer;
pub mod model;
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
    type Resource: Clone + Default;
    fn is_resource(path: &PathBuf) -> bool;
    fn load_resource(contents: &[PathBuf]) -> Result<Self::Resource, GameError>;
}

pub fn try_get_file<'a>(file: &str, entries: &'a [PathBuf]) -> Result<&'a PathBuf, GameError> {
    entries
        .iter()
        .find(|entry| entry.file_name().map_or(false, |fname| fname == file))
        .ok_or(GameError::new(&format!("Failed to find '{}'", file)))
}
