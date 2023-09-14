use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::{enemy_ship::EnemyShip, explosion::Explosion, player_ship::PlayerShip},
    graphics::{context::Context, shaders::particle_shader::ParticleShaderDefinition},
};

use super::{camera::Camera, particle_emitter::ParticleEmitter, transform::Transform};

pub struct ParticleRenderer {
    shader: ParticleShaderDefinition,
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a ParticleEmitter)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a ParticleEmitter)> + 'a> {
        let players = self
            .iter::<PlayerShip>()
            .map(|player| (&player.transform, &player.entity.thruster));
        let enemies = self
            .iter::<EnemyShip>()
            .map(|enemy| (&enemy.transform, &enemy.entity.thruster));
        let explosions = self
            .iter::<Explosion>()
            .map(|explosion| (&explosion.transform, &explosion.entity.explosion));

        Box::new(players.chain(enemies).chain(explosions))
    }
}

impl ParticleRenderer {
    pub fn new(shader: ParticleShaderDefinition) -> Self {
        Self { shader }
    }

    pub fn render(
        &self,
        context: &mut Context,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        let particles = entity_manager.get_view();

        context.use_shader(&self.shader, |context| {
            let (projection, view) = camera.ind_projection_view(camera_transform);
            context.shader.projection(&projection);
            context.shader.view(&view);

            particles.for_each(
                |(_transform, particle_emitter): (&Transform, &ParticleEmitter)| {
                    context.use_material(&particle_emitter.material, |_context| {
                        particle_emitter.mesh.render();
                    });
                },
            );
        });
    }
}
