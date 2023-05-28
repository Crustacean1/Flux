use std::mem;

use glam::Vec3;

use crate::{
    entity_manager::{ComponentMutIteratorGenerator, EntityManager, EntityManagerTrait},
    event_bus::{EventReader, EventReaderTrait},
    graphics::graphics_context::IoEvent,
};

use super::camera::Camera;

pub trait Controller {
    fn look(&mut self, movement: (f32, f32), camera: &mut Camera);
    fn translate(&mut self, movement: Vec3, camera: &mut Camera);
}

pub struct CameraControllerSystem;

impl CameraControllerSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_inputs(&self, event_reader: &EventReader, entity_manager: &mut EntityManager) {
        event_reader
            .read()
            .iter()
            .for_each(|io_event| match io_event {
                IoEvent::MouseMotion((x, y)) => {
                    Self::handle_pan((*x, *y), entity_manager);
                }
                IoEvent::KeyHeld(65) => {
                    /*A*/
                    Self::handle_movement(Vec3::new(0.1, 0.0, 0.0), entity_manager);
                }
                IoEvent::KeyHeld(87) => {
                    /*W*/
                    Self::handle_movement(Vec3::new(0.0, 0.0, 0.1), entity_manager);
                }
                IoEvent::KeyHeld(83) => {
                    /*S*/
                    Self::handle_movement(Vec3::new(0.0, 0.0, -0.1), entity_manager);
                }
                IoEvent::KeyHeld(68) => {
                    /*D*/
                    Self::handle_movement(Vec3::new(-0.1, 0.0, 0.0), entity_manager);
                }
                _ => {}
            });
    }

    fn handle_pan(pan: (f32, f32), entity_manager: &mut EntityManager) {
        entity_manager.iter().for_each(
            |((_, controller), camera): ((usize, &dyn Controller), &mut Camera)| {
                // TODO: figure better way of doing this shit
                let controller = Self::force_to_mut(controller);
                controller.look(pan, camera)
            },
        )
    }

    fn handle_movement(movement: Vec3, entity_manager: &mut EntityManager) {
        entity_manager.iter().for_each(
            |((_, controller), camera): ((usize, &dyn Controller), &mut Camera)| {
                let controller = Self::force_to_mut(controller);
                controller.translate(movement, camera)
            },
        )
    }

    fn force_to_mut(controller: &dyn Controller) -> &mut dyn Controller {
        unsafe {
            let controller: *const dyn Controller = controller;
            let controller: *mut dyn Controller = mem::transmute(controller);
            &mut *controller
        }
    }
}
