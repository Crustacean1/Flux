use crate::components::transform::Transform;

pub mod enemy_ship;
pub mod player_ship;
pub mod space_box;
pub mod sprite;
pub mod starlight;
pub mod ui_label;
pub mod bullet;
pub mod asteroid;
pub mod hud;
pub mod explosion;

pub struct GameEntity<T> {
    pub entity: T,
    pub transform: Transform,
    pub id: usize,
}

impl<T> GameEntity<T> {
    pub fn new(entity: T, transform: Transform, global_id: &mut usize) -> Self {
        let id = *global_id;
        *global_id += 1;

        Self {
            entity,
            transform,
            id,
        }
    }
}
