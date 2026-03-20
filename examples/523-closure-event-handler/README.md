📖 **[View on hightechmind.io →](https://hightechmind.io/rust/523-closure-event-handler)**

---

# Event Handler Pattern
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Event-driven programming is the foundation of GUI toolkits (GTK, Qt, winit), async runtimes (Tokio, async-std), and game engines (Bevy). The central challenge is routing typed events to the correct handlers in priority order, while allowing handlers to stop propagation — preventing lower-priority handlers from seeing an event already consumed by a higher-priority one. This example implements a typed event dispatcher with priority-ordered handler chains using closures as the handler mechanism.

## Learning Outcomes

- How `FnMut(&UiEvent) -> bool` closures model stateful handlers with propagation control
- How to sort handlers by a `Priority` enum using `Ord` derivation
- How to implement a dispatcher that calls handlers in priority order and respects stop-propagation
- How typed event enums (`UiEvent`) enable exhaustive handling with `match`
- Where this pattern appears in real frameworks: winit, egui, Bevy's event system

## Rust Application

`Handler` stores a `Box<dyn FnMut(&UiEvent) -> bool>` alongside its `Priority` and name. `EventDispatcher` holds a `Vec<Handler>` sorted by priority on each `dispatch` call. Handlers returning `true` stop further propagation. The `UiEvent` enum with `Click { x, y }`, `KeyPress(char)`, and `Scroll(f32)` variants shows how typed events enable exhaustive matching inside handlers. `Priority` derives `Ord` enabling `handlers.sort_by_key(|h| h.priority)`.

Key patterns:
- `Box<dyn FnMut(&UiEvent) -> bool>` — type-erased stateful handler
- Returning `bool` from handlers for propagation control
- Priority-based sort with `Ord`-derived enum

## OCaml Approach

OCaml event systems use lists of `priority * (event -> bool)` tuples sorted by priority. A dispatch function applies each handler in order and stops when one returns `true`. The `event` type is typically a variant type matching Rust's enum.

```ocaml
type event = Click of int * int | KeyPress of char | Scroll of float
type priority = High | Normal | Low
let dispatch handlers event =
  List.sort (fun (p1,_) (p2,_) -> compare p1 p2) handlers
  |> List.exists (fun (_,h) -> h event)
```

## Key Differences

1. **Handler state**: Rust uses `FnMut` to allow handlers to maintain state between events (e.g., drag tracking); OCaml would use `ref` inside the closure for the same effect.
2. **Priority ordering**: Rust derives `Ord` on an enum to enable `sort_by_key`; OCaml would use `compare` on a custom type or manual ordering.
3. **Type-erased storage**: Rust's `Box<dyn FnMut>` erases the concrete closure type; OCaml closures are already type-erased at the value level, no boxing annotation required.
4. **Stop-propagation**: Rust models this as `bool` in the return type — expressive and visible; OCaml uses the same approach or raises an exception for non-local control flow.

## Exercises

1. **Async handlers**: Modify `EventDispatcher` to store `Box<dyn FnMut(&UiEvent) -> Pin<Box<dyn Future<Output = bool>>>>` and implement an async dispatch loop.
2. **Wildcard handler**: Add a `subscribe_all(handler: impl FnMut(&UiEvent) -> bool + 'static)` that registers a handler for every event type regardless of variant.
3. **Handler removal by name**: Implement `remove(name: &str)` on `EventDispatcher` that finds and removes the first handler with a matching name, returning whether one was found.
