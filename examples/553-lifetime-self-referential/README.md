📖 **[View on hightechmind.io →](https://hightechmind.io/rust/553-lifetime-self-referential)**

---

# Self-Referential Structs
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A self-referential struct stores a reference to its own data — for example, a struct that owns a `String` and also holds a `&str` pointing into that same `String`. This is fundamentally incompatible with Rust's ownership model: moving the struct invalidates the internal reference. This is one of Rust's hardest problems, arising in async futures (which hold references to their own state), parsers (holding a pointer into a buffer they own), and event loops. The standard solutions are: use indices instead of references, use `Pin<Box<T>>` for unmovable data, or use external crates like `ouroboros`.

## Learning Outcomes

- Why self-referential structs are problematic in Rust's ownership model
- How storing indices instead of references sidesteps the problem safely
- How `Pin<Box<T>>` prevents a struct from moving, enabling self-references
- How the `Owner`/`View` two-struct pattern separates owned data from borrowed views
- Where self-referential structs arise: async futures, parsers, event loop state machines

## Rust Application

`Buffer` stores data as `String` with `start`/`end` indices — `view(&self) -> &str` computes the slice from indices rather than storing a pointer. `Owner` / separate-view pattern uses two structs: one owns the data, another borrows from it as a separate lifetime. `Pin<Box<T>>` enables safe self-referential structs by preventing the struct from being moved after pinning — async Rust uses this for futures that hold references to their own stack frames.

Key patterns:
- Indices instead of references: `start: usize, end: usize` computed into view on demand
- `Pin<Box<T>>` for unmovable self-referential structures
- Two-struct pattern: `Owner { data: String }` + `View<'a> { text: &'a str }`

## OCaml Approach

OCaml's GC-managed heap allows self-referential structures trivially — values can reference themselves without any pinning or special types:

```ocaml
type node = { mutable next: node option; value: int }
let rec n = { next = Some n; value = 42 }  (* self-referential — fine in OCaml *)
```

The GC tracks the cycle and keeps all nodes alive.

## Key Differences

1. **Move safety**: Rust's ownership model makes self-references dangerous when structs are moved; OCaml values are GC-managed and never "moved" in the Rust sense.
2. **Async futures**: Rust async futures are self-referential state machines requiring `Pin`; OCaml's effect-based async (OCaml 5.x) does not require pinning.
3. **Index pattern**: Storing indices instead of references is idiomatic Rust for self-referential data; OCaml can store direct references without concern.
4. **Crate solutions**: `ouroboros`, `self_cell`, and `rental` (deprecated) solve self-referential structs with macros; OCaml needs no such workarounds.

## Exercises

1. **Index-based parser**: Implement `struct Parser { source: String, pos: usize }` where all parsing methods return `&str` slices computed from `pos` into `source` — no self-reference needed.
2. **Pinned future**: Write a simple state machine struct implementing `Future` that stores a reference to a field within itself using `Pin<&mut Self>` in the `poll` method.
3. **Owner-View pair**: Implement a `TextDocument` owning a `String` and a `Selection { start: usize, end: usize }` that returns `&str` slices via `document.selected_text(&selection)`.
