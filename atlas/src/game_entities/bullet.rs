use crate::components::{
    collider::Collider, particle_emitter::ParticleEmitter, physical_body::PhysicalBody,
};

pub struct BulletEntity {
    pub collider: Collider,
    pub body: PhysicalBody,
    pub explosion_effect: Option<ParticleEmitter>,
    pub lifetime: f32,
}
