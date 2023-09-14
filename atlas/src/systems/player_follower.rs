use glam::{Quat, Vec3};

use crate::{entity_manager::EntityManager, game_entities::player_ship::PlayerShip};

pub fn follow_player(entity_manager: &mut EntityManager) {
    if let Some(player) = entity_manager.iter_mut::<PlayerShip>().next() {
        let (player, camera) = (&mut player.transform, &player.entity.camera);
        player.rotation = player.rotation.slerp(camera.rotation().conjugate(), 0.1);
    }
}
