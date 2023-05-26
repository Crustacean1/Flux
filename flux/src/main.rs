use atlas::{
    game_root::{GameError, GameRoot},
    resource_manager::root_resource_manager::{RootResourceManager, SceneInitializer},
};
use scene::{first_scene::FirstScene, main_menu_scene::MainMenuScene};

mod game_objects;
mod scene;

fn main() {
    match start_game() {
        Err(e) => {
            println!("Failed to start game: {}\n", e);
        }
        _ => {}
    }
}

fn start_game() -> Result<(), GameError> {
    let mut game_root = GameRoot::new("Flux")?;
    let root_resource_manager = game_root.resource_manager_mut();
    load_scenes(root_resource_manager)?;
    game_root.run();
    Ok(())
}

fn load_scenes(root_resource_manager: &mut RootResourceManager) -> Result<(), GameError> {
    let scenes: [(_, SceneInitializer); 2] = [
        ("main", MainMenuScene::new),
        ("first_scene", FirstScene::new),
    ];

    let failed_scenes: Vec<_> = scenes
        .iter()
        .filter(|&&(scene_id, scene_init)| {
            root_resource_manager
                .register_scene(scene_id, scene_init)
                .is_err()
        })
        .collect();

    if failed_scenes.is_empty() {
        Ok(())
    } else {
        let failed_scenes: Vec<_> = failed_scenes
            .iter()
            .map(|(scene_id, _)| String::from(*scene_id))
            .collect();

        Err(GameError::new(&format!(
            "Failed to load some scenes:\n{:?}",
            failed_scenes
        )))
    }
}
