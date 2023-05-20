use crate::graphics::{
    material::TextureMaterial,
    mesh::Mesh,
    shaders::{ShaderProgram, SkyboxShader},
    vertices::base_vertices::{TriangleIndex, Vertex3PT},
};

pub struct SkyboxRenderer {
    texture: TextureMaterial,
    mesh: Mesh<Vertex3PT, TriangleIndex>,
}

pub struct SkyboxSystem {
    shader: ShaderProgram<SkyboxShader>,
}

impl SkyboxSystem {
    pub fn render(&self) {}

    pub fn new(shader: ShaderProgram<SkyboxShader>) -> Self {
        SkyboxSystem { shader }
    }
}
