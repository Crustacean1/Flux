use crate::{graphics::mesh::Mesh, components::physical_body::PhysicalBody};

pub struct EnemyShip {
    pub physical_body: PhysicalBody,
    pub mesh: Mesh,
}
