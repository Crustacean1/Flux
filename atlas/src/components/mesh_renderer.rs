use glam::{Vec3, Vec4};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::{
        asteroid::AsteroidEntity, enemy_ship::EnemyShip, player_ship::PlayerShip,
        starlight::Starlight,
    },
    graphics::{
        lights::{Light, LightColor},
        material::{phong_material::PhongMaterial, Material},
        model::Model,
        shaders::{mesh_shader::MeshShader, ShaderProgram},
    },
};

use super::{camera::Camera, transform::Transform};

pub struct MeshRenderer {
    pub mesh: Model,
    pub material: PhongMaterial,
}

pub struct MeshRendererSystem {
    shader: MeshShader,
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a Model)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&Transform, &Model)> + 'a> {
        let enemies = self
            .iter::<EnemyShip>()
            .map(|ship| (&ship.transform, &ship.entity.mesh));
        let players = self
            .iter::<PlayerShip>()
            .map(|ship| (&ship.transform, &ship.entity.mesh));
        let asteroids = self
            .iter::<AsteroidEntity>()
            .map(|asteroid| (&asteroid.transform, &asteroid.entity.mesh));

        Box::new(enemies.chain(players).chain(asteroids))
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
    pub fn new(shader: MeshShader) -> Self {
        MeshRendererSystem { shader }
    }

    pub fn render(
        &self,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) -> Option<()> {
        let (projection, view) = camera.projection_view(&camera_transform);

        let lights = Self::get_lights(entity_manager);
        let pass = self.shader.new_pass(&lights);

        entity_manager
            .get_view()
            .for_each(|(transform, model): (&Transform, &Model)| {
                let view_model = view * transform.model();
                let projection_view_model = projection * view * transform.model();

                model.meshes.iter().for_each(|(material, mesh)| {
                    material.bind();
                    pass.render(&view_model, &projection_view_model, mesh);
                });
            });
        Some(())
    }

    fn get_lights(entity_manager: &EntityManager) -> Vec<(Vec3, LightColor)> {
        entity_manager
            .get_view()
            .map(|(transform, &light): (&Transform, &Light)| match light {
                Light::PointLight(_) => todo!(),
                Light::DirectionalLight(direction, color) => (direction, color),
            })
            .collect()
    }

    /*fn setup_lights(
        &self,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        let (_, view) = camera.projection_view(camera_transform);
        let mut directional_lights = 0;

        let pass = self.shader.new_pass(self.)

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
    }*/
}
