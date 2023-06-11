use glam::Vec3;

use crate::graphics::mesh::Mesh;

pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub size: f32,
    pub lifetime: f32,
}
