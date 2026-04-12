📖 **[View on hightechmind.io →](https://hightechmind.io/rust/514-closure-observer-pattern)**

---

# Closure Observer Pattern
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The observer (publish-subscribe) pattern originated in GUI frameworks of the 1980s and became one of the Gang of Four behavioral design patterns. The core problem: how do you notify multiple independent components when a value changes, without tightly coupling them together? Traditional OOP solutions require implementing an `EventListener` interface and registering objects. Closures eliminate the need for a class hierarchy — any callable that matches the signature can be registered as a handler, making the pattern far more composable and concise.

## Learning Outcomes

- How `FnMut` closures serve as stateful event handlers stored in a `Vec`
- Why `Box<dyn FnMut(&E)>` is needed to store heterogeneous closures together
- How to implement subscribe/emit mechanics with mutable interior dispatch
- How `Observable<T>` notifies listeners with both old and new values
- Why `'static` bounds are required when storing closures beyond the current scope

## Rust Application

`EventEmitter<E>` stores handlers as `Vec<Box<dyn FnMut(&E)>>`. Each `subscribe` call boxes an `FnMut` closure and appends it; `emit` iterates `&mut self.handlers` calling each one. `Observable<T>` calls `FnMut(&T, &T)` listeners on every value change. The `'static` bound on subscribed closures ensures they can outlive the registration site — essential for long-lived event systems like GUI loops.

Key patterns:
- `Box<dyn FnMut(&E)>` — type-erased mutable closures in a homogeneous collection
- `for handler in &mut self.handlers` — mutable reborrow enabling `FnMut` dispatch
- `impl FnMut(&E) + 'static` — accepting any compatible closure at the call site

## OCaml Approach

OCaml implements observers with mutable reference lists holding function values. A `ref` holds a list of `'a -> unit` functions; subscribers cons onto the list; emit iterates with `List.iter`. Since OCaml functions are first-class and garbage collected, there is no boxing overhead or lifetime tracking — the runtime handles memory automatically.

```ocaml
let make_emitter () =
  let handlers = ref [] in
  let subscribe h = handlers := h :: !handlers in
  let emit e = List.iter (fun h -> h e) !handlers in
  (subscribe, emit)
```

## Key Differences

1. **Storage**: Rust requires `Box<dyn FnMut>` with a `'static` bound to store closures in a collection; OCaml closures are heap-allocated and GC-managed with no annotation needed.
2. **Mutability tracking**: Rust distinguishes `Fn`, `FnMut`, and `FnOnce` — using `FnMut` explicitly signals handlers may modify captured state; OCaml has no such distinction.
3. **Propagation control**: Returning `bool` from Rust handlers to stop propagation is natural and type-safe; OCaml uses exceptions or a mutable flag for the same effect.
4. **Generic events**: Rust uses a single generic `EventEmitter<E>` working for any event type at compile time; OCaml uses polymorphic variants or a dedicated GADT for type-safe event dispatch.

## Exercises

1. **Unsubscribe support**: Extend `EventEmitter` so `subscribe` returns an index handle and add `unsubscribe(handle: usize)` that removes that handler by swapping in a no-op closure.
2. **Filtered subscription**: Add `subscribe_filtered(pred, handler)` that only calls `handler` when `pred(&event)` returns `true`, composing the predicate inside the stored closure.
3. **Once handler**: Add `subscribe_once` that automatically unregisters the handler after it fires once, using an `Option` inside the closure that is taken on first call.
