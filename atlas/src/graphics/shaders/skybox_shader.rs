use glam::Mat4;

use crate::{
    game_root::GameError,
    graphics::{
        instanced_mesh::InstancedMesh,
        vertices::{
            indices::TriangleGeometry,
            layouts::{Attribute, BufferElement, PTVertex},
            skybox::SkyboxInstance,
        },
    },
};

use super::{
    build_shader, locate_uniform, try_locate_uniform, Shader, ShaderProgram, UniformLoader,
};

pub struct SkyboxShaderPass<'a> {
    shader: &'a mut SkyboxShader,
}

impl<'a> SkyboxShaderPass<'a> {
    pub fn render(&self, mesh: &InstancedMesh<SkyboxInstance, PTVertex, TriangleGeometry>) {
        mesh.render();
    }
}

#[derive(Clone)]
pub struct SkyboxShader {
    shader_id: u32,
    billboard_uniforms: [i32; 6],
    projection_uniform: i32,
    view_uniform: i32,
}

impl Shader for SkyboxShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}

impl SkyboxShader {
    fn build(vertex: &str, fragment: &str) -> Result<SkyboxShader, GameError> {
        let shader_id = build_shader(Some(vertex), None, Some(fragment))?;
        let billboard_uniforms: Vec<_> = (0..6)
            .filter_map(|i| locate_uniform(shader_id, &format!("billboards[{}]", i)))
            .collect();
        let projection_view_uniform = try_locate_uniform(shader_id, "projection")?;
        let view_uniform = try_locate_uniform(shader_id, "view")?;

        Ok(Self {
            shader_id,
            billboard_uniforms: billboard_uniforms
                .try_into()
                .map_err(|e| GameError::new("Billboard uniforms not found in skybox shader"))?,
            projection_uniform: projection_view_uniform,
            view_uniform,
        })
    }

    pub fn new_pass(&mut self, projection: &Mat4, view: &Mat4) -> SkyboxShaderPass {
        self.bind();
        self.load(self.projection_uniform, projection);
        self.load(self.view_uniform, view);
        SkyboxShaderPass { shader: self }
    }
}
