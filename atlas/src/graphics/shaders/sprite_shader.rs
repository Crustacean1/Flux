use glam::Mat4;

use crate::game_root::GameError;

use super::{locate_uniform, Shader, ShaderDefinition, UniformLoader};

#[derive(Clone, Copy, Default)]
struct Uniform {
    projection_view_model: i32,
    sprite_texture: i32,
}

#[derive(Clone, Default)]
pub struct SpriteShaderDefinition {
    shader_id: u32,
    uniform: Uniform,
}

#[derive(Clone)]
pub struct SpriteShader {
    shader_id: u32,
    uniform: Uniform,
}

impl SpriteShader {
    pub fn projection_view(&self, mat: &Mat4) {
        self.load(self.uniform.projection_view_model, mat);
    }

    pub fn sprite(&self, sprite: i32) {
        self.load(self.uniform.sprite_texture, sprite);
    }
}

impl Shader for SpriteShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}

impl ShaderDefinition for SpriteShaderDefinition {
    fn create_shader(&self) -> SpriteShader {
        SpriteShader {
            shader_id: self.shader_id,
            uniform: self.uniform,
        }
    }

    type Shader = SpriteShader;

    const EXTENSION: &'static str = "sprite_shader";

    fn build(shader_id: u32) -> Result<Self, GameError> {
        let projection_view_model = locate_uniform(shader_id, "projection_view_model").ok_or(
            GameError::new("Failed to find uniform 'projection_view_model'"),
        )?;
        let sprite_texture = locate_uniform(shader_id, "sprite_texture")
            .ok_or(GameError::new("Failed to find uniform 'sprite_texture'"))?;

        let uniform = Uniform {
            projection_view_model,
            sprite_texture,
        };

        Ok(Self { shader_id, uniform })
    }
}

impl SpriteShaderDefinition {}
