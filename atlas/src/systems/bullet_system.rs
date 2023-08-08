use crate::{
    entity_manager::{EntityManager},
    event_bus::EventSender,
    game_entities::bullet::BulletEntity,
};

use super::player_controller::GameEvent;

pub fn update_bullets(entity_manager: &mut EntityManager, event_sender: &mut EventSender, delta: u128) {
    let delta = delta as f32 / 1_000_000_000 as f32;
    //println!("New day - new frame");
    entity_manager
        .iter_mut::<BulletEntity>()
        .for_each(|bullet| {
            bullet.entity.lifetime -= delta;
            //println!("My position in world: {:?}", bullet.transform.position);
        });
    let dead_bullets = entity_manager.iter::<BulletEntity>().filter_map(|bullet| {
        if bullet.entity.lifetime < 0.0 {
            Some(bullet.id)
        } else {
            None
        }
    });

    dead_bullets.for_each(|entity| event_sender.write(GameEvent::RemoveBullet(entity)));
}
