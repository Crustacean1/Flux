use glam::Mat4;

use crate::{
    game_root::GameError,
    graphics::{
        mesh::Mesh,
        vertices::{indices::LineGeometry, layouts::PVertex},
    },
};

use super::{build_shader, locate_uniform, Shader, ShaderProgram, UniformLoader};

pub struct FlatShaderPass<'a> {
    shader: &'a mut FlatShader,
}

impl<'a> FlatShaderPass<'a> {
    pub fn render(mesh: Mesh<PVertex, LineGeometry>) {
        mesh.bind();
        mesh.render();
    }
}

#[derive(Clone)]
pub struct FlatShader {
    shader_id: u32,
    projection_view_uniform: i32,
}

impl Shader for FlatShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}

impl FlatShader {
    fn build(vertex: &str, fragment: &str) -> Result<FlatShader, crate::game_root::GameError> {
        let shader_id = build_shader(Some(vertex), None, Some(fragment))?;
        let projection_view_model_uniform = locate_uniform(shader_id, "projection_view")
            .ok_or(GameError::uniform("projection_view"))?;
        Ok(Self {
            shader_id,
            projection_view_uniform: projection_view_model_uniform,
        })
    }

    fn new_pass(&mut self, projection_view: &Mat4) -> FlatShaderPass {
        self.bind();
        self.load(self.projection_view_uniform, projection_view);
        FlatShaderPass { shader: self }
    }
}
