//! # 523. Event Handler Pattern
//! Typed events with priority-ordered handler chains and propagation control.

#[derive(Debug, Clone)]
enum UiEvent {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Scroll(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Priority { High = 0, Normal = 1, Low = 2 }

struct Handler {
    priority: Priority,
    name: &'static str,
    handle: Box<dyn FnMut(&UiEvent) -> bool>, // true = stop propagation
}

impl Handler {
    fn new(priority: Priority, name: &'static str, f: impl FnMut(&UiEvent) -> bool + 'static) -> Self {
        Handler { priority, name, handle: Box::new(f) }
    }
}

struct EventBus {
    handlers: Vec<Handler>,
}

impl EventBus {
    fn new() -> Self { EventBus { handlers: Vec::new() } }

    fn register(&mut self, handler: Handler) {
        self.handlers.push(handler);
        self.handlers.sort_by_key(|h| h.priority);
    }

    fn dispatch(&mut self, event: &UiEvent) {
        println!("Dispatching {:?}", event);
        for handler in &mut self.handlers {
            let stop = (handler.handle)(event);
            println!("  [{}] handled, stop={}", handler.name, stop);
            if stop { break; }
        }
    }
}

fn main() {
    let mut bus = EventBus::new();

    // High priority: bounds checker
    bus.register(Handler::new(Priority::High, "bounds-checker", |event| {
        if let UiEvent::Click { x, y } = event {
            if *x < 0 || *y < 0 {
                println!("    -> Out of bounds! ({}, {})", x, y);
                return true; // stop propagation
            }
        }
        false
    }));

    // Normal priority: click handler with state
    let mut click_count = 0usize;
    bus.register(Handler::new(Priority::Normal, "click-counter", move |event| {
        if let UiEvent::Click { .. } = event {
            click_count += 1;
            println!("    -> Click #{}", click_count);
        }
        false
    }));

    // Low priority: fallback logger
    bus.register(Handler::new(Priority::Low, "logger", |event| {
        println!("    -> [LOG] {:?}", event);
        false
    }));

    bus.dispatch(&UiEvent::Click { x: 10, y: 20 });
    bus.dispatch(&UiEvent::Click { x: -1, y: 5 }); // bounds-checker stops propagation
    bus.dispatch(&UiEvent::KeyPress('r'));
    bus.dispatch(&UiEvent::Scroll(2.5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_order() {
        assert!(Priority::High < Priority::Normal);
        assert!(Priority::Normal < Priority::Low);
    }

    #[test]
    fn test_handler_registration() {
        let mut bus = EventBus::new();
        bus.register(Handler::new(Priority::Normal, "test", |_| false));
        assert_eq!(bus.handlers.len(), 1);
    }
}
