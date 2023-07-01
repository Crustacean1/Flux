use crate::{
    components::{
        camera::Camera, collider::Collider, particle_emitter::ParticleEmitter,
        physical_body::PhysicalBody,
    },
    graphics::mesh::Mesh,
};

pub struct PlayerShip {
    pub camera: Camera,
    pub physical_body: PhysicalBody,
    pub thruster: ParticleEmitter,
    pub collider: Collider,
    pub mesh: Mesh,
}
