use std::mem;

use crate::{
    components::{
        physical_body::{PhysicalBody, PhysicalInteraction},
        transform::Transform,
    },
    entity_manager::{ComponentIteratorGenerator, ComponentMutIteratorGenerator, EntityManager},
    game_entities::{enemy_ship::EnemyShip, player_ship::PlayerShip},
};

pub struct PhysicalSimulation {
    physical_interactions: Vec<(usize, PhysicalInteraction)>,
    delta: f32,
    prev_time: f32,
}

impl<'a> ComponentIteratorGenerator<'a, (usize, &'a Transform, &'a PhysicalBody)>
    for EntityManager
{
    fn get_view(
        &'a self,
    ) -> Box<dyn Iterator<Item = (usize, &'a Transform, &'a PhysicalBody)> + 'a> {
        let enemies = self
            .iter::<EnemyShip>()
            .map(|ship| (ship.id, &ship.transform, &ship.entity.physical_body));

        let players = self
            .iter::<PlayerShip>()
            .map(|ship| (ship.id, &ship.transform, &ship.entity.physical_body));

        Box::new(enemies.chain(players))
    }
}

impl PhysicalSimulation {
    pub fn new(delta: f32) -> Self {
        Self {
            delta,
            prev_time: 0.,
            physical_interactions: vec![],
        }
    }

    pub fn delta(&self) -> u128 {
        (self.delta * 1_000_000_000.0) as u128
    }

    pub fn integrate_movement(&mut self, entity_manager: &mut EntityManager) {
        entity_manager.get_view().for_each(
            |(_, transform, physical_body): (usize, &Transform, &PhysicalBody)| {
                let transform = Self::to_mut(transform);
                let physical_body = Self::to_mut(physical_body);
                transform.position += physical_body.position_delta(self.delta);
                physical_body.update(self.delta);
            },
        );
        self.physical_interactions.clear();
    }

    fn to_mut<T>(value: &T) -> &mut T {
        unsafe {
            let ptr: *const T = value;
            mem::transmute(ptr)
        }
    }
}
