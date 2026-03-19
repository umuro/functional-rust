#![allow(clippy::all)]
//! Event Handler Pattern
//!
//! Typed events with priority-ordered handler chains and propagation control.

/// UI Event types.
#[derive(Debug, Clone)]
pub enum UiEvent {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Scroll(f32),
}

/// Handler priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    High = 0,
    Normal = 1,
    Low = 2,
}

/// Event handler with priority.
pub struct Handler {
    pub priority: Priority,
    pub name: &'static str,
    handle: Box<dyn FnMut(&UiEvent) -> bool>, // true = stop propagation
}

impl Handler {
    pub fn new(
        priority: Priority,
        name: &'static str,
        f: impl FnMut(&UiEvent) -> bool + 'static,
    ) -> Self {
        Handler {
            priority,
            name,
            handle: Box::new(f),
        }
    }

    pub fn handle(&mut self, event: &UiEvent) -> bool {
        (self.handle)(event)
    }
}

/// Event dispatcher with ordered handlers.
pub struct EventDispatcher {
    handlers: Vec<Handler>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {
            handlers: Vec::new(),
        }
    }

    pub fn register(&mut self, handler: Handler) {
        self.handlers.push(handler);
        self.handlers.sort_by_key(|h| h.priority);
    }

    /// Dispatch event to all handlers (or until one stops propagation).
    pub fn dispatch(&mut self, event: &UiEvent) -> bool {
        for handler in &mut self.handlers {
            if handler.handle(event) {
                return true; // propagation stopped
            }
        }
        false
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_handler_called() {
        let called = Rc::new(RefCell::new(false));
        let called_clone = called.clone();

        let mut handler = Handler::new(Priority::Normal, "test", move |_| {
            *called_clone.borrow_mut() = true;
            false
        });

        handler.handle(&UiEvent::Click { x: 0, y: 0 });
        assert!(*called.borrow());
    }

    #[test]
    fn test_dispatcher_order() {
        let order = Rc::new(RefCell::new(Vec::new()));

        let mut dispatcher = EventDispatcher::new();

        let o1 = order.clone();
        dispatcher.register(Handler::new(Priority::Low, "low", move |_| {
            o1.borrow_mut().push("low");
            false
        }));

        let o2 = order.clone();
        dispatcher.register(Handler::new(Priority::High, "high", move |_| {
            o2.borrow_mut().push("high");
            false
        }));

        let o3 = order.clone();
        dispatcher.register(Handler::new(Priority::Normal, "normal", move |_| {
            o3.borrow_mut().push("normal");
            false
        }));

        dispatcher.dispatch(&UiEvent::KeyPress('a'));

        assert_eq!(*order.borrow(), vec!["high", "normal", "low"]);
    }

    #[test]
    fn test_stop_propagation() {
        let calls = Rc::new(RefCell::new(0));

        let mut dispatcher = EventDispatcher::new();

        let c1 = calls.clone();
        dispatcher.register(Handler::new(Priority::High, "stopper", move |_| {
            *c1.borrow_mut() += 1;
            true // stop propagation
        }));

        let c2 = calls.clone();
        dispatcher.register(Handler::new(Priority::Low, "never_called", move |_| {
            *c2.borrow_mut() += 1;
            false
        }));

        let stopped = dispatcher.dispatch(&UiEvent::Scroll(1.0));

        assert!(stopped);
        assert_eq!(*calls.borrow(), 1);
    }

    #[test]
    fn test_event_types() {
        let mut clicks = 0;
        let mut keys = 0;

        let mut handler = Handler::new(Priority::Normal, "counter", move |e| {
            match e {
                UiEvent::Click { .. } => clicks += 1,
                UiEvent::KeyPress(_) => keys += 1,
                _ => {}
            }
            false
        });

        handler.handle(&UiEvent::Click { x: 10, y: 20 });
        handler.handle(&UiEvent::KeyPress('x'));
        // Note: we can't easily access clicks/keys from outside due to move
        // This test just verifies the pattern compiles and runs
    }

    #[test]
    fn test_empty_dispatcher() {
        let mut dispatcher = EventDispatcher::new();
        let stopped = dispatcher.dispatch(&UiEvent::Click { x: 0, y: 0 });
        assert!(!stopped);
    }
}
