use atlas::{
    components::shape_renderer::ShapeRenderer,
    game_root::GameError,
    resource_manager::{scene_resource_manager::SceneResourceManager, ResourceManager},
};

use crate::component_manager::{ComponentManager, EntityManager};

pub fn main_menu<T>(
    component_manager: &mut T,
    resource_manager: &mut SceneResourceManager,
) -> Result<(), GameError>
where
    T: ComponentManager<ShapeRenderer> + EntityManager,
{
    let material_resource = resource_manager.get("button")?;

    let entity = component_manager.add_entity();
    component_manager.add_component(
        entity,
        ShapeRenderer::quad((128.0, 32.0), material_resource.res),
    );

    Ok(())
}
