use glam::{Vec2, Vec3};

use crate::{
    entity_manager::{self, ComponentIteratorGenerator, EntityManager},
    event_bus::{EventReader, EventReaderTrait, EventSender},
    graphics::graphics_context::IoEvent,
};

use super::button_handler::ButtonHandler;

pub struct Rect {
    pos: Vec2,
    size: Vec2,
}

impl Rect {
    pub fn centered_quad(pos: (f32, f32), sides: (f32, f32)) -> Self {
        Rect {
            pos: Vec2::new(pos.0 - sides.0, pos.1 - sides.1),
            size: Vec2::new(sides.0 * 2.0, sides.1 * 2.0),
        }
    }
}

#[derive(Clone)]
pub enum UiEvent {
    Pressed(String),
}

pub struct ButtonTrigger {
    bounding_rect: Rect,
    level: usize,
}

impl ButtonTrigger {
    pub fn new(level: usize, pos: (f32, f32), size: (f32, f32)) -> Self {
        ButtonTrigger {
            level,
            bounding_rect: Rect::centered_quad(pos, size),
        }
    }

    pub fn intersects(&self, x: f32, y: f32) -> bool {
        let (mut x, mut y) = (x, y);
        x -= self.bounding_rect.pos.x;
        y -= self.bounding_rect.pos.y;
        x >= 0.0 && y >= 0.0 && x <= self.bounding_rect.size.x && y <= self.bounding_rect.size.y
    }
}

pub struct ButtonTriggerSystem {}

impl ButtonTriggerSystem {
    pub fn new() -> Self {
        ButtonTriggerSystem {}
    }

    pub fn check_buttons<'a>(
        &self,
        entity_manager: &EntityManager,
        event_reader: &EventReader,
        event_sender: &mut EventSender,
    ) {
        let Some(IoEvent::LeftMousePress(click_pos)) = event_reader.read().iter().find(|e| match e {
            IoEvent::LeftMousePress(_) => true,
            _ => false,
        }) else {return;};
        Self::check_buttons_for_event(entity_manager, *click_pos, event_sender);
    }

    pub fn check_buttons_for_event<'a>(
        entity_manager: &EntityManager,
        (x, y): (f32, f32),
        event_sender: &mut EventSender,
    ) {
    }
}
