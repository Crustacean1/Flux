use std::mem;

use crate::{
    components::button_trigger::UiEvent,
    graphics::graphics_context::{ContextEvent, IoEvent},
    scene::SceneEvent,
};

pub trait EventSenderTrait<T> {
    fn send(&mut self, msg: T);
}

pub trait EventReaderTrait<T> {
    fn read(&self) -> &[T];
}

pub struct EventSender {
    io_events: Vec<IoEvent>,
    scene_events: Vec<SceneEvent>,
    context_events: Vec<ContextEvent>,
    ui_events: Vec<UiEvent>,
}

impl EventSender {
    pub fn new() -> Self {
        EventSender {
            io_events: vec![],
            scene_events: vec![],
            context_events: vec![],
            ui_events: vec![],
        }
    }
}

pub struct EventReader {
    io_events: Vec<IoEvent>,
    scene_events: Vec<SceneEvent>,
    context_events: Vec<ContextEvent>,
    ui_events: Vec<UiEvent>,
}

impl EventReader {
    pub fn new() -> Self {
        EventReader {
            io_events: vec![],
            scene_events: vec![],
            context_events: vec![],
            ui_events: vec![],
        }
    }
}

pub fn swap_event_buffers(event_reader: &mut EventReader, event_sender: &mut EventSender) {
    mem::swap(&mut event_reader.io_events, &mut event_sender.io_events);
    mem::swap(&mut event_reader.ui_events, &mut event_sender.ui_events);
    mem::swap(
        &mut event_reader.scene_events,
        &mut event_sender.scene_events,
    );
    mem::swap(
        &mut event_reader.context_events,
        &mut event_sender.context_events,
    );

    event_sender.ui_events.clear();
    event_sender.context_events.clear();
    event_sender.scene_events.clear();
    event_sender.io_events.clear();
}

impl EventSenderTrait<IoEvent> for EventSender {
    fn send(&mut self, msg: IoEvent) {
        self.io_events.push(msg);
    }
}
impl EventSenderTrait<SceneEvent> for EventSender {
    fn send(&mut self, msg: SceneEvent) {
        self.scene_events.push(msg);
    }
}
impl EventSenderTrait<ContextEvent> for EventSender {
    fn send(&mut self, msg: ContextEvent) {
        self.context_events.push(msg);
    }
}
impl EventSenderTrait<UiEvent> for EventSender {
    fn send(&mut self, msg: UiEvent) {
        self.ui_events.push(msg);
    }
}

impl EventReaderTrait<IoEvent> for EventReader {
    fn read(&self) -> &[IoEvent] {
        &self.io_events
    }
}
impl EventReaderTrait<SceneEvent> for EventReader {
    fn read(&self) -> &[SceneEvent] {
        &self.scene_events
    }
}
impl EventReaderTrait<ContextEvent> for EventReader {
    fn read(&self) -> &[ContextEvent] {
        &self.context_events
    }
}
impl EventReaderTrait<UiEvent> for EventReader {
    fn read(&self) -> &[UiEvent] {
        &self.ui_events
    }
}
