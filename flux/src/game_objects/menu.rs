use atlas::{
    components::{
        button_handler::ButtonHandler, button_trigger::ButtonTrigger,
        shape_renderer::ShapeRenderer, transform::Transform,
    },
    entity_manager::{EntityManager, EntityManagerTrait},
    event_bus::EventSenderTrait,
    game_root::GameError,
    resource_manager::{
        resource::Resource, scene_resource_manager::SceneResourceManager, ResourceManager,
    },
    scene::SceneEvent, graphics::material::sprite_material::SpriteMaterial,
};
use glam::Vec3;

pub fn create_main_menu(
    component_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
    (width, height): (i32, i32),
) -> Result<(), GameError> {
    let mut buttons = ["play", "options", "exit"];

    let button_gap = 128.0;
    let vertical_offset = button_gap * 0.5 * (buttons.len() - 1) as f32;
    let center = (width as f32 / 2.0, height as f32 / 2.0 - vertical_offset);

    buttons
        .iter_mut()
        .enumerate()
        .for_each(|(index, button_name)| {
            let material = resource_manager.get(button_name);
            add_menu_entry(
                Vec3::new(center.0, center.1 + index as f32 * button_gap, 1.0),
                component_manager,
                &material,
            );
        });

    let main_screen = resource_manager.get("main_menu");
    component_manager.add_entity((
        Transform::pos(Vec3::new(width as f32 * 0.5, height as f32 * 0.5, 0.0)),
        ShapeRenderer::quad((width as f32 * 0.5, height as f32 * 0.5), main_screen.res),
        ButtonTrigger::new(0, (0.0, 0.0), (width as f32, height as f32)),
        Box::new(BackgroundHandler::new()) as Box<dyn ButtonHandler>,
    ));

    Ok(())
}

struct PlayHandler;

impl PlayHandler {
    pub fn new() -> Self {
        PlayHandler {}
    }
}

impl ButtonHandler for PlayHandler {
    fn on_click(&self, event_sender: &mut atlas::event_bus::EventSender) {
        event_sender.send(SceneEvent::NewScene("first_scene"))
    }
}

struct BackgroundHandler;

impl BackgroundHandler {
    pub fn new() -> Self {
        BackgroundHandler {}
    }
}

impl ButtonHandler for BackgroundHandler {
    fn on_click(&self, _: &mut atlas::event_bus::EventSender) {
        println!("Background");
    }
}

fn add_menu_entry(
    pos: Vec3,
    component_manager: &mut EntityManager,
    material: &Resource<SpriteMaterial>,
) {
    component_manager.add_entity((
        Transform::pos(pos),
        ShapeRenderer::quad((128.0, 32.0), material.res.clone()),
        ButtonTrigger::new(1, (pos.x, pos.y), (128.0, 32.0)),
        Box::new(PlayHandler::new()) as Box<dyn ButtonHandler>,
    ));
}
