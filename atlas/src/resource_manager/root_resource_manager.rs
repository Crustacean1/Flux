use std::{collections::HashMap, env, path::PathBuf, rc::Rc};

use crate::{
    game_root::GameError,
    graphics::{
        graphics_context::GraphicsContext,
        shaders::{ShaderProgram, ShaderSource, SkyboxShader, UiShader},
    },
    logger::Logger,
    scene::Scene,
};

use super::{indexer::index_resources, resource::Resource, shader::index_shaders, ResourceManager};

impl From<std::io::Error> for GameError {
    fn from(value: std::io::Error) -> Self {
        GameError::new(&value.to_string())
    }
}

pub type SceneInitializer =
    fn(&mut RootResourceManager, &mut GraphicsContext) -> Result<Box<dyn Scene>, GameError>;

pub struct RootResourceManager {
    logger: Rc<dyn Logger>,
    scene_initializers: HashMap<String, SceneInitializer>,
    ui_shaders: HashMap<String, Resource<ShaderProgram<UiShader>>>,
    skybox_shaders: HashMap<String, Resource<ShaderProgram<SkyboxShader>>>,
}

impl RootResourceManager {
    pub fn new(logger: Rc<dyn Logger>) -> Result<Self, GameError> {
        Ok(RootResourceManager {
            logger,
            scene_initializers: HashMap::new(),
            ui_shaders: HashMap::new(),
            skybox_shaders: HashMap::new(),
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
        let resource_index = index_resources(&root)?;

        self.ui_shaders = resource_index
            .iter()
            .find(|(res_type, _)| res_type == "ui_shaders")
            .map_or(
                Err(GameError::new("No ui_shaders found")),
                |(_, ui_shaders)| self.load_shader(index_shaders(ui_shaders)),
            )?;

        for (key, value) in self.ui_shaders.iter() {
            println!("Shader: '{}' with res_id: '{}'", key, value.id());
        }

        println!("Resource type count: {}", resource_index.len());

        self.skybox_shaders = resource_index
            .iter()
            .find(|(res_type, _)| res_type == "skybox_shaders")
            .map_or(
                Err(GameError::new("No skybox_shaders found")),
                |(_, shaders)| self.load_shader(index_shaders(shaders)),
            )?;

        Ok(())
    }

    fn load_shader<T: Clone>(
        &self,
        shaders: HashMap<String, ShaderSource>,
    ) -> Result<HashMap<String, Resource<ShaderProgram<T>>>, GameError> {
        shaders.iter().fold(
            Ok(HashMap::new()),
            |shaders: Result<HashMap<String, Resource<ShaderProgram<T>>>, GameError>,
             (shader_name, shader_source)| {
                let mut shaders = shaders?;
                self.logger
                    .log_info(&format!("Loading shader: {}", shader_name));
                shaders.insert(
                    shader_name.clone(),
                    Resource::new(shader_name.clone(), ShaderProgram::load(&shader_source)?),
                );
                Ok(shaders)
            },
        )
    }

    fn res_root_path() -> Result<PathBuf, GameError> {
        let mut game_dir = env::current_exe()?;
        game_dir.pop();
        game_dir.pop();
        game_dir.pop();
        game_dir.push("flux");
        game_dir.push("assets");
        Ok(game_dir)
    }

    pub fn get_scene(
        &mut self,
        res_id: &str,
        graphics_context: &mut GraphicsContext,
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

impl ResourceManager<ShaderProgram<SkyboxShader>> for RootResourceManager {
    fn get(&mut self, res_id: &str) -> Result<Resource<ShaderProgram<SkyboxShader>>, GameError> {
        if let Some(shader) = self.skybox_shaders.get(res_id) {
            Ok(shader.clone())
        } else {
            Err(GameError::new(&format!("No shader with id: {}", res_id)))
        }
    }
}
