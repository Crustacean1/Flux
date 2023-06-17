use super::material::phong_material::PhongMaterial;
use super::shaders::mesh_shader::MeshShader;
use super::vertices::indices::TriangleGeometry;
use super::vertices::layouts::{PTNVertex, PTVertex};
use super::{
    material::skybox_material::SkyboxMaterial, primitive::Primitive, shaders::ShaderProgram,
};

#[derive(Clone)]
pub struct Mesh {
    pub primitives: Vec<(PhongMaterial, Primitive<PTNVertex, TriangleGeometry>)>,
}

pub struct Skybox {
    skybox: Primitive<PTVertex, TriangleGeometry>,
    skybox_material: SkyboxMaterial,
}

impl Mesh {
    pub fn render(&self, shader: &ShaderProgram<MeshShader>) {
        self.primitives
            .iter()
            .for_each(|(material, primitive)| unsafe {
                primitive.render();
            });
    }
}

impl Default for Mesh {
    fn default() -> Self {
        let primitive = Primitive::<PTNVertex, TriangleGeometry>::new(&[], &[]);
        let mat = PhongMaterial::default();
        return Mesh {
            primitives: vec![(mat, primitive)],
        };
    }
}
