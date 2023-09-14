use std::cell::RefCell;

use glam::Vec3;
use rand::Rng;

use crate::{
    components::particle_emitter::{Particle, ParticleEmitter, ParticleEmitterDefinition},
    entity_manager::EntityManager,
    event_bus::EventReader,
    game_entities::{asteroid::AsteroidEntity, explosion::Explosion},
    graphics::{instanced_mesh::InstancedMesh, vertices::generator},
    resource_manager::{scene_resource_manager::SceneResourceManager, ResourceManager},
};

use super::bullet_detonator::UnitEvent;

pub fn detonate_asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
    event_reader: &EventReader,
) {
    event_reader.read(|event: UnitEvent| match event {
        UnitEvent::Destroyed(id) => {
            let position = entity_manager
                .get::<AsteroidEntity>(id)
                .map(|a| a.transform.position)
                .unwrap();

            entity_manager.remove::<AsteroidEntity>(id);

            let (vertices, indices) = generator::quad(1.0, 1.0);
            let instanced_mesh = InstancedMesh::new(&vertices, &indices, &vec![]);
            let material = resource_manager.get("explosion").res;
            let explosion = ParticleEmitter::new(
                ParticleEmitterDefinition {
                    count: 1024,
                    rate: 0.00005,
                },
                material,
                instanced_mesh,
                Box::new(fire_ball(position)),
            );

            entity_manager.add(Explosion {
                lifetime: 25.0,
                explosion,
            });
        }
    });
}

fn fire_ring(pos: Vec3) -> impl Fn(&mut Particle) {
    let rng = RefCell::new(rand::thread_rng());
    move |particle| {
        let mut rng = rng.try_borrow_mut().unwrap();
        particle.position = pos.to_array();
        let (ang_x, ang_y) = (
            rng.gen_range(0.0..3.14 * 2.0) as f32,
            rng.gen_range(0.0..3.14 * 2.0) as f32,
        );
        let velocity = rng.gen_range(70.0..75.0);

        particle.velocity = [
            velocity * ang_x.cos() * ang_y.cos(),
            velocity * ang_y.sin(),
            velocity * ang_x.sin() * ang_y.cos(),
        ];

        particle.color = [0.4, rng.gen_range(0.0..0.5), 0.55, 0.75];

        particle.tex = rng.gen_range(0..4) as f32 / 4.0;

        particle.size = rng.gen_range(0.5..1.75);
        particle.lifetime = 3.0;
        particle.dampening = 0.4;

        particle.opacity_delta = 2.5;
    }
}

fn fire_ball(pos: Vec3) -> impl Fn(&mut Particle) {
    let rng = RefCell::new(rand::thread_rng());
    move |particle| {
        let mut rng = rng.try_borrow_mut().unwrap();
        particle.position = pos.to_array();
        let (ang_x, ang_y) = (
            rng.gen_range(0.0..3.14 * 2.0) as f32,
            rng.gen_range(0.0..3.14 * 2.0) as f32,
        );
        let velocity = rng.gen_range(40.0..45.0);
        particle.velocity = [
            velocity * ang_x.cos() * ang_y.cos(),
            velocity * ang_y.sin(),
            velocity * ang_x.sin() * ang_y.cos(),
        ];

        particle.color = [0.4, rng.gen_range(0.0..0.5), 0.55, 0.75];

        particle.tex = rng.gen_range(0..4) as f32 / 4.0;

        particle.size = rng.gen_range(0.5..1.75);
        particle.lifetime = 3.0;
        particle.dampening = 0.99;

        particle.opacity_delta = 2.5;
    }
}
