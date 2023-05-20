use crate::event_bus::EventSender;

pub trait ButtonHandler {
    fn on_click(&self, event_sender: &mut EventSender);
}
