# 005: Currying and Partial Application

**Difficulty:** 2  **Level:** Intermediate

Freeze some arguments of a function now, supply the rest later — turning general functions into specific tools.

## The Problem This Solves

You have a list of numbers and you want to keep only those greater than 5. Then you want to keep those greater than 10. Then greater than 20. Do you write three separate filter functions? Copy-paste the same logic with different thresholds? That's how bugs sneak in.

Currying is the answer: instead of a function that takes *all* arguments at once, you write a function that takes one argument and returns another function waiting for the next. This lets you create specialized versions — `greater_than(5)`, `greater_than(10)` — from a single general definition.

In Rust, closures make this pattern natural and efficient. When you call `greater_than(5)`, you get back a closure that remembers the `5` and can be passed directly to `.filter()`. This is how you write reusable, composable predicates without repetition — the same technique used inside Rust's standard library itself.

## The Intuition

Python has `functools.partial` for this. JavaScript developers do it manually with arrow functions:
```js
const greaterThan = threshold => x => x > threshold;
const bigNumbers = data.filter(greaterThan(5));
```

Rust's version looks almost identical:
```rust
fn greater_than(threshold: i64) -> impl Fn(&i64) -> bool {
    move |x| *x > threshold
}
let big_numbers: Vec<_> = data.iter().filter(greater_than(5)).collect();
```

The `move` keyword is the key difference from JS: Rust needs you to explicitly say "capture `threshold` by value into this closure" because of ownership rules. Once you understand `move`, partial application feels completely natural.

## How It Works in Rust

```rust
// A function that returns a function — this IS currying
pub fn add(n: i64) -> impl Fn(i64) -> i64 {
    move |x| x + n  // `move` captures `n` by value — the closure owns it
}

let add5 = add(5);    // freeze n=5
assert_eq!(add5(3), 8);  // apply the rest later
assert_eq!(add5(0), 5);

// Build a predicate, pass it to filter
pub fn greater_than(threshold: i64) -> impl Fn(&i64) -> bool {
    move |x| *x > threshold  // *x dereferences the &i64 to compare
}

let data = vec![1, 5, 3, 8, 2, 9];
let big: Vec<_> = data.iter().filter(greater_than(4)).copied().collect();
// result: [5, 8, 9]

// Fully curried three-argument function (rarely needed, but shows the pattern)
pub fn curried_add3(a: i64) -> Box<dyn Fn(i64) -> Box<dyn Fn(i64) -> i64>> {
    Box::new(move |b| Box::new(move |c| a + b + c))
}

assert_eq!(curried_add3(1)(2)(3), 6);

// Range predicate — capture both bounds
pub fn between(low: i64, high: i64) -> impl Fn(&i64) -> bool {
    move |x| *x >= low && *x <= high
}

let in_range: Vec<i64> = vec![1,2,3,4,5,6]
    .iter().copied().filter(between(2, 5)).collect();
// result: [2, 3, 4, 5]
```

`impl Fn(i64) -> i64` in a return position means "I return some closure, the compiler knows what it is." Use `Box<dyn Fn...>` when the closure type can't be known at compile time (e.g., stored in a collection).

## What This Unlocks

- **Configurable filters** — build predicates like `between(start, end)`, `starts_with(prefix)` once, reuse everywhere with different parameters
- **Transform pipelines** — collect a `Vec<Box<dyn Fn(i64) -> i64>>` of partially-applied transforms and apply them all to any value
- **Event handlers and callbacks** — create a handler closure that closes over context (a database connection, a user ID) without needing a struct

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default currying | Automatic — all functions are curried | Manual — use closures explicitly |
| `add 5` | Returns a function `int -> int` immediately | Write `fn add(n) -> impl Fn(i64) -> i64` |
| Capturing context | Closure over environment naturally | Need `move` to capture by value |
| Return type | Inferred automatically | Must write `impl Fn(...)` or `Box<dyn Fn(...) >` |
| `partial` | `let add5 = add 5` | `let add5 = add(5);` — looks the same! |
