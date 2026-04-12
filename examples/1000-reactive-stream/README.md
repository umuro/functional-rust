[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1000 — Reactive Stream
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement a push-based reactive stream (`Observable<T>`) with an `Observer` trait providing `on_next`, `on_error`, and `on_complete` callbacks. Support `map` and `filter` operators that create derived observables. Compare with OCaml's record-based observer and functional observable composition.

## Learning Outcomes

- Model an `Observer<T>` as a trait with three callbacks
- Represent an `Observable<T>` as a struct wrapping a `subscribe_fn: Box<dyn Fn(&mut dyn Observer<T>)>`
- Build `map` and `filter` as operators that create new `Observable`s from existing ones
- Use `Rc<RefCell<T>>` for shared mutable state inside observers
- Map Rust's trait-based observer to OCaml's record `{ on_next; on_error; on_complete }`
- Recognise that functional reactive programming is deferred composition of push pipelines

## Rust Application

`Observable<T>` wraps a `Box<dyn Fn(&mut dyn Observer<T>)>` — a factory for push pipelines. `from_iter(items)` creates an observable that pushes all items then completes. `subscribe` drives the factory by passing an observer. `map` creates a new `Observable` that subscribes to `self` with a mapping observer wrapping the original. `filter` does the same with a predicate guard. `Rc<RefCell<Vec<T>>>` is used to collect emitted values in tests where state must be shared between the observer closure and the test assertion.

## OCaml Approach

OCaml's observer is a record: `{ on_next: 'a -> unit; on_error: exn -> unit; on_complete: unit -> unit }`. The observable is `{ subscribe: 'a observer -> unit -> unit }` — subscribe returns an unsubscribe thunk. `map f obs` wraps the observer's `on_next` with `fun v -> observer.on_next (f v)`. The OCaml version is more concise because records are lighter than trait objects, and there is no need for `Box` or `dyn`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Observer | `trait Observer<T>` | Record `'a observer` |
| Observable | `Box<dyn Fn(&mut dyn Observer<T>)>` | `{ subscribe: 'a observer -> unit -> unit }` |
| Map operator | Creates new `Observable` | Creates new `observable` |
| Shared state | `Rc<RefCell<…>>` | `ref` or mutable binding |
| Unsubscribe | Not shown (complex) | Returns thunk `unit -> unit` |
| Verbosity | High (trait + Box + dyn) | Low (records + closures) |

Reactive streams implement the observer pattern functionally: pipelines are composed at subscription time, not execution time. The same observable can be subscribed to multiple times, each producing an independent stream execution.

## Exercises

1. Implement a `take(n: usize)` operator that stops the stream after `n` values.
2. Implement a `merge` operator that subscribes to two observables and pushes values from both to a single subscriber.
3. Add error propagation: modify `map` to catch panics from the mapping function and forward them to `on_error`.
4. Implement a `Subject<T>` — a combined observable and observer — that fans out emissions to multiple subscribers.
5. In OCaml, implement `flat_map : ('a -> 'b observable) -> 'a observable -> 'b observable` for observable chaining.
