use glad_gl::gl;
use glam::Mat4;

use crate::{
    game_root::GameError,
    graphics::{
        instanced_mesh::InstancedMesh,
        mesh::Mesh,
        vertices::{
            indices::PointGeometry,
            layouts::{P2TVertex, PVertex},
        },
    },
    systems::bullet_renderer::BulletInstance,
};

use super::{
    build_shader, locate_uniform, try_locate_uniform, Shader, ShaderProgram, UniformLoader,
};

#[derive(Clone)]
pub struct BulletShader {
    shader_id: u32,

    projection_uniform: i32,
    view_uniform: i32,
    texture_uniform: i32,
}

pub struct BulletShaderPass<'a> {
    shader: &'a mut BulletShader,
}

impl<'a> BulletShaderPass<'a> {
    pub fn render(&self, mesh: &InstancedMesh<BulletInstance, PVertex, PointGeometry>) {
        mesh.render();
    }
}

impl Shader for BulletShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}

impl BulletShader {
    fn build(
        vertex: &str,
        geometry: &str,
        fragment: &str,
    ) -> Result<BulletShader, crate::game_root::GameError> {
        let shader_id = build_shader(Some(vertex), Some(geometry), Some(fragment))?;

        let projection_uniform = try_locate_uniform(shader_id, "projection")?;
        let view_uniform = try_locate_uniform(shader_id, "view")?;
        let texture_uniform = try_locate_uniform(shader_id, "sprite")?;

        Ok(Self {
            shader_id,
            projection_uniform,
            texture_uniform,
            view_uniform,
        })
    }

    pub fn new_pass(&mut self, projection: &Mat4, view: &Mat4) -> BulletShaderPass {
        self.bind();
        self.load(self.projection_uniform, projection);
        self.load(self.view_uniform, view);

        BulletShaderPass { shader: self }
    }
}
