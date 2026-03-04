# 119: Zero-Cost Abstractions

**Difficulty:** 2  **Level:** Intermediate

Iterator chains, closures, and newtypes compile to the same machine code as hand-written equivalents — abstraction with no runtime penalty.

## The Problem This Solves

In many languages, using higher-order functions comes with a cost: closures allocate on the heap, `map`/`filter` create intermediate collections, and calling a function through a pointer prevents inlining. Programmers face a real trade-off between readable, composable code and code that runs fast.

Rust breaks this trade-off. Iterator adapters like `filter`, `map`, and `sum` are lazy — they produce no intermediate `Vec`. The compiler fuses the chain into a single loop. Closures are not heap-allocated; each closure gets its own anonymous struct type and is monomorphized at the call site, where LLVM can inline it completely. The result is assembly that's identical (or better) than a hand-written `for` loop with explicit `if` guards.

The same principle applies to newtypes. `struct Meters(f64)` is a distinct type that prevents you from passing a `Seconds` where a `Meters` is expected — it's a real compile-time safety guarantee — but at runtime it's just `f64`. No boxing, no indirection, no header.

## The Intuition

Rust's abstractions are instructions to the compiler, not instructions to the CPU — they disappear at compile time, leaving only the minimum necessary machine code.

## How It Works in Rust

```rust
// Iterator chain — no intermediate Vec, compiles to a single loop
let result: i64 = (0..1000)
    .filter(|x| x % 2 == 0)   // lazy — no allocation yet
    .map(|x| x * x)            // lazy — still no allocation
    .sum();                    // consumes the chain in one pass

// Exactly equivalent to this hand-written loop:
let mut manual = 0i64;
for x in 0..1000 {
    if x % 2 == 0 { manual += x * x; }
}
assert_eq!(result, manual);  // same result, same assembly

// Closures — monomorphized, not heap-allocated
fn make_polynomial(coeffs: Vec<f64>) -> impl Fn(f64) -> f64 {
    // The closure captures `coeffs` by move.
    // The compiler creates a unique anonymous struct for this closure.
    // The returned `impl Fn(f64) -> f64` is inlined at call sites — no vtable.
    move |x| coeffs.iter().enumerate()
        .map(|(i, &c)| c * x.powi(i as i32))
        .sum()
}

// Newtypes — zero runtime cost, real compile-time safety
struct Meters(f64);
struct Seconds(f64);

fn speed(d: Meters, t: Seconds) -> f64 { d.0 / t.0 }
// speed(t, d)  // ERROR: mismatched types — the compiler catches the swap
// size_of::<Meters>() == size_of::<f64>()  // same bits at runtime
```

## What This Unlocks

- **Readable pipelines** — chain `.filter().map().fold()` without worrying that you're writing slow code.
- **Type-safe domain models** — `Meters`, `Seconds`, `Euros`, `Pixels` as distinct types prevent unit confusion bugs with zero runtime overhead.
- **Returned closures and iterators** — `impl Fn(...)` and `impl Iterator` let you build factory functions whose output is fully inlined at the call site.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Iterator chains | Eager — intermediate lists allocated | Lazy — single fused loop |
| Closures | Heap-allocated (GC captures environment) | Monomorphized — inline struct, no allocation |
| Newtypes | Constructor overhead (GC-boxed) | Zero cost — same bit pattern as inner type |
| HOF cost | Real allocation per call | Zero — compiler eliminates abstraction |
