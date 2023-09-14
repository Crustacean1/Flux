use crate::graphics::{
    mesh::Mesh,
    vertices::{health_bar, indices::TriangleGeometry, layouts::P2TVertex},
};

pub struct HealthRenderer {
    pub mesh: Mesh<P2TVertex, TriangleGeometry>,
    pub health: f32,
    pub enabled: bool,
}

impl HealthRenderer {
    pub fn health_bar(health: f32) -> Self {
        let (vertices, indices) = health_bar::health_bar();
        let mesh = Mesh::new(&vertices, &indices);
        Self {
            mesh,
            health,
            enabled: false,
        }
    }
}
