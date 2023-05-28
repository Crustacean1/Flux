use std::{collections::HashMap, rc::Rc};

use crate::{
    game_root::GameError, graphics::graphics_context::GraphicsContext, logger::Logger, scene::Scene,
};

impl From<std::io::Error> for GameError {
    fn from(value: std::io::Error) -> Self {
        GameError::new(&value.to_string())
    }
}

pub type SceneInitializer = fn(&mut GraphicsContext) -> Result<Box<dyn Scene>, GameError>;

pub struct SceneManager {
    scene_initializers: HashMap<String, SceneInitializer>,
}

impl SceneManager {
    pub fn new(logger: Rc<dyn Logger>) -> Result<Self, GameError> {
        Ok(SceneManager {
            scene_initializers: HashMap::new(),
        })
    }

    pub fn register_scene(
        &mut self,
        scene_id: &str,
        scene_init: SceneInitializer,
    ) -> Result<(), GameError> {
        match self.scene_initializers.get(scene_id) {
            Some(_) => Err(GameError::new(&format!(
                "Failed to register '{}' scene with this id is already registered",
                scene_id
            ))),
            None => {
                self.scene_initializers
                    .insert(String::from(scene_id), scene_init);
                Ok(())
            }
        }
    }

    pub fn get_scene(
        &mut self,
        res_id: &str,
        graphics_context: &mut GraphicsContext,
    ) -> Result<Box<dyn Scene>, GameError> {
        if let Some(scene_initializer) = self.scene_initializers.get(res_id) {
            match scene_initializer(graphics_context) {
                Ok(scene) => Ok(scene),
                Err(e) => Err(GameError::new(&format!(
                    "Failed to initialize scene: '{}': {}",
                    res_id, e
                ))),
            }
        } else {
            Err(GameError::new(&format!("No scene with id: '{}'", res_id)))
        }
    }
}
