use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::{enemy_ship::EnemyShip, player_ship::PlayerShip},
    graphics::{
        material::Material,
        shaders::{particle_shader::ParticleShader, ShaderProgram},
    },
};

use super::{camera::Camera, particle_emitter::ParticleEmitter, transform::Transform};

pub struct ParticleRenderer {
    shader: ParticleShader,
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a ParticleEmitter)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a ParticleEmitter)> + 'a> {
        let players = self
            .iter::<PlayerShip>()
            .map(|player| (&player.transform, &player.entity.thruster));
        let enemies = self
            .iter::<EnemyShip>()
            .map(|enemy| (&enemy.transform, &enemy.entity.thruster));
        Box::new(players.chain(enemies))
    }
}

impl ParticleRenderer {
    pub fn new(shader: ParticleShader) -> Self {
        Self { shader }
    }

    pub fn render(
        &self,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        let (projection, view) = camera.projection_view(camera_transform);
        let pass = self.shader.new_pass(&projection, &view);

        entity_manager.get_view().for_each(
            |(transform, particle_emitter): (&Transform, &ParticleEmitter)| {
                particle_emitter.material.bind();
                pass.render(&particle_emitter.mesh);
            },
        );
    }
}
