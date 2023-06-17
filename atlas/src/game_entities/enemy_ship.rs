use crate::{components::{physical_body::PhysicalBody, particle_emitter::ParticleEmitter}, graphics::mesh::Mesh};

pub struct EnemyShip {
    pub physical_body: PhysicalBody,
    pub thruster: ParticleEmitter,
    pub mesh: Mesh,
}
