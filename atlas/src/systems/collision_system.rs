use std::mem;

use glam::Vec3;

use crate::{
    components::{
        collider::{collide, Collider},
        physical_body::PhysicalBody,
        transform::Transform,
    },
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    event_bus::EventSender,
    game_entities::{
        asteroid::AsteroidEntity, bullet::BulletEntity, enemy_ship::EnemyShip,
        player_ship::PlayerShip,
    },
};

impl<'a> ComponentIteratorGenerator<'a, (usize, &'a Transform, &'a Collider, &'a PhysicalBody)>
    for EntityManager
{
    fn get_view(
        &'a self,
    ) -> Box<dyn Iterator<Item = (usize, &'a Transform, &'a Collider, &'a PhysicalBody)> + 'a> {
        let enemies = self.iter::<EnemyShip>().map(|enemy| {
            (
                enemy.id,
                &enemy.transform,
                &enemy.entity.collider,
                &enemy.entity.physical_body,
            )
        });
        let players = self.iter::<PlayerShip>().map(|player| {
            (
                player.id,
                &player.transform,
                &player.entity.collider,
                &player.entity.physical_body,
            )
        });

        let asteroids = self.iter::<AsteroidEntity>().map(|asteroid| {
            (
                asteroid.id,
                &asteroid.transform,
                &asteroid.entity.collider,
                &asteroid.entity.body,
            )
        });

        let bullets = self.iter::<BulletEntity>().map(|bullet| {
            (
                bullet.id,
                &bullet.transform,
                &bullet.entity.collider,
                &bullet.entity.body,
            )
        });

        Box::new(enemies.chain(players).chain(asteroids).chain(bullets))
    }
}

pub struct CollisionSystem {}

type CollisionBundle<'a> = (usize, &'a Transform, &'a Collider, &'a PhysicalBody);

impl CollisionSystem {
    pub fn resolve_collisions(
        global_time: f32,
        delta: f32,
        _event_sender: &mut EventSender,
        entity_manager: &EntityManager,
    ) {
        entity_manager.get_view().enumerate().for_each(
            |(i, (id_a, transform_a, collider_a, physic_a)): (_, CollisionBundle)| {
                entity_manager.get_view().skip(i + 1).for_each(
                    |(id_b, transform_b, collider_b, physic_b): CollisionBundle| {
                        if let Some(time) = collide(
                            delta,
                            (transform_a, collider_a, physic_a),
                            (transform_b, collider_b, physic_b),
                        ) {
                            to_mut(physic_a).update(time, to_mut(transform_a));
                            to_mut(physic_b).update(time, to_mut(transform_b));

                            let axis = (transform_b.position - transform_a.position).normalize();
                            let contact = transform_a.position + axis * collider_a.radius;

                            collider_a
                                .callback
                                .as_ref()
                                .map(|callback| callback(id_a, id_b, contact));

                            collider_b
                                .callback
                                .as_ref()
                                .map(|callback| callback(id_b, id_a, contact));

                            let calculate_impulse = |a: &PhysicalBody, b: &PhysicalBody| -> f32 {
                                let velocity = (b.velocity() - a.velocity()).dot(axis);

                                let (v1, v2) = (0.0, velocity);
                                let (m1, m2) = (a.mass, b.mass);
                                let imp_a = (v1 * (m1 - m2) + 2.0 * v2 * m2) / (m1 + m2);
                                imp_a * m1
                            };

                            let impulse_a = calculate_impulse(physic_a, physic_b);
                            let impulse_b = calculate_impulse(physic_b, physic_a);

                            {
                                let collider_a = to_mut(collider_a);
                                let collider_b = to_mut(collider_b);
                                collider_a.toi = global_time;
                                collider_b.toi = global_time;

                                collider_a.last_impact = axis;
                                collider_b.last_impact = -axis;

                                to_mut(physic_a).impulse(axis * impulse_a);
                                to_mut(physic_b).impulse(axis * impulse_b);
                            }
                        }
                    },
                );
            },
        );
    }
}

fn to_mut<T>(val: &T) -> &mut T {
    unsafe {
        let ptr: *const T = val;
        mem::transmute(ptr)
    }
}
