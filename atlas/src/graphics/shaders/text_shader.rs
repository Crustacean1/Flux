use glam::Mat4;

use crate::game_root::GameError;

use super::{try_locate_uniform, Shader, ShaderDefinition, UniformLoader};

#[derive(Clone, Copy, Default)]
struct Uniform {
    projection_view_model: i32,
    atlas: i32,
}

#[derive(Clone)]
pub struct TextShader {
    shader_id: u32,
    uniform: Uniform,
}

#[derive(Clone, Default)]
pub struct TextShaderDefinition {
    shader_id: u32,
    uniform: Uniform,
}

impl ShaderDefinition for TextShaderDefinition {
    type Shader = TextShader;

    const EXTENSION: &'static str = "text_shader";

    fn create_shader(&self) -> TextShader {
        TextShader {
            shader_id: self.shader_id,
            uniform: self.uniform,
        }
    }

    fn build(shader_id: u32) -> Result<Self, GameError> {
        let projection_view_model = try_locate_uniform(shader_id, "projection_view_model")?;
        let atlas = try_locate_uniform(shader_id, "atlas")?;

        let uniform = Uniform {
            projection_view_model,
            atlas,
        };

        Ok(Self { shader_id, uniform })
    }
}

impl TextShader {
    pub fn atlas(&self, atlas: i32) {
        self.load(self.uniform.atlas, atlas);
    }

    pub fn projection_model(&self, mat: &Mat4) {
        self.load(self.uniform.projection_view_model, mat);
    }
}

impl Shader for TextShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}
