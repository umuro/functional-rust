# 078: Where Clauses

**Difficulty:** 2  **Level:** Intermediate

Move complex generic constraints out of the function signature and into a `where` block for readability — same semantics, cleaner code.

## The Problem This Solves

Inline bounds (`fn foo<T: A + B, U: C, F: Fn(T) -> U>`) work fine for one or two constraints. But real-world generic functions can get messy fast. When you have four type parameters and each has two or three bounds, the function signature becomes a wall of angle brackets before you even see the function name.

The `where` clause doesn't add power — it's purely a formatting choice. But readability is correctness in practice: code that's hard to read gets misunderstood and misused. `where` lets you write the function name and parameters first, then declare the constraints separately, like a mathematical "where" statement.

It also handles cases that inline syntax can't express cleanly: bounds on associated types (`T::Item: Display`), lifetime + trait combinations, and constraints on generic parameters of generic parameters.

## The Intuition

Compare:

```rust
// Inline — hard to read
fn transform<T, U, A, F: Fn(&T) -> U, G: Fn(A, U) -> A>(items: &[T], f: F, g: G, init: A) -> A

// where clause — easier to scan
fn transform<T, U, A, F, G>(items: &[T], f: F, g: G, init: A) -> A
where
    F: Fn(&T) -> U,
    G: Fn(A, U) -> A,
```

In OCaml, functor signatures play a similar role — listing what a module must provide, separately from how it's used. The `where` clause is Rust's way of saying "here's what I need from my type parameters."

## How It Works in Rust

```rust
// Simple case: where clause for a multi-bound
fn sorted_summary<T>(items: &mut [T]) -> String
where
    T: Ord + Display,   // T must be sortable AND printable
{
    items.sort();
    items.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
}
```

```rust
// Complex: multiple type params, each with bounds
fn bounded_transform<T, F>(items: &[T], transform: F, lo: T, hi: T) -> Vec<T>
where
    T: PartialOrd + Clone,  // T needs comparison and cloning
    F: Fn(&T) -> T,         // F is a function from T to T
{
    items.iter().map(|x| {
        let y = transform(x);
        if y < lo { lo.clone() } else if y > hi { hi.clone() } else { y }
    }).collect()
}
```

```rust
// Arithmetic ops in where clause (using std::ops)
fn numeric_summary<T>(a: T, b: T) -> String
where
    T: Add<Output = T> + Mul<Output = T> + Display + Copy,
{
    format!("sum={}, product={}", a + b, a * b)
}
```

Use inline bounds for simple single-constraint cases. Switch to `where` when you have more than two type parameters or any parameter has more than two bounds.

## What This Unlocks

- **Readable higher-order functions**: when `F: Fn(...)` has complex signatures, `where` keeps the function name visible at a glance.
- **Bounds on associated types**: `where T::Item: Display` — impossible with inline syntax.
- **Lifetime + trait combos**: `where T: 'a + Serialize` — cleanly separates concerns.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Constraint declaration location | Inline in type signature or separate `sig` | Inline `<T: Bound>` or `where T: Bound` |
| Bounds on associated types | Module type field constraints | `where T::Item: Trait` |
| Readability choice | Functor `sig` separates type from impl | `where` clause separates params from constraints |
| Runtime impact | None | None (both compile to same monomorphized code) |
