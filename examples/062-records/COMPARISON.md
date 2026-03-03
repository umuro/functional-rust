# Records — Immutable Update and Pattern Matching: OCaml vs Rust

## The Core Insight
Records (OCaml) and structs (Rust) are the simplest compound data types — named collections of fields. Both languages provide "functional update" syntax that constructs a new value by copying most fields from an existing one, making immutable programming ergonomic without manual field-by-field copying.

## OCaml Approach
OCaml records are defined with `type point = { x : float; y : float }`. Pattern matching directly destructures fields: `let area { width; height; _ } = ...`. Functional update with `{ r with origin = ... }` creates a new record reusing unchanged fields. Records are allocated on the GC heap, and the old and new records may share unchanged sub-values. All record fields are immutable by default (mutable fields require explicit `mutable` annotation).

## Rust Approach
Rust structs serve the same purpose: `struct Point { x: f64, y: f64 }`. Functional update uses `Struct { changed_field: val, ..old }`, which moves or copies fields from `old`. With `#[derive(Copy, Clone)]`, small structs like `Point` are stack-allocated and implicitly copied — no heap allocation or GC needed. Rust enforces visibility with `pub` on each field, whereas OCaml record fields are public within their module by default.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Definition | `type point = { x: float; y: float }` | `struct Point { x: f64, y: f64 }` |
| Functional update | `{ r with x = 5.0 }` | `Point { x: 5.0, ..r }` |
| Destructuring | `let { x; y; _ } = p` | `let Point { x, y } = p;` |
| Memory | GC heap | Stack (Copy) or heap (Box/Vec) |
| Mutability | Immutable default, opt-in `mutable` | Immutable default, opt-in `mut` |
| Visibility | Module-level | Per-field `pub` |

## What Rust Learners Should Notice
- Rust's `..old` struct update syntax is the equivalent of OCaml's `{ r with ... }` — both create new values, neither mutates
- `#[derive(Copy, Clone)]` on small structs gives you value semantics with zero overhead — the struct lives entirely on the stack
- Rust's `&Rect` borrowing lets functions read a record without taking ownership, similar to how OCaml freely passes GC-managed values
- Float comparison in Rust requires epsilon checks (`(a - b).abs() < f64::EPSILON`) — there's no built-in structural equality for floats
- Visibility is more granular in Rust: each field can be independently `pub` or private

## Further Reading
- [The Rust Book — Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [The Rust Book — Struct Update Syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax)
- [OCaml Records](https://cs3110.github.io/textbook/chapters/data/records_tuples.html)
