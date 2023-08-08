use glam::Mat4;

use crate::game_root::GameError;

use super::{try_locate_uniform, Shader, ShaderDefinition, UniformLoader};

#[derive(Clone, Copy, Default)]
struct Uniform {
    projection_view: i32,
}

#[derive(Clone, Default)]
pub struct FlatShaderDefinition {
    shader_id: u32,
    uniform: Uniform,
}

impl ShaderDefinition for FlatShaderDefinition {
    fn create_shader(&self) -> FlatShader {
        FlatShader {
            shader_id: self.shader_id,
            uniform: self.uniform,
        }
    }

    type Shader = FlatShader;

    const EXTENSION: &'static str = "flat_shader";

    fn build(shader_id: u32) -> Result<Self, GameError> {
        let projection_view = try_locate_uniform(shader_id, "projection_view")?;

        let uniform = Uniform { projection_view };

        Ok(Self { shader_id, uniform })
    }
}

#[derive(Clone, Default)]
pub struct FlatShader {
    shader_id: u32,
    uniform: Uniform,
}

impl FlatShader {}

impl Shader for FlatShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}

impl FlatShader {
    pub fn projection_view(&self, mat: &Mat4) {
        self.load(self.uniform.projection_view, mat)
    }
}
