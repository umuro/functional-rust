📖 **[View on hightechmind.io →](https://hightechmind.io/rust/516-closure-environment)**

---

# Complex Closure Environments
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Closures derive much of their power from capturing their surrounding environment — local variables, structs, collections, even other closures. Understanding what a closure captures, how it captures it (by move vs by reference), and what that means for ownership is central to writing idiomatic Rust. This example explores complex capture scenarios: closures over structs with boxed function fields, cyclic iterators over vectors, closures wrapping other closures, mutable counters, and growing accumulators.

## Learning Outcomes

- How `move` closures take ownership of captured variables
- How a closure can capture a struct containing a `Box<dyn Fn>` field
- How mutable state (counters, accumulators) lives inside `FnMut` closures
- How closures can wrap other closures to add behavior like logging
- The relationship between closure capture mode and the `Fn`/`FnMut`/`FnOnce` trait bound

## Rust Application

`make_formatter` takes ownership of a `Config` struct (including its `Box<dyn Fn>` field) via `move`, producing an `impl FnMut(&str) -> String`. `make_cycler` captures a `Vec<T>` and a mutable `index` counter — it must be `FnMut` because it mutates `index` on each call. `make_logged_fn` wraps an existing closure `F` with a `String` name, demonstrating closure-over-closure composition. `make_accumulator` captures a `Vec<T>` and grows it on each call, returning the whole accumulated state.

Key patterns:
- `move |s| { ... cfg ... }` — transferring struct ownership into closure
- `let mut index = 0; move || { index = ...; }` — mutable counter in `FnMut`
- `impl Fn(A) -> B` wrapping another `F: Fn(A) -> B` — transparent decoration

## OCaml Approach

OCaml closures capture by reference to the heap — all values are boxed, so there is no move/copy distinction. A mutable counter is represented with `ref`:

```ocaml
let make_cycler items =
  let arr = Array.of_list items in
  let i = ref 0 in
  fun () ->
    let v = arr.(!i) in
    i := (!i + 1) mod Array.length arr;
    v
```

Wrapping a closure with logging is identical syntactically — just `fun x -> log name; f x`.

## Key Differences

1. **Capture semantics**: Rust closures capture by reference by default but require `move` to take ownership; OCaml always captures by reference to GC-managed heap values.
2. **Mutability in captures**: Rust requires `FnMut` for closures that mutate captured state, making the mutation explicit at the type level; OCaml uses `ref` cells with no type-level distinction.
3. **Nested closures**: Rust must satisfy lifetime/ownership rules when a closure captures another closure (e.g., both must be `'static` if stored); OCaml has no such constraint.
4. **Cycler state**: Rust's cycler owns its `Vec` and `index` completely inside the closure; OCaml's equivalent uses `ref` and an array, with the GC preventing dangling references.

## Exercises

1. **Throttle closure**: Write `make_throttle(f, n)` that calls `f` only every `n` invocations, tracking the call count inside the returned `FnMut`.
2. **Logging wrapper**: Extend `make_logged_fn` to record every call's argument and return value in a `Vec` captured inside the wrapper closure.
3. **Composable formatter**: Implement `make_pipeline(steps: Vec<Box<dyn Fn(String) -> String>>)` that returns an `impl FnMut(String) -> String` applying each step in sequence.
