pub mod indexer;
pub mod mesh;
pub mod resource;
pub mod root_resource_manager;
pub mod scene_resource_manager;
pub mod shader;
pub mod texture;

use resource::Resource;
use std::fmt;

use crate::game_root::GameError;

#[derive(Debug, Clone)]
pub struct ResourceError {
    msg: String,
}

impl ResourceError {
    pub fn new<T>(path: &str, msg: &str) -> Result<T, Self> {
        Err(ResourceError {
            msg: format!("Failed to load '{}':{}", path, msg),
        })
    }
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)?;
        Ok(())
    }
}

pub trait Resourcefull<T> {
    fn load(path: &str) -> Result<T, ResourceError>;
}

pub trait ResourceManager<T: Clone> {
    fn get(&mut self, res_id: &str) -> Result<Resource<T>, GameError>;
}

