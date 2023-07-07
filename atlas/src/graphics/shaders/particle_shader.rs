use glam::Mat4;

use crate::graphics::{
    instanced_mesh::InstancedMesh,
    vertices::{
        indices::TriangleGeometry,
        layouts::{BufferElement, P2TVertex, PTVertex},
    },
};

use super::{
    build_shader, text_shader::TextShaderPass, try_locate_uniform, Shader, ShaderProgram,
    UniformLoader,
};

#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct ParticleInstance {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub transform: [f32; 4],
}

pub struct ParticleShaderPass<'a> {
    shader: &'a mut ParticleShader,
}

impl<'a> ParticleShaderPass<'a> {
    pub fn render(&self, mesh: &InstancedMesh<ParticleInstance, P2TVertex, TriangleGeometry>) {
        mesh.render();
    }
}

#[derive(Clone)]
pub struct ParticleShader {
    shader_id: u32,
    projection_uniform: i32,
    view_uniform: i32,
}

impl Shader for ParticleShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}

impl ParticleShader {
    fn build(vertex: &str, fragment: &str) -> Result<ParticleShader, crate::game_root::GameError> {
        let shader_id = build_shader(Some(vertex), None, Some(fragment))?;

        let view_uniform = try_locate_uniform(shader_id, "view")?;
        let projection_uniform = try_locate_uniform(shader_id, "projection")?;

        Ok(Self {
            shader_id,
            projection_uniform,
            view_uniform,
        })
    }

    pub fn new_pass(&mut self, projection: &Mat4, view: &Mat4) -> ParticleShaderPass {
        self.bind();
        self.load(self.projection_uniform, projection);
        self.load(self.view_uniform, view);
        ParticleShaderPass { shader: self }
    }
}
