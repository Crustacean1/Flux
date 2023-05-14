use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    game_root::GameError,
    graphics::{
        graphics_context::GraphicsContext,
        shaders::{ShaderProgram, UiShader},
    },
    logger::Logger,
    scene::Scene,
};

use super::{
    indexer::crawl_dirs, resource::Resource, shader::index_shaders, texture::index_textures,
    ResourceManager,
};

impl From<std::io::Error> for GameError {
    fn from(value: std::io::Error) -> Self {
        GameError::new(&value.to_string())
    }
}

type SceneInitializer =
    fn(&mut RootResourceManager, &GraphicsContext) -> Result<Box<dyn Scene>, GameError>;

pub struct RootResourceManager {
    logger: Rc<dyn Logger>,
    scene_initializers: HashMap<String, SceneInitializer>,
    /// all shaders are compiled on startup, no need for lazy loading
    ui_shaders: HashMap<String, Resource<ShaderProgram<UiShader>>>,
}

impl RootResourceManager {
    pub fn new(logger: Rc<dyn Logger>) -> Result<Self, GameError> {
        Ok(RootResourceManager {
            logger,
            scene_initializers: HashMap::new(),
            ui_shaders: HashMap::new(),
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
                self.logger
                    .log_info(&format!("Registered scene '{}'", scene_id));
                Ok(())
            }
        }
    }

    pub fn index_resources(&mut self) -> Result<(), GameError> {
        let root = Self::res_root_path()?;
        self.logger.log_info("indexing resources");

        let shader_sources = index_shaders(crawl_dirs(&root.join("ui_shaders"))?);

        self.ui_shaders = shader_sources.iter().fold(
            Ok(HashMap::new()),
            |shaders: Result<HashMap<String, Resource<ShaderProgram<UiShader>>>, GameError>,
             (shader_name, shader_source)| {
                let mut shaders = shaders?;
                shaders.insert(
                    shader_name.clone(),
                    Resource::new(&shader_name, ShaderProgram::load(&shader_source)?),
                );
                self.logger
                    .log_info(&format!("Loaded shader: {}", shader_name));
                Ok(shaders)
            },
        )?;
        Ok(())
    }

    fn res_root_path() -> Result<PathBuf, GameError> {
        let mut game_dir = env::current_exe()?;
        game_dir.pop();
        game_dir.pop();
        game_dir.pop();
        game_dir.push("flux");
        Ok(game_dir)
    }

    pub fn get_scene(
        &mut self,
        res_id: &str,
        graphics_context: &GraphicsContext,
    ) -> Result<Box<dyn Scene>, GameError> {
        if let Some(scene_initializer) = self.scene_initializers.get(res_id) {
            match scene_initializer(self, graphics_context) {
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

impl ResourceManager<ShaderProgram<UiShader>> for RootResourceManager {
    fn get(&mut self, res_id: &str) -> Result<Resource<ShaderProgram<UiShader>>, GameError> {
        if let Some(shader) = self.ui_shaders.get(res_id) {
            Ok(shader.clone())
        } else {
            Err(GameError::new(&format!("No shader with id: {}", res_id)))
        }
    }
}
