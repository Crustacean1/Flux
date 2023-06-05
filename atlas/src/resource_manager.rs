pub mod font;
pub mod indexer;
pub mod mesh;
pub mod resource;
pub mod scene_manager;
pub mod scene_resource_manager;
pub mod shader;
pub mod material;

use resource::Resource;

pub trait ResourceManager<T: Clone> {
    fn get(&mut self, res_id: &str) -> Resource<T>;
    fn register(&mut self, res_id: &str, resource: T);
}
