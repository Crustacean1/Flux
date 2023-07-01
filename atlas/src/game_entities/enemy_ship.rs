use crate::{
    components::{
        collider::Collider, particle_emitter::ParticleEmitter, physical_body::PhysicalBody,
    },
    graphics::mesh::Mesh,
};

pub struct EnemyShip {
    pub physical_body: PhysicalBody,
    pub thruster: ParticleEmitter,
    pub collider: Collider,
    pub mesh: Mesh,
}
