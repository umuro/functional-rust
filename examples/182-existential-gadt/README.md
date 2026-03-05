📖 **[View on hightechmind.io →](https://hightechmind.io/rust/182-existential-gadt)**

---

# 182: Existential Types

**Difficulty:** 4  **Level:** Advanced

Store values of any type alongside their operations — the type is forgotten, the capability is preserved.

## The Problem This Solves

You have values of different types — `i32`, `String`, `f64`, a custom struct — and you want to put them all in one collection. Not as raw bytes; you want to be able to *do* something with each value, like display it. But you don't want to enumerate all possible types in an enum.

An existential type says: "there exists some type T, and I have a value of that T, and T supports these operations." The concrete T is hidden from the outside — only the operations are visible. This is the key idea behind `Box<dyn Trait>`.

The contrast with generics: `fn show<T: Display>(v: T)` requires the caller to name `T`. An existential says the *callee* picks `T` — and nobody needs to name it.

## The Intuition

When you write `Box::new(42_i32) as Box<dyn Display>`, you're packing two things into the heap: the value `42` and a pointer to `i32`'s implementation of `Display`. The type `i32` is erased — you can only call `Display`'s methods. This is an existential: *some type T* implements `Display`, and here's a value of that T.

OCaml achieves the same with GADT constructors: `Show : 'a * ('a -> string) -> showable` packs a value with its display function, hiding `'a`. Rust's `Box<dyn Trait>` is the runtime analog with vtable dispatch.

## How It Works in Rust

```rust
use std::fmt;

// Approach 1: Box<dyn Trait> — Rust's native existential
fn make_showables() -> Vec<Box<dyn fmt::Display>> {
    vec![
        Box::new(42),           // i32 erased — only Display survives
        Box::new("hello"),      // &str erased
        Box::new(3.14),         // f64 erased
    ]
}

for item in make_showables() {
    println!("{}", item);  // dispatch via vtable — correct impl called
}
```

Approach 2: closure-based packing (mirrors OCaml GADT constructor):

```rust
struct Showable {
    show_fn: Box<dyn Fn() -> String>,
}

impl Showable {
    fn new<T: 'static>(value: T, to_string: fn(&T) -> String) -> Self {
        // Both `value` and `to_string` are captured — T is erased from the outside
        Showable {
            show_fn: Box::new(move || to_string(&value)),
        }
    }

    fn show(&self) -> String { (self.show_fn)() }
}

let items = vec![
    Showable::new(42, |x| x.to_string()),
    Showable::new(String::from("hello"), |x| x.clone()),
    Showable::new(3.14f64, |x| format!("{}", x)),
];
```

Multi-trait existential via supertrait:

```rust
trait Printable: fmt::Display + fmt::Debug {}
impl<T: fmt::Display + fmt::Debug> Printable for T {}

let items: Vec<Box<dyn Printable>> = vec![Box::new(42), Box::new("hi")];
// Can call both display and debug on each item
```

## What This Unlocks

- **Heterogeneous collections** — `Vec<Box<dyn Event>>` for event queues with different event types
- **Plugin interfaces** — store plugins as `Box<dyn Plugin>` regardless of their concrete types
- **Capability bundles** — pack a value with its serializer, validator, or formatter

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Existential encoding | GADT constructor: `Show : 'a * ('a -> string) -> showable` | `Box<dyn Trait>` via vtable |
| Closure packing | `let pack v f = Show (v, f)` | `struct { show_fn: Box<dyn Fn() -> String> }` |
| Multi-capability | First-class module with multiple fields | Super-trait combining multiple traits |
| Recovering T | Pattern match on GADT constructor | Not possible — use `Any::downcast_ref` if needed |
| Allocation | GC-managed heap | `Box` — heap allocation, drop via `Drop` |
