use super::material::phong_material::PhongMaterial;
use super::vertices::indices::TriangleGeometry;
use super::vertices::layouts::{PTNVertex, PTVertex};
use super::{material::skybox_material::SkyboxMaterial, mesh::Mesh};

#[derive(Clone)]
pub struct Model {
    pub meshes: Vec<(PhongMaterial, Mesh<PTNVertex, TriangleGeometry>)>,
}

pub struct Skybox {
    skybox: Mesh<PTVertex, TriangleGeometry>,
    skybox_material: SkyboxMaterial,
}

impl Default for Model {
    fn default() -> Self {
        let primitive = Mesh::<PTNVertex, TriangleGeometry>::new(&[], &[]);
        let mat = PhongMaterial::default();
        return Model {
            meshes: vec![(mat, primitive)],
        };
    }
}
