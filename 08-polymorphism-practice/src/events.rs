//! Система событий: Copy/Clone, FnMut и Box<dyn Fn>.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    Click,
    Hover,
    Submit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub kind: EventKind,
    pub payload: String,
}

impl Event {
    pub fn new(kind: EventKind, payload: &str) -> Self {
        Self {
            kind,
            payload: payload.to_string(),
        }
    }
}

pub fn handle_once<F>(event: Event, handler: F)
where
    F: FnOnce(Event),
{
    handler(event);
}

pub fn process_stream<F>(events: Vec<Event>, mut handler: F)
where
    F: FnMut(Event),
{
    for event in events {
        handler(event);
    }
}

#[derive(Default)]
pub struct EventBus {
    handlers: HashMap<String, Box<dyn Fn(Event)>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on(&mut self, event: &str, handler: impl Fn(Event) + 'static) {
        self.handlers.insert(event.to_string(), Box::new(handler));
    }

    pub fn emit(&self, event_name: &str, payload: &str) {
        let event = Event::new(EventKind::Click, payload);
        if let Some(handler) = self.handlers.get(event_name) {
            handler(event.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn copy_and_clone_have_different_roles() {
        let kind = EventKind::Submit;
        let copied = kind;
        assert_eq!(kind, copied);

        let event = Event::new(kind, "form");
        let cloned = event.clone();
        assert_eq!(event, cloned);
    }

    #[test]
    fn fn_mut_handler_accumulates_state() {
        let mut clicks = 0;
        let mut log = Vec::new();
        process_stream(
            vec![
                Event::new(EventKind::Click, "btn-1"),
                Event::new(EventKind::Hover, "menu"),
                Event::new(EventKind::Click, "btn-2"),
            ],
            |event| {
                if event.kind == EventKind::Click {
                    clicks += 1;
                }
                log.push(event.kind);
            },
        );

        assert_eq!(clicks, 2);
        assert_eq!(log, [EventKind::Click, EventKind::Hover, EventKind::Click]);
    }

    #[test]
    fn event_bus_stores_dyn_handlers() {
        let received = Rc::new(RefCell::new(Vec::new()));
        let handler_log = Rc::clone(&received);
        let mut bus = EventBus::new();
        bus.on("click", move |event| {
            handler_log.borrow_mut().push(event.payload);
        });

        bus.emit("click", "button-1");
        bus.emit("unknown", "ignored");
        assert_eq!(*received.borrow(), ["button-1"]);
    }
}
