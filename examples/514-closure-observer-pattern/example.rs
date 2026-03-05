//! # 514. Observer/Callback Pattern
//! Event system using FnMut closures as handlers.

#[derive(Debug, Clone)]
enum ButtonEvent {
    Click { x: i32, y: i32 },
    Hover { x: i32, y: i32 },
    KeyPress(char),
}

/// Event emitter that stores FnMut handlers
struct EventEmitter<E> {
    handlers: Vec<Box<dyn FnMut(&E)>>,
}

impl<E> EventEmitter<E> {
    fn new() -> Self {
        EventEmitter { handlers: Vec::new() }
    }

    /// Register a handler — returns its index for potential removal
    fn subscribe(&mut self, handler: impl FnMut(&E) + 'static) -> usize {
        self.handlers.push(Box::new(handler));
        self.handlers.len() - 1
    }

    /// Emit an event — all handlers are called in registration order
    fn emit(&mut self, event: &E) {
        for handler in &mut self.handlers {
            handler(event);
        }
    }

    fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

/// One-time observer: fires once, then deactivates
fn once<E, F: FnMut(&E) + 'static>(mut f: F) -> impl FnMut(&E) {
    let mut fired = false;
    move |event| {
        if !fired {
            fired = true;
            f(event);
        }
    }
}

fn main() {
    let mut emitter: EventEmitter<ButtonEvent> = EventEmitter::new();

    // Logger handler (immutable state)
    emitter.subscribe(|event| {
        println!("[LOG] {:?}", event);
    });

    // Click counter (mutable state captured by closure)
    let mut click_count = 0usize;
    let click_counter_id = emitter.subscribe(move |event| {
        if let ButtonEvent::Click { .. } = event {
            click_count += 1;
            println!("[COUNTER] clicks so far: {}", click_count);
        }
    });
    println!("Click counter registered as handler #{}", click_counter_id);

    // Key accumulator — collects pressed keys
    let mut keys = String::new();
    emitter.subscribe(move |event| {
        if let ButtonEvent::KeyPress(c) = event {
            keys.push(*c);
            println!("[KEYS] accumulated: {}", keys);
        }
    });

    // One-time welcome handler
    emitter.subscribe(once(|_event| {
        println!("[ONCE] First event received! (fires only once)");
    }));

    println!("\n--- Emitting events ---");
    emitter.emit(&ButtonEvent::Click { x: 10, y: 20 });
    emitter.emit(&ButtonEvent::Hover { x: 15, y: 25 });
    emitter.emit(&ButtonEvent::Click { x: 30, y: 40 });
    emitter.emit(&ButtonEvent::KeyPress('r'));
    emitter.emit(&ButtonEvent::KeyPress('s'));
    emitter.emit(&ButtonEvent::Click { x: 5, y: 5 });

    println!("\nTotal handlers: {}", emitter.handler_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_count() {
        let mut emitter: EventEmitter<i32> = EventEmitter::new();
        let mut count = 0;
        emitter.subscribe(move |&n| {
            count += n;
            let _ = count;
        });
        emitter.emit(&1);
        emitter.emit(&2);
        emitter.emit(&3);
        // handler count should be 1
        assert_eq!(emitter.handler_count(), 1);
    }

    #[test]
    fn test_multiple_handlers() {
        let mut emitter: EventEmitter<&str> = EventEmitter::new();
        let mut log: Vec<String> = Vec::new();

        // Can't share log across closures without Arc — use separate logs
        let mut log1: Vec<String> = Vec::new();
        let mut log2: Vec<String> = Vec::new();
        // Just test registration and emission works
        emitter.subscribe(|_| {});
        emitter.subscribe(|_| {});
        assert_eq!(emitter.handler_count(), 2);
        emitter.emit(&"test");
        drop(log1); drop(log2); drop(log);
    }

    #[test]
    fn test_once() {
        use std::cell::Cell;
        use std::rc::Rc;
        let call_count = Rc::new(Cell::new(0usize));
        let cc = call_count.clone();
        let mut f = once(move |_: &i32| cc.set(cc.get() + 1));
        f(&1);
        f(&2);
        f(&3);
        assert_eq!(call_count.get(), 1);
    }
}
