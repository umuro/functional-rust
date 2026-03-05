# OCaml vs Rust: Event Handler Pattern

## OCaml
```ocaml
type ui_event = Click of int * int | KeyPress of char | Scroll of float

type handler = {
  priority: int;
  name: string;
  handle: ui_event -> bool;
}

let dispatch handlers event =
  List.sort (fun a b -> compare a.priority b.priority) handlers
  |> List.exists (fun h -> h.handle event)
```

## Rust
```rust
pub struct Handler {
    pub priority: Priority,
    pub name: &'static str,
    handle: Box<dyn FnMut(&UiEvent) -> bool>,
}

impl EventDispatcher {
    pub fn dispatch(&mut self, event: &UiEvent) -> bool {
        for handler in &mut self.handlers {
            if handler.handle(event) { return true; }
        }
        false
    }
}
```

## Key Differences

1. **OCaml**: Handlers are simple record types with function fields
2. **Rust**: Box<dyn FnMut> for mutable closure handlers
3. Both: Support priority ordering and propagation control
4. **Rust**: FnMut allows stateful handlers
5. Both model event-driven systems with closure-based handlers
