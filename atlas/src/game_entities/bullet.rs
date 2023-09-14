use crate::components::collider::Collider;
use crate::components::physical_body::PhysicalBody;

pub struct BulletEntity {
    pub collider: Collider,
    pub body: PhysicalBody,
    pub lifetime: f32,
}
