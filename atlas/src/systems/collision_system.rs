use std::mem;

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

use super::bullet_detonator::BulletEvent;

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a Collider, &'a PhysicalBody)>
    for EntityManager
{
    fn get_view(
        &'a self,
    ) -> Box<dyn Iterator<Item = (&'a Transform, &'a Collider, &'a PhysicalBody)> + 'a> {
        let enemies = self
            .iter::<EnemyShip>()
            .map(|e| (&e.transform, &e.entity.collider, &e.entity.physical_body));
        let players = self
            .iter::<PlayerShip>()
            .map(|p| (&p.transform, &p.entity.collider, &p.entity.physical_body));

        let asteroids = self
            .iter::<AsteroidEntity>()
            .map(|a| (&a.transform, &a.entity.collider, &a.entity.body));

        let bullets = self
            .iter::<BulletEntity>()
            .map(|a| (&a.transform, &a.entity.collider, &a.entity.body));

        Box::new(enemies.chain(players).chain(asteroids).chain(bullets))
    }
}

pub struct CollisionSystem {}

impl CollisionSystem {
    pub fn resolve_collisions(event_sender: &mut EventSender, entity_manager: &EntityManager) {
        entity_manager.get_view().enumerate().for_each(
            |(i, (transform_a, collider_a, physic_a)): (
                _,
                (&Transform, &Collider, &PhysicalBody),
            )| {
                entity_manager.get_view().skip(i + 1).for_each(
                    |(transform_b, collider_b, physic_b): (
                        &Transform,
                        &Collider,
                        &PhysicalBody,
                    )| {
                        if let Some(contact) =
                            collide((transform_a, collider_a), (transform_b, collider_b))
                        {
                            collider_a
                                .callback
                                .as_ref()
                                .map(|callback| callback(contact));
                            collider_b
                                .callback
                                .as_ref()
                                .map(|callback| callback(contact));
                            //let direction =
                            //let velocity = (physic_a.velocity() - physic_b.velocity()).dot();

                            let a_displacement = transform_a.position - contact;
                            let a_magnitude = collider_a.radius - a_displacement.length();
                            let a_force = a_displacement.normalize() * (a_magnitude * 256.0);

                            let b_displacement = transform_b.position - contact;
                            let b_magnitude = collider_b.radius - b_displacement.length();
                            let b_force = b_displacement.normalize() * (b_magnitude * 256.0);

                            let physic_a = to_mut(physic_a);
                            physic_a.add_force(a_force);

                            let physic_b = to_mut(physic_b);
                            physic_b.add_force(b_force);
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
