use std::cell::RefCell;

use glam::Vec3;
use rand::Rng;

use crate::{
    components::particle_emitter::{Particle, ParticleEmitter, ParticleEmitterDefinition},
    entity_manager::{EntityManager},
    event_bus::EventReader,
    game_entities::bullet::BulletEntity,
    graphics::{instanced_mesh::InstancedMesh, vertices::generator},
};

pub enum BulletEvent {
    Collision(usize),
}

pub fn process_bullet_events(event_bus: &mut EventReader, entity_manager: &mut EntityManager) {
    event_bus.read::<BulletEvent>().map(|events| {
        events.for_each(|event| match event {
            BulletEvent::Collision(bullet_id) => {
                if let Some(bullet) = entity_manager.get_mut::<BulletEntity>(bullet_id) {
                    let (vertices, indices) = generator::quad(1.0, 1.0);
                    let instanced_mesh = InstancedMesh::new(&vertices, &indices, &vec![]);
                    let explosion = ParticleEmitter::new(
                        ParticleEmitterDefinition {
                            count: 1000,
                            rate: 1000.0,
                        },
                        Default::default(),
                        instanced_mesh,
                        Box::new(create_spawner(Vec3::new(0.0, 0.0, 0.0))),
                    );

                    bullet.entity.explosion_effect = Some(explosion);
                }
            }
        })
    });
}
fn create_spawner(pos: Vec3) -> impl Fn(&mut Particle) {
    let rng = RefCell::new(rand::thread_rng());
    move |particle| {
        let mut rng = rng.try_borrow_mut().unwrap();
        particle.position = pos.to_array();
        let (ang_x, ang_y) = (
            rng.gen_range(0.0..3.14 * 2.0) as f32,
            rng.gen_range(0.0..3.14 * 2.0) as f32,
        );
        particle.velocity = [
            10. * ang_x.cos() * ang_y.cos(),
            10. * ang_y.sin(),
            10. * ang_x.sin() * ang_y.cos(),
        ];
        particle.color = [1.0, 0.0, 0.0]
    }
}
