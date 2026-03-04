# 122: Higher-Order Functions with Lifetime Constraints

**Difficulty:** 3  **Level:** Intermediate

Write functions that accept and return other functions — and annotate lifetimes when those functions deal with references.

## The Problem This Solves

Higher-order functions are natural in Rust — pass a closure, return a closure, compose two functions — until references are involved. When a function takes a reference as input and returns a reference derived from it, the compiler needs to know: how long does the output reference live? If it comes from the input, the output can't outlive the input. Without lifetime annotations, the compiler can't verify this and rejects the code.

This isn't just bureaucracy. It prevents a real class of bugs. In C, returning a pointer to a local variable compiles fine and produces undefined behavior at runtime. In Rust, forgetting to connect input and output lifetimes is a compile error. The annotation is how you tell the compiler "these references have the same lifetime" — and it checks that claim throughout the call site.

The practical relief: when you return owned data (`String`, `Vec<T>`) from a higher-order function, no lifetime annotations are needed. Ownership and borrowing are separate concerns — only borrows need lifetime tracking.

## The Intuition

Lifetime annotations on higher-order functions are promises to the compiler: "this output reference comes from this input reference, so they live the same amount of time."

## How It Works in Rust

```rust
// HOF returning a reference — lifetime 'a connects input to output
fn find_first<'a, F>(items: &'a [&'a str], pred: F) -> Option<&'a str>
where
    F: Fn(&str) -> bool,
{
    // The returned &str points into `items`, which lives for 'a.
    // Without 'a, the compiler can't know that the output is valid.
    items.iter().copied().find(|&s| pred(s))
}

let data = vec!["apple", "banana", "cherry"];
let long = find_first(&data, |s| s.len() > 5);
assert_eq!(long, Some("banana"));

// Function composition — no lifetimes needed (owned values)
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))  // f and g are moved into the returned closure
}

let double_then_add = compose(|x: i32| x + 1, |x: i32| x * 2);
assert_eq!(double_then_add(5), 11);  // (5*2)+1 = 11

// HOF that returns owned data — no lifetime annotation needed
fn transform_all(items: &[&str], f: impl Fn(&str) -> String) -> Vec<String> {
    items.iter().map(|&s| f(s)).collect()
}
let lower = transform_all(&["Hello", "WORLD"], |s| s.to_lowercase());
```

## What This Unlocks

- **Predicate-based search** — `find_first`, `find_all`, `partition_by` over borrowed slices while preserving lifetime safety.
- **Function composition pipelines** — `compose(f, g)`, `pipe(x, f)` patterns for building transformation chains.
- **Zero-copy data processing** — return slices and references into the original data instead of allocating copies.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| HOF with references | No annotations — GC manages lifetimes | Lifetime parameters required |
| Function composition | Simple: `fun x -> f (g x)` | Generic bounds: `F: Fn(B) -> C, G: Fn(A) -> B` |
| Return references from inputs | GC keeps input alive automatically | `'a` lifetime links input to output |
| Returning owned data | N/A | No lifetimes needed — ownership is sufficient |
