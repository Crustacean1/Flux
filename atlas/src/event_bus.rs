use std::{any::Any, cell::RefCell, mem, rc::Rc};

pub fn create_event_queue() -> (EventSender, EventReader) {
    let events = Rc::new(RefCell::new(Vec::new()));
    (
        EventSender {
            events: events.clone(),
        },
        EventReader { events },
    )
}

#[derive(Clone)]
pub struct EventSender {
    events: Rc<RefCell<Vec<Box<dyn Any>>>>,
}

pub struct EventReader {
    events: Rc<RefCell<Vec<Box<dyn Any>>>>,
}

impl EventReader {
    pub fn read<T: 'static>(&self, mut reader: impl FnMut(T) -> ()) {
        let mut events = self.events.borrow_mut();
        events.iter_mut().find_map(|queue| {
            queue.downcast_mut::<Vec<T>>().map(|queue| {
                queue.drain(0..queue.len()).for_each(|event| {
                    reader(event);
                })
            })
        });
    }
}

impl EventSender {
    pub fn write<T: 'static>(&self, event: T) {
        let mut events = self.events.borrow_mut();
        if let Some(queue) = events
            .iter_mut()
            .find_map(|queue| queue.downcast_mut::<Vec<T>>())
        {
            queue.push(event);
        } else {
            events.push(Box::new(vec![event]));
        }
    }
}

fn to_mut<T>(val: &T) -> &mut T {
    unsafe {
        let ptr: *const T = val;
        mem::transmute(ptr)
    }
}
