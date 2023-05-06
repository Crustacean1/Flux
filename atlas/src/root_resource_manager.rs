use std::{collections::HashMap, fmt, rc::Rc};

use crate::{game_root::GameError, logger::Logger, scene::Scene};

#[derive(Debug, Clone)]
pub struct ResourceError {
    msg: String,
}

impl ResourceError {
    pub fn new<T>(path: &str, msg: &str) -> Result<T, Self> {
        Err(ResourceError {
            msg: format!("Failed to load '{}':{}", path, msg),
        })
    }
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)?;
        Ok(())
    }
}

pub trait Resourcefull<T> {
    fn load(path: &str) -> Result<T, ResourceError>;
}

pub trait ResourceManager<T> {
    fn get(&mut self, res_id: &str) -> Option<T>;
}

type SceneInitializer = fn() -> Box<dyn Scene>;

pub struct RootResourceManager {
    logger: Rc<dyn Logger>,
    scene_initializers: HashMap<String, SceneInitializer>,
}

impl RootResourceManager {
    pub fn new(logger: Rc<dyn Logger>) -> Self {
        RootResourceManager {
            logger,
            scene_initializers: HashMap::new(),
        }
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
                self.logger
                    .log_info(&format!("Registered scene '{}'", scene_id));
                Ok(())
            }
        }
    }
}

impl ResourceManager<Box<dyn Scene>> for RootResourceManager {
    fn get(&mut self, res_id: &str) -> Option<Box<dyn Scene>> {
        if let Some(scene_initializer) = self.scene_initializers.get(res_id) {
            Some(scene_initializer())
        } else {
            None
        }
    }
}
