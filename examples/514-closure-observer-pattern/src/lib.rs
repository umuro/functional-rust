//! Observer/Callback Pattern
//!
//! Event system using FnMut closures as handlers.

/// Event emitter that stores FnMut handlers.
pub struct EventEmitter<E> {
    handlers: Vec<Box<dyn FnMut(&E)>>,
}

impl<E> EventEmitter<E> {
    pub fn new() -> Self {
        EventEmitter {
            handlers: Vec::new(),
        }
    }

    /// Register a handler — returns its index for potential removal.
    pub fn subscribe(&mut self, handler: impl FnMut(&E) + 'static) -> usize {
        self.handlers.push(Box::new(handler));
        self.handlers.len() - 1
    }

    /// Emit an event — all handlers are called in registration order.
    pub fn emit(&mut self, event: &E) {
        for handler in &mut self.handlers {
            handler(event);
        }
    }

    /// Number of registered handlers.
    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

impl<E> Default for EventEmitter<E> {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple button events.
#[derive(Debug, Clone)]
pub enum ButtonEvent {
    Click { x: i32, y: i32 },
    Hover { x: i32, y: i32 },
    KeyPress(char),
}

/// Observable value that notifies on change.
pub struct Observable<T> {
    value: T,
    listeners: Vec<Box<dyn FnMut(&T, &T)>>, // (old, new)
}

impl<T: Clone> Observable<T> {
    pub fn new(value: T) -> Self {
        Observable {
            value,
            listeners: Vec::new(),
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, new_value: T) {
        let old = self.value.clone();
        self.value = new_value;
        for listener in &mut self.listeners {
            listener(&old, &self.value);
        }
    }

    pub fn on_change(&mut self, listener: impl FnMut(&T, &T) + 'static) {
        self.listeners.push(Box::new(listener));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_emitter_subscribe_emit() {
        let counter = Rc::new(RefCell::new(0));
        let counter_clone = counter.clone();

        let mut emitter: EventEmitter<i32> = EventEmitter::new();
        emitter.subscribe(move |_| {
            *counter_clone.borrow_mut() += 1;
        });

        emitter.emit(&42);
        emitter.emit(&43);

        assert_eq!(*counter.borrow(), 2);
    }

    #[test]
    fn test_emitter_multiple_handlers() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let log1 = log.clone();
        let log2 = log.clone();

        let mut emitter: EventEmitter<&str> = EventEmitter::new();
        emitter.subscribe(move |e| log1.borrow_mut().push(format!("h1:{}", e)));
        emitter.subscribe(move |e| log2.borrow_mut().push(format!("h2:{}", e)));

        emitter.emit(&"test");

        assert_eq!(*log.borrow(), vec!["h1:test", "h2:test"]);
    }

    #[test]
    fn test_emitter_handler_count() {
        let mut emitter: EventEmitter<()> = EventEmitter::new();
        assert_eq!(emitter.handler_count(), 0);
        emitter.subscribe(|_| {});
        assert_eq!(emitter.handler_count(), 1);
        emitter.subscribe(|_| {});
        assert_eq!(emitter.handler_count(), 2);
    }

    #[test]
    fn test_observable_set_notifies() {
        let changes = Rc::new(RefCell::new(Vec::new()));
        let changes_clone = changes.clone();

        let mut obs = Observable::new(10);
        obs.on_change(move |old, new| {
            changes_clone.borrow_mut().push((*old, *new));
        });

        obs.set(20);
        obs.set(30);

        assert_eq!(*changes.borrow(), vec![(10, 20), (20, 30)]);
    }

    #[test]
    fn test_observable_get() {
        let obs = Observable::new("hello");
        assert_eq!(*obs.get(), "hello");
    }

    #[test]
    fn test_button_events() {
        let clicks = Rc::new(RefCell::new(0));
        let clicks_clone = clicks.clone();

        let mut emitter: EventEmitter<ButtonEvent> = EventEmitter::new();
        emitter.subscribe(move |e| {
            if matches!(e, ButtonEvent::Click { .. }) {
                *clicks_clone.borrow_mut() += 1;
            }
        });

        emitter.emit(&ButtonEvent::Click { x: 10, y: 20 });
        emitter.emit(&ButtonEvent::Hover { x: 15, y: 25 });
        emitter.emit(&ButtonEvent::Click { x: 30, y: 40 });

        assert_eq!(*clicks.borrow(), 2);
    }
}
