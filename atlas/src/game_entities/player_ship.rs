use crate::{
    components::{camera::Camera, physical_body::PhysicalBody},
    graphics::mesh::Mesh,
};

pub struct PlayerShip {
    pub camera: Camera,
    pub physical_body: PhysicalBody,
    pub mesh: Mesh,
}
