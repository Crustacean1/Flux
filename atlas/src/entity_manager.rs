mod controller;
mod menu_button;
mod skybox;

use crate::components::{
    button_handler::ButtonHandler, button_trigger::ButtonTrigger, camera::Camera,
    controller::Controller, shape_renderer::ShapeRenderer, skybox_renderer::SkyboxRenderer,
    transform::Transform,
};

pub enum ComponentEvent<T> {
    AddComponent(T),
    RemoveComponent(usize),
}

pub trait ComponentIterator<'a, T> {
    fn iter(&'a self) -> &[T];
}

pub trait EntityManagerTrait<T> {
    fn add_entity(&mut self, entity: T) -> usize;
    fn remove_entity(&mut self, entity: usize);
}

pub struct EntityManager {
    menu_buttons: (
        Vec<usize>,
        Vec<Transform>,
        Vec<ShapeRenderer>,
        Vec<ButtonTrigger>,
        Vec<Box<dyn ButtonHandler>>,
    ),
    skyboxes: (Vec<usize>, Vec<SkyboxRenderer>),
    cameras: (Vec<usize>, Vec<Box<dyn Controller>>, Vec<Camera>),

    transform_shape_iter: Vec<(usize, *const Transform, *const ShapeRenderer)>,
    trigger_handler_iter: Vec<(usize, *const ButtonTrigger, *const dyn ButtonHandler)>,
    skybox_iter: Vec<(usize, *const SkyboxRenderer)>,
    camera_controller_iter: Vec<(usize, *const (dyn Controller), *mut Camera)>,

    next_entity_id: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        let menu_buttons = (vec![], vec![], vec![], vec![], vec![]);
        let skyboxes = (vec![], vec![]);
        let cameras = (vec![], vec![], vec![]);

        EntityManager {
            menu_buttons,
            skyboxes,
            next_entity_id: 0,
            transform_shape_iter: vec![],
            trigger_handler_iter: vec![],
            skybox_iter: vec![],
            camera_controller_iter: vec![],
            cameras,
        }
    }

    pub fn get_camera(&self, id: usize) -> Option<&Camera> {
        let cam_ref = self.cameras.0.iter().position(|&cam_id| cam_id == id)?;
        Some(&self.cameras.2[cam_ref])
    }
}

impl<'a> ComponentIterator<'a, (usize, *const Transform, *const ShapeRenderer)> for EntityManager {
    fn iter(&'a self) -> &'a [(usize, *const Transform, *const ShapeRenderer)] {
        let transforms = self.menu_buttons.1.iter();
        let shapes = self.menu_buttons.2.iter();
        &self.transform_shape_iter
    }
}

impl<'a> ComponentIterator<'a, (usize, *const ButtonTrigger, *const (dyn ButtonHandler + 'a))>
    for EntityManager
{
    fn iter(&'a self) -> &[(usize, *const ButtonTrigger, *const (dyn ButtonHandler + 'a))] {
        &self.trigger_handler_iter
    }
}

pub type Skybox<'a> = &'a SkyboxRenderer;

impl<'a> ComponentIterator<'a, (usize, *const SkyboxRenderer)> for EntityManager {
    fn iter(&'a self) -> &[(usize, *const SkyboxRenderer)] {
        &self.skybox_iter
    }
}

impl<'a> ComponentIterator<'a, (usize, *const (dyn Controller + 'a), *mut Camera)>
    for EntityManager
{
    fn iter(&'a self) -> &[(usize, *const (dyn Controller + 'a), *mut Camera)] {
        &self.camera_controller_iter
    }
}
