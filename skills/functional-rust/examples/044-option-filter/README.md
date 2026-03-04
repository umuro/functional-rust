# 044: Option Filter

**Difficulty:** 1  **Level:** Foundations

Conditionally discard the value inside an `Option` — turning `Some(x)` into `None` if a predicate fails.

## The Problem This Solves

You've parsed a number and want to reject it if it's negative. Or you have an optional ID and want to discard it if it's out of range. `.filter()` on `Option` converts `Some(x)` to `None` whenever `x` doesn't meet a condition — with no `if let` needed.

The naive approach: unwrap, check, re-wrap. Three lines, and easy to forget the re-wrap:
```rust
let result = if let Some(x) = value {
    if x > 0 { Some(x) } else { None }
} else { None };
```

With `.filter()`:
```rust
let result = value.filter(|&x| x > 0);
```

Same semantics. One line. Chainable with `.map()` and `.and_then()`.

## The Intuition

In Python:
```python
result = x if x is not None and x > 0 else None
```

In JavaScript:
```javascript
const result = value !== null && value > 0 ? value : null;
```

In Rust: `opt.filter(|x| condition)` — applies the predicate to the inner value. If the predicate returns `true`, keep `Some(x)`. If `false` (or if the option is already `None`), become `None`.

It's the Option equivalent of `.filter()` on iterators — same mental model, different context.

## How It Works in Rust

```rust
// filter: keep Some(x) only if predicate holds
Some(4_i64).filter(|x| x % 2 == 0)   // → Some(4)  (4 is even ✓)
Some(3_i64).filter(|x| x % 2 == 0)   // → None     (3 is odd ✗)
None::<i64>.filter(|x| x % 2 == 0)   // → None     (nothing to filter)

// Chain multiple filters:
fn parse_positive_even(s: &str) -> Option<i64> {
    parse_int(s)
        .filter(|&x| x > 0)      // reject zero and negatives
        .filter(|&x| x % 2 == 0) // reject odd numbers
}
// "8"   → Some(8)
// "7"   → None  (odd)
// "-4"  → None  (negative)
// "abc" → None  (parse failed)
```

**On iterators:** `filter_map` on an iterator applies a `T → Option<U>` function and keeps only the `Some` results. Combining `Option::filter` with `Iterator::filter_map` is a common pattern for validating + extracting from collections:

```rust
let opts = vec![Some(1), Some(4), None, Some(6), Some(3)];
let evens: Vec<i64> = opts.iter()
    .filter_map(|opt| opt.as_ref().filter(|&&x| x % 2 == 0).cloned())
    .collect();
// → [4, 6]
```

**Clamping with filter:**
```rust
fn clamp_opt(opt: Option<i64>, min: i64, max: i64) -> Option<i64> {
    opt.filter(|&x| x >= min && x <= max)
}
// Values outside the range silently become None
```

## What This Unlocks

- **Input validation** — filter parsed values through a validity predicate before using them.
- **Safe range checking** — `clamp_opt` turns out-of-range values into `None` rather than silently wrapping or panicking.
- **Clean filter-map pipelines** — combine with `.and_then()` for validate-then-transform chains.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Filter option | `Option.filter pred opt` | `opt.filter(\|x\| pred(x))` |
| None passthrough | Automatic | Automatic — `None.filter(...)` is always `None` |
| Chaining | `Option.bind (Option.filter p opt) f` | `opt.filter(p).and_then(f)` |
| Iterator filter_map | `List.filter_map` | `iter.filter_map(f).collect()` |
| Validation pattern | `Option.filter` → `Option.map` | `.filter(validate).map(transform)` |
