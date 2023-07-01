use crate::{
    components::{collider::Collider, physical_body::PhysicalBody},
    graphics::{mesh::Mesh, primitive::Primitive, vertices::sphere::sphere},
    resource_manager::{self, scene_resource_manager::SceneResourceManager, ResourceManager},
};

pub struct BulletEntity {
    pub collider: Collider,
    pub body: PhysicalBody,
    pub lifetime: f32,
}
