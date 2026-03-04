# 042: Option Map

**Difficulty:** 1  **Level:** Foundations

Transform the value inside an `Option` without unwrapping it — using `.map()`, `.map_or()`, and `filter_map`.

## The Problem This Solves

You have an `Option<i64>` and want to double the number if it exists, or leave it as `None` if it doesn't. The naive approach: unwrap, check for None, transform, re-wrap. That's four lines and easy to get wrong.

`.map()` does this in one line. It says: "apply this function to the value inside the `Option`, but only if there's a value. If it's `None`, stay `None`." No unwrapping, no `if let`, no boilerplate. Just a clean transformation that threads through the optional.

This pattern — transforming a wrapped value without opening the box — is the heart of functional programming with option types. Python's walrus operator `:=`, JavaScript's optional chaining `?.`, and Kotlin's `?.let {}` all approximate this. Rust makes it explicit and composable.

## The Intuition

In Python:
```python
x = parse_int("42")
result = x * 2 if x is not None else None
```

In Rust:
```rust
let result = parse_int("42").map(|x| x * 2);
```

Same idea — "apply a function if the value exists" — but the Rust version is a single expression. Chain multiple `.map()` calls to build a transformation pipeline:

```rust
parse_int("42")
    .map(|x| x * 2)   // doubles: Some(84)
    .map(|x| x + 1)   // adds one: Some(85)
```

If *any* step encounters `None`, the whole chain short-circuits to `None`. No null checks at each step.

## How It Works in Rust

```rust
// map: apply a function to Some(x), pass through None
Some(4.0_f64).map(|x| x.sqrt())   // → Some(2.0)
None::<f64>.map(|x| x.sqrt())     // → None

// map_or: apply a function to Some(x), use default for None
Some(5_i64).map_or(0, |x| x * 2)  // → 10
None::<i64>.map_or(0, |x| x)      // → 0

// filter_map: map + flatten — apply a function that itself returns Option
// Commonly used to extract Some values from a list of Options:
let opts = vec![Some(1), None, Some(3), None, Some(5)];
let values: Vec<i64> = opts.iter().filter_map(|x| *x).collect();
// → [1, 3, 5]
```

**The mental model:**
- `map(f)` — lifts `f: T → U` into `Option<T> → Option<U>`
- `map_or(default, f)` — like `map` but extracts the value (returns `U`, not `Option<U>`)
- `filter_map` on iterators — apply a `T → Option<U>` function, keep only the `Some` results

## What This Unlocks

- **Safe transformation pipelines** — chain operations on optional values without null checks.
- **Parsing with fallback** — `parse().ok().map_or(default, transform)` is a complete parse+transform+default pipeline.
- **List filtering** — `filter_map` is one of the most-used iterator methods in Rust for extracting valid values from noisy data.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map over option | `Option.map f opt` | `opt.map(\|x\| f(x))` (method syntax) |
| Default on None | `Option.value ~default opt` | `opt.map_or(default, f)` |
| Map + flatten list | `List.filter_map f lst` | `iter.filter_map(f).collect()` |
| Chaining | `Option.map f (Option.map g opt)` | `opt.map(g).map(f)` (left-to-right) |
| Type annotation | Inferred from `f` | Inferred from closure return type |
