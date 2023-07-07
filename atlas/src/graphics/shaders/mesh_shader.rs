use glam::{Mat4, Vec3};

use crate::{
    game_root::GameError,
    graphics::{
        lights::LightColor,
        mesh::Mesh,
        vertices::{indices::TriangleGeometry, layouts::PTNVertex},
    },
};

use super::{
    build_shader, locate_uniform, try_locate_uniform, Shader, ShaderProgram, UniformLoader,
};

pub struct MeshShaderPass<'a> {
    shader: &'a mut MeshShader,
}

impl<'a> MeshShaderPass<'a> {
    pub fn render(
        &self,
        model_view: &Mat4,
        projection_model_view: &Mat4,
        mesh: &Mesh<PTNVertex, TriangleGeometry>,
    ) {
        self.shader.load(self.shader.view_model_uniform, model_view);
        self.shader.load(
            self.shader.projection_view_model_uniform,
            projection_model_view,
        );
        mesh.bind();
        mesh.render();
    }
}

#[derive(Clone)]
pub struct MeshShader {
    shader_id: u32,
    projection_view_model_uniform: i32,
    view_model_uniform: i32,
    directional_light_count_uniform: i32,
    directional_light_uniforms: [DirectionalLightUniform; 4],
    material_uniform: MaterialUniform,
}

#[derive(Clone, Copy, Debug)]
struct DirectionalLightUniform {
    direction: i32,
    ambient: i32,
    diffuse: i32,
    specular: i32,
}

#[derive(Clone, Copy, Debug)]
struct MaterialUniform {
    diffuse: i32,
}

impl Shader for MeshShader {
    fn shader_id(&self) -> u32 {
        self.shader_id
    }
}

impl MeshShader {
    fn build(vertex: &str, geometry: &str, fragment: &str) -> Result<MeshShader, GameError> {
        let shader_id = build_shader(Some(vertex), None, Some(fragment))?;

        let projection_view_model_uniform = locate_uniform(shader_id, "projection_view_model")
            .ok_or(GameError::uniform("projection_view_model"))?;

        let view_model_uniform = try_locate_uniform(shader_id, "view_model")?;

        let directional_light_count_uniform =
            try_locate_uniform(shader_id, "directional_light_count")?;

        let directional_light_uniforms: [DirectionalLightUniform; 4] = (0..5)
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

        let material_uniform = MaterialUniform {
            diffuse: locate_uniform(shader_id, &format!("material.diffuse"))
                .ok_or(GameError::uniform("material.diffuse"))?,
        };

        Ok(MeshShader {
            shader_id,
            directional_light_uniforms,
            material_uniform,
            directional_light_count_uniform,
            projection_view_model_uniform,
            view_model_uniform,
        })
    }

    pub fn new_pass(&mut self, lights: &[(Vec3, LightColor)]) -> MeshShaderPass {
        self.bind();

        lights
            .iter()
            .enumerate()
            .for_each(|(i, (direction, color))| {
                self.load(self.directional_light_uniforms[i].ambient, color.ambient);
                self.load(self.directional_light_uniforms[i].diffuse, color.diffuse);
            });

        MeshShaderPass { shader: self }
    }
}
