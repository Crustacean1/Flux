use glam::{Vec3, Vec4};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    graphics::{
        lights::Light,
        material::phong_material::PhongMaterial,
        mesh::Mesh,
        shaders::{mesh_shader::MeshShader, ShaderProgram},
    },
};

use super::{camera::Camera, transform::Transform};

pub struct MeshRenderer {
    pub mesh: Mesh<MeshShader, PhongMaterial>,
    pub material: PhongMaterial,
}

pub struct MeshRendererSystem {
    shader: ShaderProgram<MeshShader>,
}

type MeshBundle<'a> = ((usize, &'a Transform), &'a Mesh<MeshShader, PhongMaterial>);

type LightBundle<'a> = (&'a Transform, &'a Light);

impl MeshRendererSystem {
    pub fn new(shader: ShaderProgram<MeshShader>) -> Self {
        MeshRendererSystem { shader }
    }

    pub fn render(&mut self, entity_manager: &EntityManager, camera: &Camera) {
        let view = camera.view_mat();
        let projection_view = camera.projection_view_mat();

        self.shader.bind();

        self.setup_lights(entity_manager, camera);

        entity_manager
            .iter()
            .for_each(|((_, transform), mesh): MeshBundle| {
                let view_model = view * transform.model();
                let projection_view_model = projection_view * transform.model();

                self.shader
                    .bind_projection_view_model(&projection_view_model.to_cols_array());
                self.shader.bind_view_model(&view_model.to_cols_array());

                mesh.render(&self.shader);
            })
    }

    fn setup_lights(&mut self, entity_manager: &EntityManager, camera: &Camera) {
        let view = camera.view_mat();
        let mut directional_lights = 0;

        entity_manager
            .iter()
            .enumerate()
            .for_each(|(i, (transform, light)): (_, LightBundle)| match light {
                Light::PointLight(light_color) => {
                    self.shader
                        .bind_directional_light(i, &transform.position, light_color);
                }
                Light::DirectionalLight(dir, light_color) => {
                    let dir = view * Vec4::new(dir.x, dir.y, dir.z, 0.0);
                    let dir = Vec3::new(dir.x, dir.y, dir.z);
                    self.shader.bind_directional_light(i, &dir, light_color);
                    directional_lights += 1;
                }
            });
        self.shader.bind_directional_light_count(directional_lights);
    }
}
