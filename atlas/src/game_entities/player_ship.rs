use crate::{
    components::{camera::Camera, particle_emitter::ParticleEmitter, physical_body::PhysicalBody},
    graphics::mesh::Mesh,
};

pub struct PlayerShip {
    pub camera: Camera,
    pub physical_body: PhysicalBody,
    pub thruster: ParticleEmitter,
    pub mesh: Mesh,
}
