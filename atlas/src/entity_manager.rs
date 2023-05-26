mod controller;
mod menu_button;
mod mesh_renderer;
mod skybox;

use crate::components::{
    button_handler::ButtonHandler, button_trigger::ButtonTrigger, camera::Camera,
    controller::Controller, mesh_renderer::MeshRenderer, shape_renderer::ShapeRenderer,
    skybox_renderer::SkyboxRenderer, transform::Transform,
};

pub enum ComponentEvent<T> {
    AddComponent(T),
    RemoveComponent(usize),
}

pub trait ComponentIterator<'a, T> {
    fn iter(&self) -> &[T];
    fn reload(&mut self);
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
    meshes: (Vec<usize>, Vec<Transform>, Vec<MeshRenderer>),

    transform_shape_iter: Vec<((usize, *const Transform), *const ShapeRenderer)>,
    trigger_handler_iter: Vec<((usize, *const ButtonTrigger), *const dyn ButtonHandler)>,
    skybox_iter: Vec<(usize, *const SkyboxRenderer)>,
    camera_controller_iter: Vec<((usize, *const (dyn Controller)), *mut Camera)>,
    transform_mesh_iter: Vec<((usize, *const Transform), *const MeshRenderer)>,

    next_entity_id: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        let menu_buttons = (vec![], vec![], vec![], vec![], vec![]);
        let skyboxes = (vec![], vec![]);
        let cameras = (vec![], vec![], vec![]);
        let meshes = (vec![], vec![], vec![]);

        EntityManager {
            next_entity_id: 0,
            menu_buttons,
            skyboxes,
            cameras,
            meshes,
            transform_shape_iter: vec![],
            trigger_handler_iter: vec![],
            skybox_iter: vec![],
            camera_controller_iter: vec![],
            transform_mesh_iter: vec![],
        }
    }

    pub fn get_camera(&self, id: usize) -> Option<&Camera> {
        let cam_ref = self.cameras.0.iter().position(|&cam_id| cam_id == id)?;
        Some(&self.cameras.2[cam_ref])
    }
}

impl<'a> ComponentIterator<'a, ((usize, *const Transform), *const ShapeRenderer)>
    for EntityManager
{
    fn iter(&self) -> &[((usize, *const Transform), *const ShapeRenderer)] {
        &self.transform_shape_iter
    }

    fn reload(&mut self) {
        let entities = self.menu_buttons.0.iter().map(|e| *e);
        let transforms = self.menu_buttons.1.iter().map(|t| t as *const Transform);
        let shapes = self
            .menu_buttons
            .2
            .iter()
            .map(|s| s as *const ShapeRenderer);
        self.transform_shape_iter = entities.zip(transforms).zip(shapes).collect()
    }
}

impl<'a>
    ComponentIterator<
        'a,
        (
            (usize, *const ButtonTrigger),
            *const (dyn ButtonHandler + 'a),
        ),
    > for EntityManager
{
    fn iter(
        &self,
    ) -> &[(
        (usize, *const ButtonTrigger),
        *const (dyn ButtonHandler + 'a),
    )] {
        &self.trigger_handler_iter
    }

    fn reload(&mut self) {
        let entities = self.menu_buttons.0.iter().map(|e| *e);
        let triggers = self
            .menu_buttons
            .3
            .iter()
            .map(|t| t as *const ButtonTrigger);
        let handlers = self
            .menu_buttons
            .4
            .iter()
            .map(|h| &**h as *const (dyn ButtonHandler));

        self.trigger_handler_iter = entities.zip(triggers).zip(handlers).collect();
    }
}

pub type SkyboxBundle = (usize, *const SkyboxRenderer);

impl<'a> ComponentIterator<'a, SkyboxBundle> for EntityManager {
    fn iter(&self) -> &[(usize, *const SkyboxRenderer)] {
        &self.skybox_iter
    }

    fn reload(&mut self) {
        let entities = self.skyboxes.0.iter().map(|e| *e);
        let skyboxes = self.skyboxes.1.iter().map(|s| s as *const SkyboxRenderer);

        self.skybox_iter = entities.zip(skyboxes).collect();
    }
}

impl<'a> ComponentIterator<'a, ((usize, *const (dyn Controller + 'a)), *mut Camera)>
    for EntityManager
{
    fn iter(&self) -> &[((usize, *const (dyn Controller + 'a)), *mut Camera)] {
        &self.camera_controller_iter
    }

    fn reload(&mut self) {
        let entities = self.cameras.0.iter().map(|e| *e);
        let controller = self.cameras.1.iter_mut().map(|c| {
            return (*c).as_mut() as *const dyn Controller;
        });
        let cameras = self.cameras.2.iter_mut().map(|c| c as *mut Camera);

        self.camera_controller_iter = entities.zip(controller).zip(cameras).collect();
    }
}

impl<'a> ComponentIterator<'a, ((usize, *const Transform), *const MeshRenderer)> for EntityManager {
    fn iter(&self) -> &[((usize, *const Transform), *const MeshRenderer)] {
        &self.transform_mesh_iter
    }

    fn reload(&mut self) {
        let entities = self.meshes.0.iter().map(|e| *e);
        let transforms = self.meshes.1.iter().map(|t| t as *const Transform);
        let renderers = self.meshes.2.iter().map(|r| r as *const MeshRenderer);

        self.transform_mesh_iter = entities.zip(transforms).zip(renderers).collect();
    }
}
