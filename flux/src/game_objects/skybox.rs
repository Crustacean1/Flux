use atlas::{
    components::skybox_renderer::SkyboxRenderer,
    entity_manager::{EntityManager, EntityManagerTrait},
    resource_manager::{scene_resource_manager::SceneResourceManager, ResourceManager},
};

pub fn skybox(entity_manager: &mut EntityManager, resource_manager: &mut SceneResourceManager) {
    /*let space_box_textures = [
        "bkg1_right",
        "bkg1_left",
        "bkg1_bot",
        "bkg1_top",
        "bkg1_front",
        "bkg1_back",
    ];

    let space_box_textures: Vec<_> = space_box_textures
        .iter()
        .map(|texture| resource_manager.get(texture).res)
        .collect();

    let space_box = SkyboxRenderer::new(50.0, &space_box_textures);
    entity_manager.add_entity(space_box);*/
}
