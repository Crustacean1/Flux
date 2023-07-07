use glad_gl::gl;
use glam::Mat4;

use crate::{
    game_root::GameError,
    graphics::{
        mesh::Mesh,
        vertices::{indices::TriangleGeometry, layouts::P2TVertex},
    },
};

use super::{build_shader, locate_uniform, Shader, UniformLoader};

#[derive(Clone)]
pub struct SpriteShader {
    shader_id: u32,
    projection_view_model_uniform: i32,
    sprite_texuture_uniform: i32,
}

pub struct SpriteShaderPass<'a> {
    shader: &'a mut SpriteShader,
}

impl<'a> SpriteShaderPass<'a> {
    pub fn render(&self, mesh: Mesh<P2TVertex, TriangleGeometry>, model_view_projection: &Mat4) {
        self.shader.load(
            self.shader.projection_view_model_uniform,
            model_view_projection,
        );
        mesh.bind();
        mesh.render();
    }
}

impl Shader for SpriteShader {
    type ShaderPass = SpriteShaderPass<'a>;
    fn shader_id(&self) -> u32 {
        self.shader_id
    }

    fn new_pass(&self) -> Self::ShaderPass {
        todo!()
    }
}

impl SpriteShader {
    fn build(vertex: &str, fragment: &str) -> Result<SpriteShader, GameError> {
        let shader_id = build_shader(Some(vertex), None, Some(fragment))?;

        let projection_view_model_uniform = locate_uniform(shader_id, "projection_view_model")
            .ok_or(GameError::new(
                "Failed to find uniform 'projection_view_model'",
            ))?;
        let sprite_texuture_uniform = locate_uniform(shader_id, "sprite_texture")
            .ok_or(GameError::new("Failed to find uniform 'sprite_texture'"))?;

        Ok(Self {
            shader_id,
            projection_view_model_uniform,
            sprite_texuture_uniform,
        })
    }

    pub fn new_pass(&mut self) -> SpriteShaderPass {
        self.bind();

        SpriteShaderPass { shader: self }
    }
}
