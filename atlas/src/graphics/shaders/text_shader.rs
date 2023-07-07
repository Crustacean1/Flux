use glad_gl::gl;
use glam::Mat4;

use crate::{
    game_root::GameError,
    graphics::{
        mesh::Mesh,
        vertices::{indices::TriangleGeometry, layouts::{PTVertex, P2TVertex}},
    },
};

use super::{
    build_shader, locate_uniform, try_locate_uniform, Shader, ShaderProgram, UniformLoader,
};

pub struct TextShaderPass<'a> {
    shader: &'a mut TextShader,
}

impl<'a> TextShaderPass<'a> {
    pub fn render(
        &mut self,
        mesh: &Mesh<P2TVertex, TriangleGeometry>,
        projection_view_model: &Mat4,
        atlas: i32,
    ) {
        self.shader.load(
            self.shader.projection_view_model_uniform,
            projection_view_model,
        );
        self.shader.load(self.shader.atlas_uniform, atlas);

        mesh.bind();
        mesh.render();
    }
}

#[derive(Clone)]
pub struct TextShader {
    shader_id: u32,
    projection_view_model_uniform: i32,
    atlas_uniform: i32,
}

impl Shader for TextShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}

impl TextShader {
    fn build(vertex: &str, fragment: &str) -> Result<TextShader, GameError> {
        let shader_id = build_shader(Some(vertex), None, Some(fragment))?;
        let projection_view_model_uniform = try_locate_uniform(shader_id, "projection_view_model")?;
        let atlas_uniform = try_locate_uniform(shader_id, "atlas")?;

        Ok(Self {
            shader_id,
            projection_view_model_uniform,
            atlas_uniform,
        })
    }

    pub fn new_pass(&mut self) -> TextShaderPass {
        TextShaderPass { shader: self }
    }
}
