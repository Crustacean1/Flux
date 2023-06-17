use glam::Mat4;

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::thruster::ParticleEmitterEntity,
    graphics::{
        material::Material,
        shaders::{particle_shader::ParticleShader, ShaderProgram},
    },
};

use super::{camera::Camera, particle_emitter::ParticleEmitter, transform::Transform};

pub struct ParticleRenderer {
    shader: ShaderProgram<ParticleShader>,
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a ParticleEmitter)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a ParticleEmitter)> + 'a> {
        Box::new(
            self.iter::<ParticleEmitterEntity>()
                .map(|emmiter| (&emmiter.transform, &emmiter.entity.emitter)),
        )
    }
}

impl ParticleRenderer {
    pub fn new(shader: ShaderProgram<ParticleShader>) -> Self {
        Self { shader }
    }

    pub fn render(
        &self,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        entity_manager.get_view().for_each(
            |(transform, particle_emmiter): (&Transform, &ParticleEmitter)| {
                self.shader.bind();
                let (projection, view) = camera.projection_view(camera_transform);

                self.shader
                    .bind_projection_view(&projection.to_cols_array(), &view.to_cols_array());
                particle_emmiter.material.bind();
                particle_emmiter.mesh.render();
            },
        );
    }
}
