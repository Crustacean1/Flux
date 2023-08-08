use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

use crate::{
    game_root::GameError,
    graphics::{
        lights::LightColor, material::phong_material::PhongMaterial, shaders::UniformLoader,
    },
};

use super::{locate_uniform, try_locate_uniform, Shader, ShaderDefinition};

#[derive(Clone, Copy, Default)]
struct Uniform {
    projection_view_model: i32,
    view_model: i32,
    directional_light_count: i32,
    directional_lights: [DirectionalLightUniform; 4],
    material: i32,
}

#[derive(Clone, Default)]
pub struct MeshShaderDefinition {
    shader_id: u32,
    uniform: Uniform,
}

impl ShaderDefinition for MeshShaderDefinition {
    fn create_shader(&self) -> MeshShader {
        MeshShader {
            shader_id: self.shader_id,
            uniform: self.uniform,
        }
    }

    type Shader = MeshShader;

    const EXTENSION: &'static str = "mesh_shader";

    fn build(shader_id: u32) -> Result<Self, GameError> {
        let projection_view_model = try_locate_uniform(shader_id, "projection_view_model")?;

        let view_model = try_locate_uniform(shader_id, "view_model")?;

        let directional_light_count = try_locate_uniform(shader_id, "directional_light_count")?;

        let directional_lights: [DirectionalLightUniform; 4] = (0..5)
            .filter_map(|i| {
                let instance = format!("directional_lights[{}]", i);

                let direction = locate_uniform(shader_id, &format!("{}.direction", instance))?;
                let ambient = locate_uniform(shader_id, &format!("{}.ambient", instance))?;
                let diffuse = locate_uniform(shader_id, &format!("{}.diffuse", instance))?;
                let specular = locate_uniform(shader_id, &format!("{}.specular", instance))?;

                Some(DirectionalLightUniform {
                    direction,
                    ambient,
                    diffuse,
                    specular,
                })
            })
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| GameError::new("Failed to find all light uniforms"))?;

        let material = try_locate_uniform(shader_id, &format!("material.diffuse"))?;

        let uniform = Uniform {
            projection_view_model,
            view_model,
            directional_light_count,
            directional_lights,
            material,
        };

        Ok(MeshShaderDefinition { shader_id, uniform })
    }
}

#[derive(Clone)]
pub struct MeshShader {
    shader_id: u32,
    uniform: Uniform,
}

#[derive(Clone, Copy, Debug, Default)]
struct DirectionalLightUniform {
    direction: i32,
    ambient: i32,
    diffuse: i32,
    specular: i32,
}

impl Shader for MeshShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}

impl MeshShader {
    pub fn projection_view_model(&self, mat: &Mat4) {
        self.load(self.uniform.projection_view_model, mat);
    }

    pub fn view_model(&self, mat: &Mat4) {
        self.load(self.uniform.view_model, mat);
    }

    pub fn directional_lights(&self, view: &Mat4, lights: &[(Vec3, LightColor)]) {
        lights.iter().enumerate().for_each(|(i, (dir, light))| {
            self.load(
                self.uniform.directional_lights[i].direction,
                (*view * Vec4::from((*dir, 0.0))).xyz().normalize(),
            );
            self.load(
                self.uniform.directional_lights[i].ambient,
                Vec3::new(0.0, 0.0, 0.0),
            );
            self.load(self.uniform.directional_lights[i].diffuse, light.diffuse);
            self.load(self.uniform.directional_lights[i].specular, light.specular);
        });

        self.load(self.uniform.directional_light_count, 1);
    }

    pub fn material(&self, material: &PhongMaterial) {}
}
