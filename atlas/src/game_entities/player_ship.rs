use crate::{
    components::{camera::Camera, physical_body::PhysicalBody},
    graphics::{
        material::phong_material::PhongMaterial,
        mesh::Mesh,
        vertices::layouts::{PTNVertex, TriangleGeometry},
    },
};

pub struct PlayerShip {
    pub camera: Camera,
    pub physical_body: PhysicalBody,
    pub mesh: Mesh<PTNVertex, TriangleGeometry, PhongMaterial>,
}
