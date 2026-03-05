# OCaml vs Rust: Observer Pattern

## OCaml
```ocaml
type 'e emitter = {
  mutable handlers: ('e -> unit) list
}

let subscribe emitter handler =
  emitter.handlers <- handler :: emitter.handlers

let emit emitter event =
  List.iter (fun h -> h event) emitter.handlers
```

## Rust
```rust
pub struct EventEmitter<E> {
    handlers: Vec<Box<dyn FnMut(&E)>>,
}

impl<E> EventEmitter<E> {
    pub fn subscribe(&mut self, handler: impl FnMut(&E) + 'static) {
        self.handlers.push(Box::new(handler));
    }

    pub fn emit(&mut self, event: &E) {
        for handler in &mut self.handlers { handler(event); }
    }
}
```

## Key Differences

1. **OCaml**: Mutable list of handlers, immutable closures
2. **Rust**: FnMut allows handlers to mutate their captured state
3. Both: Closures stored in collections for later invocation
4. **Rust**: Explicit lifetime bounds with 'static
5. Both enable decoupled event-driven architectures
