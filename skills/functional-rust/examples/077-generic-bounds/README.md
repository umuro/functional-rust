# 077: Generic Bounds

**Difficulty:** 2  **Level:** Intermediate

Constrain generic type parameters so a function can call trait methods on them — more flexible than concrete types, more expressive than unconstrained generics.

## The Problem This Solves

You want to write `find_max` that works for `i32`, `f64`, `String`, and any other comparable type — without writing it three times. Unconstrained generics can't call any methods (Rust doesn't know what `T` can do). But if you write `T: PartialOrd`, you're promising "T can be compared" — and Rust will accept your `if a >= b`.

Without bounds, the compiler error is swift: `error[E0369]: binary operation >= cannot be applied to type T`. The fix is always the same: declare what you need from `T` upfront. This is fundamentally different from Python (which just tries the method at runtime and crashes if it's missing) or Java generics (which are erased and can only call `Object` methods without a bound).

Bounds make implicit assumptions explicit and checked at compile time. Your function signature *documents* what it requires — and the compiler enforces it.

## The Intuition

`T: PartialOrd` means "T must implement the `PartialOrd` trait." Multiple bounds use `+`: `T: PartialOrd + Display` means "T must be both comparable *and* printable."

- **Python**: `def find_max(lst)` — assumes elements support `>`, crashes if they don't
- **Java**: `<T extends Comparable<T>>` — similar concept, more verbose, erased at runtime
- **OCaml**: type variables with constraints, inferred by the compiler
- **Rust**: `<T: PartialOrd>` — explicit, monomorphized (a separate copy of the function is compiled for each concrete type `T`)

Monomorphization means generic Rust code is as fast as hand-written concrete code — no boxing, no vtable.

## How It Works in Rust

```rust
// Single bound: T must support comparison
fn find_max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    slice.iter().reduce(|a, b| if a >= b { a } else { b })
}

// Multiple bounds: T must be both comparable AND printable
fn print_max<T: PartialOrd + Display>(slice: &[T]) -> Option<String> {
    find_max(slice).map(|v| format!("Max: {}", v))
}

// Clamp works for any ordered type — floats, integers, strings
fn clamp<T: PartialOrd>(value: T, lo: T, hi: T) -> T {
    if value < lo { lo }
    else if value > hi { hi }
    else { value }
}
```

```rust
// Traits can require other traits as supertraits
trait Summarize: Display {          // anything that impls Summarize must also impl Display
    fn summary(&self) -> String;
}

fn print_summaries<T: Summarize>(items: &[T]) -> String {
    items.iter().map(|i| i.summary()).collect::<Vec<_>>().join(", ")
}
```

## What This Unlocks

- **Generic algorithms**: write `sort`, `min`, `max`, `binary_search` once and reuse across all comparable types.
- **Reusable utility functions**: `clamp`, `between`, `deduplicate` work on any type with the right trait — no duplication.
- **Custom trait hierarchies**: define your own `Metric: PartialOrd + Display + Clone` supertrait and build all your domain logic around it.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type constraints | `'a. 'a -> 'a -> bool` inferred | `<T: Trait>` explicit in signature |
| Multiple constraints | Module signatures, functors | `T: Trait1 + Trait2` |
| Dispatch | Compile-time (parametric polymorphism) | Compile-time (monomorphization) |
| Supertrait | Module inclusion / functor composition | `trait Foo: Bar + Baz` |
| Runtime cost | Zero | Zero (monomorphized to concrete types) |
