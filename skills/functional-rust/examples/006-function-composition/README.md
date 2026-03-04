# 006: Function Composition

**Difficulty:** 2  **Level:** Intermediate

Build complex data transformations by snapping simple functions together — like LEGO blocks for logic.

## The Problem This Solves

Imagine you're cleaning user-submitted text: trim whitespace, lowercase it, strip punctuation, collapse double spaces. Without composition, you either write one huge function that does everything (hard to test, impossible to reuse), or you chain temporary variables one after another (messy, verbose).

In many languages you'd write helper utilities and call them in sequence: `strip(lower(trim(input)))`. But reading inside-out is awkward. Some languages have a pipe operator (`|>`) to flip this into left-to-right reading. Rust doesn't have a built-in pipe, but it doesn't need one.

Rust's iterator method chaining *is* composition. When you write `.map(f).filter(g).map(h)`, you're composing three functions into a pipeline that reads left to right, runs lazily, and produces zero intermediate allocations. This is both more readable and faster than equivalent imperative loops.

## The Intuition

In Python you might write:
```python
result = [x * 2 for x in data if x * 2 % 2 == 0]
```

Or in JavaScript:
```js
data.map(x => x * 2).filter(x => x % 2 === 0).reduce((a, b) => a + b, 0)
```

Rust's iterator chains look almost identical to that JS version — except they're lazy (nothing runs until you ask for a result) and the compiler optimizes the whole chain into a single loop.

For composing standalone functions, Rust uses closures. The `compose(f, g)` pattern — where you get back a new function that applies `g` then `f` — is just a function that returns a closure. The `move` keyword in the closure captures `f` and `g` by value so they live long enough.

## How It Works in Rust

```rust
// Compose two functions: g runs first, then f
pub fn compose<A, B, C>(
    f: impl Fn(B) -> C + 'static,
    g: impl Fn(A) -> B + 'static,
) -> Box<dyn Fn(A) -> C> {
    Box::new(move |x| f(g(x)))  // move captures f and g into the closure
}

// Usage: double_then_inc(5) → 5*2=10, then +1 = 11
let double = |x: i64| x * 2;
let inc    = |x: i64| x + 1;
let double_then_inc = compose(inc, double);
assert_eq!(double_then_inc(5), 11);

// Idiomatic Rust: iterator method chain IS composition
pub fn process(data: &[i64]) -> i64 {
    data.iter()
        .map(|x| x * 2)       // step 1: double each
        .filter(|x| x % 2 == 0) // step 2: keep evens (all of them, since we just doubled)
        .sum()                 // step 3: add up
}

// String pipeline: method chaining composes string transforms
pub fn process_string(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace("  ", " ")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect()  // collect() is what triggers the whole chain to run
}
```

The `Box<dyn Fn(A) -> C>` in `compose` is Rust's way of returning "some function" when the compiler can't know the exact type at compile time. For iterator chains, the compiler figures it all out and you never need that.

## What This Unlocks

- **Data transformation pipelines** — ETL jobs, log processing, report generation: read → clean → filter → aggregate in one readable chain
- **Reusable validators** — compose `trim`, `lowercase`, `validate_length` into a single `normalize` function for any text input
- **Configuration-driven transforms** — build a `Vec<Box<dyn Fn(i64) -> i64>>` at runtime and run any value through it with `fold`

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Pipe operator | `x \|> f \|> g` (built-in) | No built-in — use `.method()` chaining |
| Compose function | `let compose f g x = f (g x)` | `fn compose(f, g) -> Box<dyn Fn...>` |
| Iterator pipeline | `List.map f \|> List.filter g` | `.map(f).filter(g).collect()` |
| Laziness | Eager by default | Lazy by default — runs only when consumed |
| Returning a function | Natural (everything is a value) | Need `impl Fn(...)` or `Box<dyn Fn(...) >` |
