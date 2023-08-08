use glam::Mat4;

use crate::game_root::GameError;

use super::{locate_uniform, try_locate_uniform, Shader, ShaderDefinition, UniformLoader};

#[derive(Clone, Copy, Default)]
struct Uniform {
    billboards: [i32; 6],
    projection: i32,
    view: i32,
}

#[derive(Clone, Default)]
pub struct SkyboxShaderDefinition {
    shader_id: u32,
    uniform: Uniform,
}

impl ShaderDefinition for SkyboxShaderDefinition {
    fn create_shader(&self) -> SkyboxShader {
        SkyboxShader {
            shader_id: self.shader_id,
            uniform: self.uniform,
        }
    }

    type Shader = SkyboxShader;

    const EXTENSION: &'static str = "sky_shader";

    fn build(shader_id: u32) -> Result<Self, GameError> {
        let billboards: Vec<_> = (0..6)
            .filter_map(|i| locate_uniform(shader_id, &format!("billboards[{}]", i)))
            .collect();
        let billboards: [i32; 6] = billboards
            .try_into()
            .map_err(|_| GameError::uniform("billboards"))?;

        let projection = try_locate_uniform(shader_id, "projection")?;
        let view = try_locate_uniform(shader_id, "view")?;

        let uniform = Uniform {
            billboards,
            projection,
            view,
        };

        Ok(Self { shader_id, uniform })
    }
}

#[derive(Clone)]
pub struct SkyboxShader {
    shader_id: u32,
    uniform: Uniform,
}

impl SkyboxShader {
    pub fn projection(&self, mat: &Mat4) {
        self.load(self.uniform.projection, mat);
    }

    pub fn view(&self, mat: &Mat4) {
        self.load(self.uniform.view, mat);
    }

    pub fn billboards(&self, billboards: [i32; 6]) {
        billboards
            .iter()
            .enumerate()
            .for_each(|(i, &billboard)| self.load(self.uniform.billboards[i], billboard));
    }
}

impl Shader for SkyboxShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}
