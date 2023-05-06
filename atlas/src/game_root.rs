use std::{fmt, rc::Rc};

use crate::{
    graphics::graphics_context::GraphicsContext,
    logger::{console_logger::ConsoleLogger, Logger},
    root_resource_manager::{self, ResourceError, ResourceManager, RootResourceManager},
    scene::{Scene, SceneAction},
};

#[derive(Clone)]
pub struct GameError {
    msg: String,
}

impl GameError {
    pub fn new(msg: &str) -> Self {
        GameError {
            msg: String::from(msg),
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)?;
        Ok(())
    }
}

impl From<ResourceError> for GameError {
    fn from(value: ResourceError) -> Self {
        GameError {
            msg: value.to_string(),
        }
    }
}

pub struct GameRoot {
    root_resource_manager: RootResourceManager,
    logger: Rc<dyn Logger>,
    graphics_context: GraphicsContext,
}

impl GameRoot {
    pub fn new(title: &str) -> Result<Self, GameError> {
        let logger = Rc::new(ConsoleLogger::new());
        let graphics_context = GraphicsContext::new(title)?;
        let root_resource_manager = RootResourceManager::new(logger.clone());

        Ok(GameRoot {
            logger,
            root_resource_manager,
            graphics_context,
        })
    }

    pub fn run(&mut self) {
        let mut next_scene = String::from("main");
        loop {
            if let Some(mut scene) = self.root_resource_manager.get(&next_scene) {
                match scene.run(self.logger.clone(), &mut self.graphics_context) {
                    SceneAction::Exit => {
                        self.logger.log_info("Exiting the game");
                        break;
                    }
                    SceneAction::NewScene(scene) => {
                        self.logger
                            .log_info(&format!("Transitioning to: {}", scene));
                        next_scene = String::from(scene);
                    }
                    _ => {}
                }
            } else {
                self.logger
                    .log_error(&format!("No scene with name: '{}' found", next_scene))
            }
        }
    }

    pub fn resource_manager_mut(&mut self) -> &mut RootResourceManager {
        &mut self.root_resource_manager
    }
}

impl Drop for GameRoot {
    fn drop(&mut self) {
        self.logger.log_info("GameRoot cleanup");
    }
}
