mod controller;
mod light;
mod menu_button;
mod mesh_renderer;
mod skybox;

use crate::{
    components::{
        button_handler::ButtonHandler, button_trigger::ButtonTrigger, camera::Camera,
        controller::Controller, mesh_renderer::MeshRenderer, shape_renderer::ShapeRenderer,
        skybox_renderer::SkyboxRenderer, transform::Transform,
    },
    graphics::{lights::Light, material::TextureMaterial, mesh::Mesh, shaders::MeshShader},
};

pub type ComponentIterator<T> = Box<dyn Iterator<Item = T>>;

pub enum ComponentEvent<T> {
    AddComponent(T),
    RemoveComponent(usize),
}

pub trait ComponentIteratorGenerator<'a, T> {
    fn iter(&'a self) -> Box<dyn Iterator<Item = T> + 'a>;
}

pub trait ComponentMutIteratorGenerator<'a, T> {
    fn iter(&'a mut self) -> Box<dyn Iterator<Item = T> + 'a>;
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
    meshes: (
        Vec<usize>,
        Vec<Transform>,
        Vec<Mesh<MeshShader, TextureMaterial>>,
    ),
    lights: (Vec<usize>, Vec<Transform>, Vec<Light>),

    next_entity_id: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        let menu_buttons = (vec![], vec![], vec![], vec![], vec![]);
        let skyboxes = (vec![], vec![]);
        let cameras = (vec![], vec![], vec![]);
        let meshes = (vec![], vec![], vec![]);
        let lights = (vec![], vec![], vec![]);

        EntityManager {
            next_entity_id: 0,
            menu_buttons,
            skyboxes,
            cameras,
            meshes,
            lights,
        }
    }

    pub fn get_camera(&self, id: usize) -> Option<&Camera> {
        let cam_ref = self.cameras.0.iter().position(|&cam_id| cam_id == id)?;
        Some(&self.cameras.2[cam_ref])
    }
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a ShapeRenderer)> for EntityManager {
    fn iter(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a ShapeRenderer)> + 'a> {
        let transforms = self.menu_buttons.1.iter();
        let shapes = self.menu_buttons.2.iter();
        Box::new(transforms.zip(shapes))
    }
}

impl<'a> ComponentIteratorGenerator<'a, ((usize, &'a ButtonTrigger), &'a dyn ButtonHandler)>
    for EntityManager
{
    fn iter(
        &'a self,
    ) -> Box<dyn Iterator<Item = ((usize, &'a ButtonTrigger), &'a dyn ButtonHandler)> + 'a> {
        let entities = self.menu_buttons.0.iter().map(|entity| *entity);
        let triggers = self.menu_buttons.3.iter();
        let handlers = self.menu_buttons.4.iter().map(|handler| handler.as_ref());
        Box::new(entities.zip(triggers).zip(handlers))
    }
}

pub type SkyboxBundle<'a> = (usize, &'a SkyboxRenderer);

impl<'a> ComponentIteratorGenerator<'a, SkyboxBundle<'a>> for EntityManager {
    fn iter(&'a self) -> Box<dyn Iterator<Item = SkyboxBundle<'a>> + 'a> {
        let entities = self.skyboxes.0.iter().map(|entity| *entity);
        let skyboxes = self.skyboxes.1.iter();
        Box::new(entities.zip(skyboxes))
    }
}

impl<'a> ComponentMutIteratorGenerator<'a, ((usize, &'a dyn Controller), &'a mut Camera)>
    for EntityManager
{
    fn iter(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = ((usize, &'a dyn Controller), &'a mut Camera)> + 'a> {
        let entities = self.cameras.0.iter().map(|entity| *entity);
        let controllers = self.cameras.1.iter().map(|controller| controller.as_ref());
        let cameras = self.cameras.2.iter_mut();
        Box::new(entities.zip(controllers).zip(cameras))
    }
}

impl<'a>
    ComponentIteratorGenerator<
        'a,
        (
            (usize, &'a Transform),
            &'a Mesh<MeshShader, TextureMaterial>,
        ),
    > for EntityManager
{
    fn iter(
        &'a self,
    ) -> Box<
        dyn Iterator<
                Item = (
                    (usize, &'a Transform),
                    &'a Mesh<MeshShader, TextureMaterial>,
                ),
            > + 'a,
    > {
        let entities = self.meshes.0.iter().map(|entity| *entity);
        let transforms = self.meshes.1.iter();
        let meshes = self.meshes.2.iter();
        Box::new(entities.zip(transforms).zip(meshes))
    }
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a Light)> for EntityManager {
    fn iter(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a Light)> + 'a> {
        let transforms = self.lights.1.iter();
        let lights = self.lights.2.iter();
        Box::new(transforms.zip(lights))
    }
}
