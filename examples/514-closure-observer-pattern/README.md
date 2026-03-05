# 514: Observer/Callback Pattern

**Difficulty:** 3  **Level:** Intermediate

Register closures as event handlers that fire with state — no interface boilerplate, just `Box<dyn FnMut>`.

## The Problem This Solves

You're building a UI widget, a game event system, or a data pipeline that needs to notify multiple parties when something happens. The classic Observer pattern requires an `Observer` trait, a registration method, and concrete implementors. For simple callbacks — "log this", "count that", "update the UI" — you end up with a file-per-observer.

In C, callbacks use `void (*callback)(void* userdata)` — you manually pass context around through opaque pointers. Type safety is gone. In Java, anonymous inner classes or lambdas solve this but still need `@FunctionalInterface` declarations.

Rust closures with `FnMut` give you stateful callbacks that *carry their own context*. The counter closure remembers its count. The accumulator closure remembers what it's accumulated. No userdata pointers, no separate context structs.

## The Intuition

Think of an event emitter as a mailing list. When you subscribe (`subscribe(handler)`), you add your email address. When an event fires (`emit(event)`), everyone on the list gets a copy. Each subscriber (closure) independently decides what to do with the event.

The critical trait choice: `FnMut` rather than `Fn`. Observers are often *stateful* — they count events, accumulate data, update local caches. `FnMut` allows mutation of captured variables on each call. `Fn` would require all observers to be read-only.

In JavaScript, `element.addEventListener('click', (e) => count++)` is exactly this pattern. The arrow function captures `count` and mutates it. Rust's equivalent: a `FnMut` closure.

## How It Works in Rust

```rust
struct EventEmitter<E> {
    handlers: Vec<Box<dyn FnMut(&E)>>,   // FnMut: handlers can have state
}

impl<E> EventEmitter<E> {
    fn new() -> Self { EventEmitter { handlers: Vec::new() } }

    fn subscribe(&mut self, handler: impl FnMut(&E) + 'static) -> usize {
        self.handlers.push(Box::new(handler));
        self.handlers.len() - 1   // return ID for potential removal
    }

    fn emit(&mut self, event: &E) {
        for handler in &mut self.handlers {
            handler(event);        // FnMut: &mut self required on call
        }
    }
}

let mut emitter: EventEmitter<ButtonEvent> = EventEmitter::new();

// Stateless logger
emitter.subscribe(|event| println!("[LOG] {:?}", event));

// Stateful click counter — captures mut count by move
let mut click_count = 0usize;
emitter.subscribe(move |event| {
    if let ButtonEvent::Click { .. } = event {
        click_count += 1;                          // mutates captured state
        println!("Click #{}", click_count);
    }
});

// One-time handler: wraps any FnMut to fire only once
fn once<E, F: FnMut(&E) + 'static>(mut f: F) -> impl FnMut(&E) {
    let mut fired = false;
    move |event| {
        if !fired { fired = true; f(event); }
    }
}
emitter.subscribe(once(|_| println!("First event only!")));

emitter.emit(&ButtonEvent::Click { x: 10, y: 20 });
emitter.emit(&ButtonEvent::Click { x: 30, y: 40 });
// "Click #1", "Click #2", "First event only!" fires only once
```

## What This Unlocks

- **UI event systems** — button clicks, hover events, and keyboard input with stateful handlers that don't need shared mutable state across handlers.
- **Plugin hooks** — register `Box<dyn FnMut>` handlers for lifecycle events; each plugin maintains its own state in its closure's captures.
- **Reactive data pipelines** — emit data events; handlers filter, transform, and store results independently.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Event handler type | `'event -> unit` function | `Box<dyn FnMut(&Event)>` |
| Stateful handler | `ref` variable captured in closure | `FnMut` — mutable captures |
| Register handler | `handlers := f :: !handlers` | `handlers.push(Box::new(f))` |
| Fire event | `List.iter (fun h -> h event) !handlers` | `handlers.iter_mut().for_each(\|h\| h(&event))` |
| Thread-safe version | Mutex around handler list | `Box<dyn Fn + Send + Sync>` + `Mutex` |
