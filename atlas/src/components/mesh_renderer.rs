use glam::{Vec3, Vec4};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::{enemy_ship::EnemyShip, player_ship::PlayerShip, starlight::Starlight},
    graphics::{
        lights::Light,
        material::phong_material::PhongMaterial,
        mesh::Mesh,
        shaders::{mesh_shader::MeshShader, ShaderProgram},
        vertices::layouts::{PTNVertex, TriangleGeometry},
    },
};

use super::{camera::Camera, transform::Transform};

pub struct MeshRenderer {
    pub mesh: Mesh<PTNVertex, TriangleGeometry, PhongMaterial>,
    pub material: PhongMaterial,
}

pub struct MeshRendererSystem {
    shader: ShaderProgram<MeshShader>,
}

impl<'a>
    ComponentIteratorGenerator<
        'a,
        (
            &'a Transform,
            &'a Mesh<PTNVertex, TriangleGeometry, PhongMaterial>,
        ),
    > for EntityManager
{
    fn get_view(
        &'a self,
    ) -> Box<
        dyn Iterator<
                Item = (
                    &Transform,
                    &Mesh<PTNVertex, TriangleGeometry, PhongMaterial>,
                ),
            > + 'a,
    > {
        let enemies = self
            .iter::<EnemyShip>()
            .map(|ship| (&ship.transform, &ship.entity.mesh));
        let players = self
            .iter::<PlayerShip>()
            .map(|ship| (&ship.transform, &ship.entity.mesh));

        Box::new(enemies.chain(players))
    }
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a Light)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&Transform, &Light)> + 'a> {
        let lights = self
            .iter::<Starlight>()
            .map(|ship| (&ship.transform, &ship.entity.light));

        Box::new(lights)
    }
}

impl MeshRendererSystem {
    pub fn new(shader: ShaderProgram<MeshShader>) -> Self {
        MeshRendererSystem { shader }
    }

    pub fn render(
        &self,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) -> Option<()> {
        let (projection, view) = camera.projection_view(&camera_transform);

        self.shader.bind();

        self.setup_lights(entity_manager, camera, camera_transform);

        entity_manager.get_view().for_each(
            |(transform, mesh): (&Transform, &Mesh<MeshShader, PhongMaterial>)| {
                let view_model = view * transform.model();
                let projection_view_model = projection * view * transform.model();

                self.shader
                    .bind_projection_view_model(&projection_view_model.to_cols_array());
                self.shader.bind_view_model(&view_model.to_cols_array());

                mesh.render(&self.shader);
            },
        );
        Some(())
    }

    fn setup_lights(
        &self,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        let (_, view) = camera.projection_view(camera_transform);
        let mut directional_lights = 0;

        entity_manager.get_view().enumerate().for_each(
            |(i, (transform, light)): (_, (&Transform, &Light))| match light {
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
            },
        );
        self.shader.bind_directional_light_count(directional_lights);
    }
}
