use crate::{
    components::{
        collider::Collider, particle_emitter::ParticleEmitter, physical_body::PhysicalBody,
        unit::Unit,
    },
    graphics::model::Model,
};

pub struct EnemyShip {
    pub physical_body: PhysicalBody,
    pub thruster: ParticleEmitter,
    pub collider: Collider,
    pub mesh: Model,
    pub info: Unit,
}
