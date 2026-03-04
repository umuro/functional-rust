# 505: Boxing Closures — Box\<dyn Fn\>

**Difficulty:** 3  **Level:** Intermediate

Erase closure types to store them in collections, struct fields, and return positions that need runtime flexibility.

## The Problem This Solves

Each closure in Rust has a unique, anonymous type. `|x| x + 1` and `|x| x * 2` are different types even though they have the same signature. This means you can't put them in a `Vec` together, store different closures in a struct field without a generic parameter, or return different closures from different branches of a `match`.

Generics solve part of this — `struct Processor<F: Fn(i32) -> i32> { f: F }` works if you always use the same closure. But it doesn't scale: you can't have a `Vec<Processor<???>>` where each element has a different closure. You need type erasure.

`Box<dyn Fn(i32) -> i32>` is the answer. It heap-allocates the closure and stores a fat pointer (data + vtable). All boxed closures with the same signature are the same type — they can live in the same `Vec`, struct field, or `HashMap`.

## The Intuition

`Box<dyn Fn>` is Rust's equivalent of a first-class function value in functional languages. In OCaml, every function value already is a heap-allocated closure. In Rust, you make this explicit.

Think of it as sealing the closure in an envelope. The envelope has a standard size and a label (`Fn(i32) -> i32`). You can sort envelopes without caring what's inside. The cost is that calling the function requires opening the envelope (vtable lookup) rather than jumping directly.

Python and JavaScript have this automatically — all functions are objects on the heap. Rust makes the heap allocation visible via `Box`, and the dynamic dispatch visible via `dyn`.

## How It Works in Rust

```rust
// Build a Vec of heterogeneous closures — all behind Box<dyn Fn>
let transforms: Vec<Box<dyn Fn(i32) -> i32>> = vec![
    Box::new(|x| x + 1),     // closure type A
    Box::new(|x| x * 2),     // closure type B (different!)
    Box::new(|x| x * x),     // closure type C
];
// Fold: apply each transform left-to-right
let result = transforms.iter().fold(3, |acc, f| f(acc));
// (3+1)*2 = 8, 8*8 = 64

// Struct with a boxed closure field (no generic parameter needed)
struct Handler {
    name: &'static str,
    apply: Box<dyn Fn(i32) -> String>,
}
impl Handler {
    fn new(name: &'static str, f: impl Fn(i32) -> String + 'static) -> Self {
        Handler { name, apply: Box::new(f) }  // 'static: closure owns all captures
    }
}

// Factory returning different closure types from branches
fn get_transform(mode: &str) -> Box<dyn Fn(i32) -> i32> {
    match mode {
        "double" => Box::new(|x| x * 2),
        "square" => Box::new(|x| x * x),
        _        => Box::new(|x| x),
    }
}
// Caller doesn't know (or care) which branch was taken
let f = get_transform("square");
println!("{}", f(5)); // 25
```

The `+ 'static` bound means the closure must own all its captured data (no borrowed references) — required when the `Box` outlives the creation scope.

## What This Unlocks

- **Heterogeneous closure collections** — `Vec<Box<dyn Fn>>` for plugin registries, middleware stacks, and event handler lists.
- **Struct fields without generic parameters** — `struct Config { validator: Box<dyn Fn(&str) -> bool> }` — no `<F>` needed.
- **Runtime-configurable behavior** — swap strategies, handlers, and formatters at runtime without recompilation.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| First-class function value | Built-in `'a -> 'b` (always heap) | `Box<dyn Fn(A) -> B>` (explicit heap) |
| Vec of closures | `(int -> int) list` | `Vec<Box<dyn Fn(i32) -> i32>>` |
| Struct with closure | `{ f: int -> int }` | `struct S { f: Box<dyn Fn(i32) -> i32> }` |
| Dispatch cost | Always vtable (OCaml's representation) | Opt-in: `Box<dyn>` vs zero-cost `impl` |
| Type erasure | Automatic | Explicit `Box<dyn ...>` |
