use glam::{Vec3, Vec4};

pub struct Transform {
    entity_id: usize,
    position: Vec3,
    scale: Vec3,
    rotation: Vec4,
}
