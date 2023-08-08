use glam::Mat4;

use crate::game_root::GameError;

use super::{try_locate_uniform, Shader, ShaderDefinition, UniformLoader};

#[derive(Clone, Copy, Default)]
struct Uniform {
    projection: i32,
    view: i32,
    texture: i32,
}

pub struct BulletShader {
    shader_id: u32,
    uniform: Uniform,
}

impl BulletShader {
    pub fn projection(&self, mat: &Mat4) {
        self.load(self.uniform.projection, mat);
    }

    pub fn view(&self, mat: &Mat4) {
        self.load(self.uniform.view, mat);
    }

    pub fn texture(&self, tex: i32) {
        self.load(self.uniform.texture, tex);
    }
}

#[derive(Clone, Default)]
pub struct BulletShaderDefinition {
    shader_id: u32,
    uniform: Uniform,
}

impl ShaderDefinition for BulletShaderDefinition {
    fn create_shader(&self) -> BulletShader {
        BulletShader {
            shader_id: self.shader_id,
            uniform: self.uniform,
        }
    }

    type Shader = BulletShader;

    const EXTENSION: &'static str = "bullet_shader";

    fn build(shader_id: u32) -> Result<Self, GameError> {
        let projection = try_locate_uniform(shader_id, "projection")?;
        let view = try_locate_uniform(shader_id, "view")?;
        let texture = try_locate_uniform(shader_id, "sprite")?;

        let uniform = Uniform {
            projection,
            view,
            texture,
        };

        Ok(Self { shader_id, uniform })
    }
}

impl Shader for BulletShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}
