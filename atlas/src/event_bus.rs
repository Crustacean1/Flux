use std::{any::{Any, TypeId}, mem, rc::Rc };

pub fn create_event_queue() -> (EventSender, EventReader) {
    let events = Rc::new(Vec::new());
    (
        EventSender {
            events: events.clone(),
        },
        EventReader { events },
    )
}

pub struct EventSender {
    events: Rc<Vec<Box<dyn Any>>>,
}

pub struct EventReader {
    events: Rc<Vec<Box<dyn Any>>>,
}

impl EventReader {
    pub fn read<T: 'static>(&mut self) -> Option<impl Iterator<Item = T> + '_> {
        let events = to_mut(self.events.as_ref());
        events.iter_mut().find_map(|queue| {
            queue
                .downcast_mut::<Vec<T>>()
                .map(|queue| queue.drain(0..queue.len()))
        })
    }
}

impl EventSender {
    pub fn write<T: 'static>(&mut self, event: T) {
        let events = to_mut(self.events.as_ref());
        if let Some(queue) = events
            .iter_mut()
            .find_map(|queue| queue.downcast_mut::<Vec<T>>())
        {
            queue.push(event);
        } else {
            events.push(Box::new(vec![event]));
            println!("Adding event type");
        }
    }
}

fn to_mut<T>(val: &T) -> &mut T {
    unsafe {
        let ptr: *const T = val;
        mem::transmute(ptr)
    }
}
