use atlas::{
    components::skybox_renderer::SkyboxSystem,
    game_root::GameError,
    graphics::graphics_context::GraphicsContext,
    resource_manager::{root_resource_manager::RootResourceManager, ResourceManager},
    scene::Scene,
};

pub struct FirstScene {
    skybox_system: SkyboxSystem,
}

impl Scene for FirstScene {
    fn run(
        &mut self,
        graphics_context: &mut atlas::graphics::graphics_context::GraphicsContext,
    ) -> atlas::scene::SceneEvent {
        loop {
            self.skybox_system.render();

            graphics_context.display();
        }
    }
}

impl FirstScene {
    pub fn new(
        root_resource_manager: &mut RootResourceManager,
        graphics_context: &mut GraphicsContext,
    ) -> Result<Box<dyn Scene>, GameError> {
        let skybox_system = SkyboxSystem::new(root_resource_manager.get("basic_skybox")?.res);

        Ok(Box::new(FirstScene { skybox_system }))
    }
}
