# 136: Existential Types

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Hide a concrete type behind a trait so callers can use the behavior without knowing the implementation — the type exists, but its identity is secret.

## The Problem This Solves

You're writing a function that returns "something you can display." The caller doesn't care if it's an `i32` or a `String` or a custom struct — they just want to call `.show()`. But Rust's type system is explicit: a function must declare its exact return type. You can't just return "some type that implements `Showable`" without either naming the concrete type (breaking abstraction) or paying the cost of a `Box<dyn Trait>`.

Or you have a collection that needs to hold values of different types — integers, strings, floats — all behind the same interface. A `Vec<Box<dyn Display>>` works, but where's the documentation pattern? How do you communicate "this is the existential idiom" to readers?

Existential types are the formal name for what Rust calls `impl Trait` (for opaque return types) and `dyn Trait` (for trait objects). Both say: "there exists some concrete type here — you can use it through this trait interface, but you don't need to know what it is."

## The Intuition

The word "existential" comes from logic: "there exists a type T such that T implements Showable." The caller doesn't know which T — just that one exists, and they can use the `Showable` interface on it.

In practice:
- `-> impl Showable`: "I return *some* showable thing. It's always the same concrete type, but I'm not telling you what it is." Faster (no indirection), but inflexible — all branches must return the same concrete type.
- `-> Box<dyn Showable>`: "I return *some* showable thing. It might be different each time." Flexible (heterogeneous), costs one heap allocation and one pointer dereference.

The closure-based variant captures a value and a display function together, hiding the type even more completely — you don't even need a trait impl on the original type.

## How It Works in Rust

```rust
trait Showable {
    fn show(&self) -> String;
}
impl Showable for i32    { fn show(&self) -> String { format!("{}", self) } }
impl Showable for String { fn show(&self) -> String { self.clone() } }
impl Showable for f64    { fn show(&self) -> String { format!("{:.2}", self) } }

// impl Trait: opaque return — caller sees Showable interface, not i32
// Limitation: all branches MUST return the same concrete type
fn make_thing() -> impl Showable {
    42i32   // always i32, but caller just sees "some Showable"
}

// Box<dyn Trait>: dynamic dispatch — can return different types per branch
fn make_showable_dyn(choice: u8) -> Box<dyn Showable> {
    match choice {
        0 => Box::new(42i32),              // i32
        1 => Box::new("hello".to_string()), // String
        _ => Box::new(3.14f64),            // f64
    }
}

// Heterogeneous collection — all items behind the same trait interface
let items: Vec<Box<dyn Showable>> = vec![
    Box::new(42i32),
    Box::new("hello".to_string()),
    Box::new(3.14f64),
];
let strings: Vec<String> = items.iter().map(|item| item.show()).collect();
// ["42", "hello", "3.14"]

// Closure-based variant: pack a value and its display logic together
// The concrete type T is completely hidden — not even a trait is needed on T
struct ShowableBox {
    show_fn: Box<dyn Fn() -> String>,
}

impl ShowableBox {
    fn new<T: 'static>(value: T, show: impl Fn(&T) -> String + 'static) -> Self {
        ShowableBox {
            show_fn: Box::new(move || show(&value)),  // T is captured and hidden
        }
    }
    fn show(&self) -> String { (self.show_fn)() }
}

let box1 = ShowableBox::new(42, |x| format!("{}", x));
let box2 = ShowableBox::new("hello", |s| s.to_string());
// box1 and box2 have the same type ShowableBox, but hide different concrete types
```

## What This Unlocks

- **Plugin systems** — load plugins as `Box<dyn Plugin>` collections; each plugin's concrete type is unknown to the host, but the interface is uniform.
- **Iterator composition** — `impl Iterator<Item = T>` return types let you compose lazy iterators without exposing complex nested types like `Map<Filter<Vec<T>>>`.
- **Strategy pattern** — store different algorithms (`Box<dyn Sorter>`, `Box<dyn Renderer>`) without a top-level enum, keeping extension open.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Existential packing | `pack (type a) show value : (module SHOWABLE)` — first-class module | `Box::new(value) as Box<dyn Trait>` — heap allocation |
| Opaque return | Module signature hiding `type t` | `-> impl Trait` — same concrete type, hidden |
| Heterogeneous | `(module SHOWABLE) list` | `Vec<Box<dyn Trait>>` |
| Closure existential | Closures capture type implicitly | `Box<dyn Fn()>` erases closure type |
