use atlas::components::ComponentEntity;
use glam::{Vec3, Vec4};

pub struct Transform {
    entity_id: usize,
    position: Vec3,
    scale: Vec3,
    rotation: Vec4,
}

impl ComponentEntity for Transform {
    fn entity_id(&self) -> usize {
        self.entity_id
    }
}
