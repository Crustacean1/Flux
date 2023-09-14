use std::{mem, usize};

use glam::{Vec3, Vec4, Vec4Swizzles};

use crate::{
    components::{
        collider::{solve_quadratic, Collider, QuadraticSolution},
        health_renderer::HealthRenderer,
        physical_body::PhysicalBody,
        sprite_renderer::SpriteRenderer,
        text_renderer::TextRenderer,
        transform::Transform,
        unit::Unit,
    },
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    event_bus::EventSender,
    game_entities::enemy_ship::EnemyShip,
    systems::trail_renderer::TrailEvent,
};

use super::{asteroid::AsteroidEntity, player_ship::PlayerShip, GameEntity};

pub struct HudEntity {
    pub crosshair: SpriteRenderer,
    pub health: HealthRenderer,

    pub unit: TextRenderer,
    pub velocity: TextRenderer,
    pub mass: TextRenderer,
    pub player_id: usize,
}

impl<'a>
    ComponentIteratorGenerator<
        'a,
        (
            &'a Transform,
            usize,
            &'a Unit,
            &'a Collider,
            &'a PhysicalBody,
        ),
    > for EntityManager
{
    fn get_view(
        &'a self,
    ) -> Box<
        dyn Iterator<
                Item = (
                    &'a Transform,
                    usize,
                    &'a Unit,
                    &'a Collider,
                    &'a PhysicalBody,
                ),
            > + 'a,
    > {
        let enemies = self.iter::<EnemyShip>().map(|ship| {
            (
                &ship.transform,
                ship.id,
                &ship.entity.info,
                &ship.entity.collider,
                &ship.entity.physical_body,
            )
        });
        let asteroids = self.iter::<AsteroidEntity>().map(|ship| {
            (
                &ship.transform,
                ship.id,
                &ship.entity.info,
                &ship.entity.collider,
                &ship.entity.body,
            )
        });

        Box::new(enemies.chain(asteroids))
    }
}

pub fn update_hud(entity_manager: &mut EntityManager, event_sender: &EventSender) {
    let player: Option<&GameEntity<PlayerShip>> = entity_manager.iter().next();

    if let Some(player) = player {
        let (position, dir) = (
            player.transform.position
                + player
                    .transform
                    .to_global(Vec4::new(0.0, 1.5, 5.0, 0.0))
                    .xyz(),
            player.transform.to_global(Vec4::new(0.0, 0.0, -1.0, 0.0)),
        );
        let dir = dir.xyz();

        let intersection = entity_manager
            .get_view()
            .filter_map(
                |(transform, id, u, collider, physical): (
                    &Transform,
                    usize,
                    &Unit,
                    &Collider,
                    &PhysicalBody,
                )| {
                    let distance = dir
                        .cross((position - transform.position).normalize())
                        .length();

                    Some((distance, id, u, physical))
                },
            )
            .filter(|(d, _, _, _)| *d < 0.1)
            .min_by(|(a, _, _, _), (b, _, _, _)| a.total_cmp(b));

        intersection.map(|(_, id, _, _)| event_sender.write(TrailEvent::Focus(id)));

        let name = intersection
            .map(|i| i.2.name.clone())
            .map(|name| format!("Unit: {}", name))
            .unwrap_or(String::new());

        let unit_velocity = intersection
            .map(|i| (i.3.velocity() - player.entity.physical_body.velocity()).length())
            .map(|velocity| format!("Unit Velocity: {:.2}", velocity))
            .unwrap_or(String::new());

        let unit_mass = intersection
            .map(|i| i.3.mass)
            .map(|mass| format!("Mass: {:.2} Mg", mass))
            .unwrap_or(String::new());

        let helth = intersection.map(|i| i.2.health / i.2.max_health);

        entity_manager
            .iter()
            .for_each(|hud: &GameEntity<HudEntity>| {
                if let Some(player) = entity_manager
                    .iter()
                    .find(|e: &&GameEntity<PlayerShip>| e.id == hud.entity.player_id)
                {
                    let hud = to_mut(hud);
                    hud.entity.unit.set_text(name.clone());
                    hud.entity.velocity.set_text(unit_velocity.clone());
                    hud.entity.mass.set_text(unit_mass.clone());

                    if let Some(helth) = helth {
                        hud.entity.health.health = helth;
                        hud.entity.health.enabled = true;
                    } else {
                        hud.entity.health.enabled = false;
                    }
                }
            });
    }
}

fn cast_ray(pos: Vec3, dir: Vec3, sphere: Vec3, radius: f32) -> Option<f32> {
    let pos = pos - sphere;

    let [dx, dy, dz] = dir.to_array();
    let [x, y, z] = pos.to_array();

    let (a, b, c) = (
        dx.powi(2) + dy.powi(2) + dz.powi(2),
        2.0 * dx * x + 2.0 * dy * y + 2.0 * dz * z,
        x.powi(2) + y.powi(2) + z.powi(2) - radius.powi(2),
    );
    let time = solve_quadratic(a, b, c);

    match time {
        QuadraticSolution::Single(dist) => Some(dist),
        QuadraticSolution::Double(dist, _) => Some(dist),
        _ => None,
    }
}

fn to_mut<T>(v: &T) -> &mut T {
    unsafe {
        let ptr: *const T = v;
        mem::transmute(ptr)
    }
}
