use crate::{components::physical_body::PhysicalBody, graphics::mesh::Mesh};

pub struct Bullet {
    pub physical_body: PhysicalBody,
    pub mesh: Mesh,
}
