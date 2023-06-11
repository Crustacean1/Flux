use crate::{
    components::{physical_body::PhysicalBody, transform::Transform},
    graphics::{
        material::phong_material::PhongMaterial,
        mesh::Mesh,
        shaders::mesh_shader::MeshShader,
        vertices::layouts::{PTNVertex, TriangleGeometry},
    },
};

pub struct EnemyShip {
    pub physical_body: PhysicalBody,
    pub mesh: Mesh<PTNVertex, TriangleGeometry, PhongMaterial>,
}
