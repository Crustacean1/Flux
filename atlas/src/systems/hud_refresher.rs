use glam::Vec3;

use crate::{
    entity_manager::{self, EntityManager},
    game_entities::{hud::HudEntity, player_ship::PlayerShip},
};

pub struct HudRefresher {
    pub hud_id: usize,
    pub player_id: usize,
}

impl HudRefresher {
    pub fn update(&self, entity_manager: &mut EntityManager) {
        let mut info = (0.0, Vec3::ZERO);
        entity_manager
            .iter::<PlayerShip>()
            .find(|p| p.id == self.player_id)
            .map(|p| {
                info = (
                    p.entity.physical_body.velocity().length(),
                    p.transform.position,
                )
            });

        entity_manager
            .iter_mut::<HudEntity>()
            .find(|h| h.id == self.hud_id)
            .map(|h| {
                h.entity
                    .velocity
                    .set_text(format!("Velocity: {:.2} [m/s]", info.0))
            });
    }
}
