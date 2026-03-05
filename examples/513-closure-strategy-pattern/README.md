📖 **[View on hightechmind.io →](https://hightechmind.io/rust/513-closure-strategy-pattern)**

---

# 513: Strategy Pattern via Closures

**Difficulty:** 3  **Level:** Intermediate

Replace interchangeable algorithms with closures stored in structs — no interfaces, no inheritance, no boilerplate.

## The Problem This Solves

The classic Gang-of-Four Strategy pattern requires: a `Strategy` interface, one concrete class per algorithm, dependency injection plumbing, and often a factory. In Java this is 5 files for what amounts to "sometimes do A, sometimes do B."

Without closures, Rust still requires traits and multiple implementing types. That's appropriate for complex strategies, but overkill when the "strategy" is just a comparison function, a discount calculation, or a formatting rule.

When behavior varies but the *shape* of the algorithm is fixed (same inputs, same output type), closures let you express strategy as configuration data rather than code structure.

## The Intuition

A strategy closure is a behavior stored as a value. Instead of calling `discount_strategy.apply(price)` on an object, you call `(self.discount)(price)` on a struct field. The closure *is* the strategy — no interface needed.

In Python, you'd pass a function: `sorter = Sorter(key=lambda x: x.name)`. In JavaScript, `array.sort((a, b) => a.price - b.price)` passes the comparison strategy inline. Rust's closures work the same way, with the addition that the closure type is checked at compile time.

Use `F: Fn(...)` generics when the strategy is fixed at construction time. Use `Box<dyn Fn(...)>` when you need to store different strategies in the same struct or swap them at runtime.

## How It Works in Rust

```rust
// Struct with a boxed closure strategy field
struct Sorter<T> {
    compare: Box<dyn Fn(&T, &T) -> std::cmp::Ordering>,
}

impl<T: Clone> Sorter<T> {
    fn new(compare: impl Fn(&T, &T) -> std::cmp::Ordering + 'static) -> Self {
        Sorter { compare: Box::new(compare) }
    }
    fn sort(&self, mut data: Vec<T>) -> Vec<T> {
        data.sort_by(|a, b| (self.compare)(a, b));
        data
    }
}

let nums = vec![3, 1, 4, 1, 5, 9];
// Three sorters — same struct, different strategies
let asc  = Sorter::new(|a: &i32, b| a.cmp(b));
let desc = Sorter::new(|a: &i32, b| b.cmp(a));
let abs_desc = Sorter::new(|a: &i32, b| b.abs().cmp(&a.abs()));

// Runtime strategy selection
let use_premium = true;
let discount: Box<dyn Fn(f64) -> f64> = if use_premium {
    Box::new(|p| p * 0.70)   // 30% off
} else {
    Box::new(|p| p * 0.95)   // 5% off
};
println!("${:.2}", discount(200.0)); // $140.00

// Composable validation strategies — add_rule chains multiple strategies
struct Validator<T> {
    rules: Vec<Box<dyn Fn(&T) -> bool>>,
}
impl<T> Validator<T> {
    fn new() -> Self { Validator { rules: Vec::new() } }
    fn add_rule(mut self, rule: impl Fn(&T) -> bool + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }
    fn validate(&self, value: &T) -> bool {
        self.rules.iter().all(|rule| rule(value))  // ALL rules must pass
    }
}

let validator = Validator::new()
    .add_rule(|&x: &i32| x > 0)
    .add_rule(|&x| x < 1000)
    .add_rule(|&x| x % 2 == 0);
println!("{}", validator.validate(&42));   // true
println!("{}", validator.validate(&1001)); // false (> 1000)
```

## What This Unlocks

- **Configurable algorithms** — sort orders, pricing rules, validation chains, and formatters as closure fields that callers swap without subclassing.
- **Runtime behavior switching** — read strategy from config at startup, store as `Box<dyn Fn>`, use throughout the program's lifetime.
- **Composable rule systems** — accumulate validation, transformation, or routing rules in a `Vec<Box<dyn Fn>>` and apply them as a pipeline.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Strategy | Higher-order function | `F: Fn(T) -> U` or `Box<dyn Fn>` |
| Struct with strategy | `{ strategy: int -> int }` | `struct S { f: Box<dyn Fn(i32) -> i32> }` |
| Runtime swap | Pass a different function | Replace `Box<dyn Fn>` field value |
| Multiple strategies | `('a -> 'b) list` | `Vec<Box<dyn Fn(A) -> B>>` |
| Interface requirement | Modules / functors | None — closure type serves as interface |
