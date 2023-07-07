use std::{fmt, rc::Rc};

use crate::{
    graphics::graphics_context::GraphicsContext,
    logger::{console_logger::ConsoleLogger, Logger},
    resource_manager::scene_manager::SceneManager,
    scene::{Scene, SceneEvent},
};

#[derive(Debug, Clone)]
pub struct GameError {
    msg: String,
}

impl GameError {
    pub fn new(msg: &str) -> Self {
        GameError {
            msg: String::from(msg),
        }
    }

    pub fn err<T>(msg: String) -> Result<T, Self> {
        Err(GameError { msg })
    }

    pub fn uniform(uniform: &str) -> Self {
        GameError {
            msg: format!("Failed to find uniform with id: '{}'", uniform),
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)?;
        Ok(())
    }
}

pub struct GameRoot {
    scene_manager: SceneManager,
    logger: Rc<dyn Logger>,
    graphics_context: GraphicsContext,
}

impl GameRoot {
    pub fn new(title: &str) -> Result<Self, GameError> {
        let logger = Rc::new(ConsoleLogger::new());
        let graphics_context = GraphicsContext::new(title)?;
        let scene_manager = SceneManager::new()?;

        Ok(GameRoot {
            logger,
            scene_manager,
            graphics_context,
        })
    }

    pub fn run(&mut self) {
        let mut next_scene = String::from("first_scene");
        loop {
            let mut scene: Box<dyn Scene> = match self
                .scene_manager
                .get_scene(&next_scene, &mut self.graphics_context)
            {
                Ok(scene) => scene,
                Err(e) => {
                    self.logger.log_error(&format!(
                        "Failed to load scene '{}' : {}",
                        next_scene,
                        e.to_string()
                    ));
                    return;
                }
            };

            match scene.run(&mut self.graphics_context) {
                SceneEvent::Exit => {
                    self.logger.log_info("Exiting the game");
                    break;
                }
                SceneEvent::NewScene(scene) => {
                    self.logger
                        .log_info(&format!("Transitioning to: {}", scene));
                    next_scene = String::from(scene);
                }
                _ => {}
            }
        }
    }

    pub fn scene_manager(&mut self) -> &mut SceneManager {
        &mut self.scene_manager
    }
}

impl Drop for GameRoot {
    fn drop(&mut self) {
        self.logger.log_info("GameRoot cleanup");
    }
}
