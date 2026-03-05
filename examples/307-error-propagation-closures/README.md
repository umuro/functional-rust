# 307: Error Propagation in Closures

**Difficulty:** 3  **Level:** Advanced

`?` works in closures — but only when the closure itself returns `Result`, and that changes how you write iterators.

## The Problem This Solves

The `?` operator is Rust's answer to early return on error — concise, ergonomic, readable. But `?` only works inside functions (or closures) whose return type is `Result` or `Option`. When you pass a closure to `.map()`, the closure's return type is whatever the outer iterator chain expects — and that's usually just `T`, not `Result<T, E>`.

This creates a genuine tension: iterator adapters like `.map()` are your most powerful compositional tool, but they fight the error-handling model you rely on everywhere else. The common mistake is trying to use `?` in a `.map()` closure and hitting a confusing type error. The fix isn't to abandon iterators — it's to learn the idioms that restore both ergonomics and correctness.

This tension is fundamental: Rust's iterators assume success by default, but the real world produces failures. Several patterns exist, each with different trade-offs between early exit, error collection, and silent skipping.

## The Intuition

The key insight: if you want `?` inside a closure, make the closure return `Result`. Then `.collect::<Result<Vec<_>, _>>()` does the rest — it short-circuits on the first error and either gives you a clean `Vec` or the first failure. If you want to skip failures silently, use `.filter_map(|r| r.ok())`. If you want to accumulate until failure, use `.try_fold()`.

## How It Works in Rust

```rust
// Pattern 1: collect into Result — fail-fast
let numbers: Result<Vec<i32>, _> = ["1", "2", "bad"]
    .iter()
    .map(|s| s.parse::<i32>())   // each closure returns Result<i32, _>
    .collect();                   // short-circuits on first Err

// Pattern 2: filter_map — silently drop failures
let valid: Vec<i32> = ["1", "bad", "3"]
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();  // → [1, 3]

// Pattern 3: closure uses ? explicitly (return type must be Result)
let doubled: Vec<Result<i32, _>> = inputs.iter()
    .map(|s| -> Result<i32, _> { Ok(s.parse::<i32>()? * 2) })
    .collect();

// Pattern 4: try_fold for stateful accumulation
let sum = inputs.iter().try_fold(0i32, |acc, s| {
    Ok(acc + s.parse::<i32>()?)
});
```

## What This Unlocks

- **Idiomatic fallible pipelines** — process collections of strings, lines, or bytes with full error handling and zero boilerplate
- **Fine-grained control** — choose between fail-fast (`.collect::<Result<_,_>>()`), silent skip (`.filter_map`), or accumulate-all (`.partition`)
- **Named function escape hatch** — when closure syntax gets awkward, extract a named function that freely uses `?`, then pass it to `.map()`

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error in map | `List.filter_map` or `List.map` with `Result` | Multiple patterns |
| Fail-fast collection | Manual `fold` with early return | `.collect::<Result<Vec<_>,_>>()` |
| Skip errors | `List.filter_map` | `.filter_map(\|r\| r.ok())` |
| Early return in closure | Natural with `let*` / `Result.bind` | Closure must return `Result` for `?` |
| Stateful short-circuit | Recursive or `fold` with option | `.try_fold()` |
