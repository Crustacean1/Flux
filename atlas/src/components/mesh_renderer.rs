use glam::{Mat4, Vec3};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::{
        asteroid::AsteroidEntity, enemy_ship::EnemyShip, player_ship::PlayerShip,
        starlight::Starlight,
    },
    graphics::{
        context::Context,
        lights::{Light, LightColor},
        material::phong_material::PhongMaterial,
        model::Model,
        shaders::mesh_shader::MeshShaderDefinition,
    },
};

use super::{camera::Camera, transform::Transform};

pub struct MeshRenderer {
    pub mesh: Model,
    pub material: PhongMaterial,
}

pub struct MeshRendererSystem {
    shader: MeshShaderDefinition,
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
    pub fn new(shader: MeshShaderDefinition) -> Self {
        MeshRendererSystem { shader }
    }

    pub fn render(
        &self,
        context: &mut Context,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        let (projection, view) = camera.projection_view(&camera_transform);

        let lights = Self::get_lights(entity_manager, view);

        context.use_shader(&self.shader, |context| {
            context.shader.directional_lights(&view, &lights);

            entity_manager
                .get_view()
                .for_each(|(transform, model): (&Transform, &Model)| {
                    let view_model = view * transform.model();
                    let projection_view_model = projection * view_model;
                    context.shader.projection_view_model(&projection_view_model);
                    context.shader.view_model(&view_model);

                    model.meshes.iter().for_each(|(material, mesh)| {
                        context.use_material(material, |_| {
                            mesh.render();
                        });
                    });
                });
        })
    }

    fn get_lights(entity_manager: &EntityManager, _view: Mat4) -> Vec<(Vec3, LightColor)> {
        entity_manager
            .get_view()
            .map(|(_transform, &light): (&Transform, &Light)| match light {
                Light::PointLight(_) => todo!(),
                Light::DirectionalLight(direction, color) => (direction, color),
            })
            .collect()
    }
}
