use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    mem,
};

use glam::Vec3;
use rand::Rng;

use crate::{
    components::{
        particle_emitter::{Particle, ParticleEmitter, ParticleEmitterDefinition},
        unit::Unit,
    },
    entity_manager::{ComponentIteratorGenerator, EntityManager, EntityManagerTrait},
    event_bus::{EventReader, EventSender},
    game_entities::{
        asteroid::AsteroidEntity, bullet::BulletEntity, enemy_ship::EnemyShip, explosion::Explosion,
    },
    graphics::{instanced_mesh::InstancedMesh, vertices::generator},
    resource_manager::{
        self, scene_resource_manager::SceneResourceManager, ResourceLoader, ResourceManager,
    },
};

use super::player_controller::GameEvent;

pub enum BulletEvent {
    Exploded(usize, Vec3),
    Damaged(usize, f32),
    Extinguished(usize),
}

pub enum UnitEvent {
    Destroyed(usize),
}

impl<'a> ComponentIteratorGenerator<'a, (usize, &'a Unit)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (usize, &Unit)> + 'a> {
        let enemies = self
            .iter::<EnemyShip>()
            .map(|ship| (ship.id, &ship.entity.info));

        let asteroids = self
            .iter::<AsteroidEntity>()
            .map(|asteroid| (asteroid.id, &asteroid.entity.info));

        Box::new(enemies.chain(asteroids))
    }
}

pub fn process_bullet_events(
    event_bus: &EventReader,
    event_writer: &EventSender,
    resource_manager: &mut SceneResourceManager,
    entity_manager: &mut EntityManager,
    delta: f32,
) {
    let mut deaths = vec![];
    event_bus.read::<BulletEvent>(|event| match event {
        BulletEvent::Exploded(bullet, position) => {
            entity_manager.remove::<BulletEntity>(bullet);
            let (vertices, indices) = generator::quad(1.0, 1.0);
            let instanced_mesh = InstancedMesh::new(&vertices, &indices, &vec![]);
            let material = resource_manager.get("explosion").res;
            let explosion = ParticleEmitter::new(
                ParticleEmitterDefinition {
                    count: 256,
                    rate: 0.00005,
                },
                material,
                instanced_mesh,
                Box::new(create_spawner(position)),
            );

            entity_manager.add(Explosion {
                lifetime: 0.5,
                explosion,
            });
        }
        BulletEvent::Damaged(entity, damage) => {
            if let Some((id, unit)) = entity_manager
                .get_view()
                .find(|(id, _): &(usize, &Unit)| *id == entity)
            {
                to_mut(unit).health -= damage;
                if unit.health < 0.0 {
                    deaths.push(id);
                }
            }
        }
        BulletEvent::Extinguished(explosion) => {
            entity_manager.remove::<Explosion>(explosion);
        }
    });
    entity_manager
        .iter_mut::<Explosion>()
        .for_each(|explosion| explosion.entity.lifetime -= delta);

    let mut death_messages = HashSet::<usize>::new();
    deaths.iter().for_each(|death| {
        death_messages.insert(*death);
    });
    death_messages
        .iter()
        .for_each(|death| event_writer.write(UnitEvent::Destroyed(*death)));

    let extinguished: Vec<_> = entity_manager
        .iter::<Explosion>()
        .filter_map(|explosion| {
            if explosion.entity.lifetime <= 0.0 {
                Some(explosion.id)
            } else {
                None
            }
        })
        .collect();

    extinguished.iter().for_each(|&id| {
        entity_manager.remove::<Explosion>(id);
    })
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
        let velocity = rng.gen_range(25.0..35.0);
        particle.velocity = [
            velocity * ang_x.cos() * ang_y.cos(),
            velocity * ang_y.sin(),
            velocity * ang_x.sin() * ang_y.cos(),
        ];
        particle.color = [0.8, rng.gen_range(0.0..0.5), 0.15, 0.75];

        particle.tex = rng.gen_range(0..4) as f32 / 4.0;

        particle.size = rng.gen_range(0.5..1.75);
        particle.lifetime = 0.75;
        particle.dampening = 0.1;

        particle.opacity_delta = 2.5;
    }
}

fn to_mut<T>(val: &T) -> &mut T {
    unsafe {
        let ptr: *const T = val;
        mem::transmute(ptr)
    }
}
