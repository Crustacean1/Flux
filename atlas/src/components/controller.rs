use std::mem;

use glam::Vec3;

use crate::{
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

    pub fn read_inputs(
        &self,
        event_reader: &EventReader,
        controller_pairs: &[(usize, *const dyn Controller, *mut Camera)],
    ) {
        event_reader
            .read()
            .iter()
            .for_each(|io_event| match io_event {
                IoEvent::MouseMotion((x, y)) => unsafe {
                    Self::handle_pan((*x, *y), mem::transmute(controller_pairs));
                },
                IoEvent::KeyHeld(65) => unsafe {
                    /*A*/
                    Self::handle_movement(
                        Vec3::new(0.1, 0.0, 0.0),
                        mem::transmute(controller_pairs),
                    );
                },
                IoEvent::KeyHeld(87) => unsafe {
                    /*W*/
                    Self::handle_movement(
                        Vec3::new(0.0, 0.0, 0.1),
                        mem::transmute(controller_pairs),
                    );
                },
                IoEvent::KeyHeld(83) => unsafe {
                    /*S*/
                    Self::handle_movement(
                        Vec3::new(0.0, 0.0, -0.1),
                        mem::transmute(controller_pairs),
                    );
                },
                IoEvent::KeyHeld(68) => unsafe {
                    /*D*/
                    Self::handle_movement(
                        Vec3::new(-0.1, 0.0, 0.0),
                        mem::transmute(controller_pairs),
                    );
                },
                _ => {}
            });
    }

    fn handle_pan(pan: (f32, f32), controller_pairs: &[(usize, *mut dyn Controller, *mut Camera)]) {
        controller_pairs
            .iter()
            .for_each(|(_, controller, camera)| unsafe {
                let camera = &mut **camera;
                let controller = &mut **controller;
                controller.look(pan, camera)
            });
    }

    fn handle_movement(
        movement: Vec3,
        controller_pairs: &[(usize, *mut dyn Controller, *mut Camera)],
    ) {
        controller_pairs
            .iter()
            .for_each(|(_, controller, camera)| unsafe {
                let camera = &mut **camera;
                let controller = &mut **controller;
                controller.translate(movement, camera);
            })
    }
}
