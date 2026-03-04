# 392: impl Trait in Argument Position

**Difficulty:** 2  **Level:** Intermediate

Syntactic sugar for generic type parameters — cleaner signatures when you don't need to name the type.

## The Problem This Solves

Generic functions need type parameters. Writing `fn print_all<T: Display, I: Iterator<Item=T>>(items: I)` is correct but verbose — you're naming types only to immediately constrain them. When you just want to say "give me anything that iterates over displayable items," the naming adds noise without adding value.

`impl Trait` in argument position lets you express the same constraint without the boilerplate type parameter declaration. The compiler still monomorphizes the function — you get the same zero-cost abstraction, just without the angle brackets cluttering your API.

There's a trade-off: unlike explicit generics, `impl Trait` arguments can't be turbofished (`::<ConcreteType>`) and each `impl Trait` parameter is its own independent type (so two `impl Display` arguments can be different types, but you can't express that they must be the same).

## The Intuition

`impl Trait` as an argument is shorthand for "I need something that satisfies this trait, and I don't care what concrete type it is."

## How It Works in Rust

```rust
// These two are equivalent:
fn sum_iter(iter: impl Iterator<Item = i32>) -> i32 { iter.sum() }
fn sum_iter<I: Iterator<Item = i32>>(iter: I) -> i32 { iter.sum() }

// Works with any iterator of i32: Vec, Range, Chain...
let a = sum_iter(vec![1, 2, 3].into_iter());  // 6
let b = sum_iter(1..=100);                     // 5050

// Two impl Trait = two INDEPENDENT type params
fn show_twice(item: &impl Display) { println!("{} {}", item, item); }

// Nested: Iterator of Display items
fn print_all(items: impl Iterator<Item = impl Display>) {
    for item in items { println!("{}", item); }
}
```

1. Compiler sees `impl Trait` → treats it as an anonymous generic `<T: Trait>`
2. Monomorphizes at call site — no runtime overhead
3. Each `impl Trait` position is a separate type variable

## What This Unlocks

- **Cleaner public APIs**: Trait constraints front-and-center without angle-bracket noise.
- **Closure arguments**: `fn apply(f: impl Fn(i32) -> i32)` reads naturally.
- **Chaining constraints**: `impl Iterator<Item = impl Display + Debug>` in one expression.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Parametric polymorphism | `'a list` with explicit converter `'a -> string` | `impl Iterator<Item = impl Display>` |
| Type inference | Hindley-Milner, always inferred | Inferred at call site, anonymous in signature |
| Named type param | `let f : 'a -> 'b = ...` | `fn f<T, U>(x: T) -> U` |
| Sugar form | N/A (always explicit type vars) | `impl Trait` elides the type variable name |
| Turbofish | N/A | Not available with `impl Trait` (use explicit `<T>` instead) |
