use glam::{Vec3, Vec4};

use crate::{
    entity_manager::{self, ComponentIteratorGenerator, EntityManager},
    graphics::{
        lights::Light,
        material::TextureMaterial,
        mesh::Mesh,
        shaders::{MeshShader, ShaderProgram},
    },
};

use super::{camera::Camera, transform::Transform};

pub struct MeshRenderer {
    pub mesh: Mesh<MeshShader, TextureMaterial>,
    pub material: TextureMaterial,
}

pub struct MeshRendererSystem {
    shader: ShaderProgram<MeshShader>,
}

type MeshBundle<'a> = (
    (usize, &'a Transform),
    &'a Mesh<MeshShader, TextureMaterial>,
);

type LightBundle<'a> = (&'a Transform, &'a Light);

impl MeshRendererSystem {
    pub fn new(shader: ShaderProgram<MeshShader>) -> Self {
        MeshRendererSystem { shader }
    }

    pub fn render(&mut self, camera: &Camera, entity_manager: &EntityManager) {
        let projection_view = camera.projection_view_mat();
        let view = camera.view_mat();

        self.shader.reset_directional_lights();
        entity_manager
            .iter()
            .for_each(|(transform, light): LightBundle| match light {
                Light::PointLight(light_color) => {
                    self.shader
                        .add_point_light(&transform.position, light_color);
                }
                Light::DirectionalLight(dir, light_color) => {
                    let dir = view * Vec4::new(dir.x, dir.y, dir.z, 0.0);
                    let dir = Vec3::new(dir.x, dir.y, dir.z);
                    self.shader.add_directional_light(&dir, light_color);
                }
            });

        entity_manager
            .iter()
            .for_each(|((_, transform), mesh): MeshBundle| {
                let view_model = view * transform.model();
                let projection_view_model = projection_view * transform.model();

                self.shader
                    .load_projection_view_model(&projection_view_model.to_cols_array());
                self.shader.load_view_model(&view_model.to_cols_array());
                self.shader.load_view(&view.to_cols_array());

                mesh.render(&self.shader);
            })
    }
}
