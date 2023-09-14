use glam::Mat4;

use crate::game_root::GameError;

use super::{locate_uniform, Shader, ShaderDefinition, UniformLoader};

#[derive(Clone, Copy, Default)]
struct Uniform {
    projection_view_model: i32,
    health: i32,
}

#[derive(Clone, Default)]
pub struct HealthShaderDefinition {
    shader_id: u32,
    uniform: Uniform,
}

#[derive(Clone)]
pub struct HealthShader {
    shader_id: u32,
    uniform: Uniform,
}

impl Shader for HealthShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}

impl HealthShader {
    pub fn projection_view_model(&self, projection_view_model: &Mat4) {
        self.load(self.uniform.projection_view_model, projection_view_model);
    }
    pub fn health(&self, health: f32) {
        self.load(self.uniform.health, health);
    }
}

impl ShaderDefinition for HealthShaderDefinition {
    fn create_shader(&self) -> HealthShader {
        HealthShader {
            shader_id: self.shader_id,
            uniform: self.uniform,
        }
    }

    type Shader = HealthShader;

    const EXTENSION: &'static str = "health_shader";

    fn build(shader_id: u32) -> Result<Self, GameError> {
        let projection_view_model = locate_uniform(shader_id, "projection_view_model").ok_or(
            GameError::new("Failed to find uniform 'projection_view_model'"),
        )?;
        let health = locate_uniform(shader_id, "health")
            .ok_or(GameError::new("Failed to find uniform 'health'"))?;

        let uniform = Uniform {
            projection_view_model,
            health,
        };

        Ok(Self { shader_id, uniform })
    }
}
