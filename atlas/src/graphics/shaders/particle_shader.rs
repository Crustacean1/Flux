use glam::Mat4;

use crate::game_root::GameError;

use super::{try_locate_uniform, Shader, ShaderDefinition, UniformLoader};

#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct ParticleInstance {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub transform: [f32; 4],
}

#[derive(Clone, Copy, Default)]
struct Uniform {
    projection: i32,
    view: i32,
}

#[derive(Clone, Default)]
pub struct ParticleShaderDefinition {
    shader_id: u32,
    uniform: Uniform,
}

impl ShaderDefinition for ParticleShaderDefinition {
    fn create_shader(&self) -> ParticleShader {
        ParticleShader {
            shader_id: self.shader_id,
            uniform: self.uniform,
        }
    }

    type Shader = ParticleShader;

    const EXTENSION: &'static str = "particle_shader";

    fn build(shader_id: u32) -> Result<Self, GameError> {
        let view = try_locate_uniform(shader_id, "view")?;
        let projection = try_locate_uniform(shader_id, "projection")?;

        let uniform = Uniform { view, projection };

        Ok(Self { shader_id, uniform })
    }
}

#[derive(Clone)]
pub struct ParticleShader {
    shader_id: u32,
    uniform: Uniform,
}

impl ParticleShader {
    pub fn projection(&self, mat: &Mat4) {
        self.load(self.uniform.projection, mat);
    }

    pub fn view(&self, mat: &Mat4) {
        self.load(self.uniform.view, mat);
    }
}

impl Shader for ParticleShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}
