📖 **[View on hightechmind.io →](https://hightechmind.io/rust/513-closure-strategy-pattern)**

---

# Closure Strategy Pattern

The Strategy pattern replaces object hierarchies with closures stored as `Box<dyn Fn>` fields — the algorithm is a runtime-swappable value rather than a compile-time type, enabling clean separation of structure and behaviour.

## Problem Statement

The classic Strategy pattern (GoF) uses an interface with multiple implementations: `SortStrategy`, `PriceStrategy`. In languages without closures, this requires class hierarchies. In Rust, a `Box<dyn Fn(&T, &T) -> Ordering>` is a strategy — any comparator logic works, including closures created inline. This eliminates boilerplate: no traits, no implementations, no dispatch indirection beyond what `Box<dyn Fn>` already provides. The pattern applies to: sorting, pricing rules, validation, logging, retry policies, and any algorithm that varies independently of the structure using it.

## Learning Outcomes

- Store a strategy as `Box<dyn Fn>` in a struct field
- Accept `impl Fn + 'static` in constructors and box internally
- Apply the strategy via `(self.strategy)(args)`
- Build factory functions for common strategies (`no_discount`, `percentage_discount`, `fixed_discount`)
- Compose multiple validation rules in a `Validator` that collects all errors

## Rust Application

`Sorter` stores a comparator strategy:

```rust
pub struct Sorter<T> {
    compare: Box<dyn Fn(&T, &T) -> Ordering>,
}

impl<T: Clone> Sorter<T> {
    pub fn new(compare: impl Fn(&T, &T) -> Ordering + 'static) -> Self {
        Sorter { compare: Box::new(compare) }
    }
    pub fn sort(&self, mut data: Vec<T>) -> Vec<T> {
        data.sort_by(|a, b| (self.compare)(a, b)); data
    }
}
```

Interchangeable pricing strategies:

```rust
let calc = PriceCalculator::new(percentage_discount(20.0));
assert_eq!(calc.calculate(100.0), 80.0);

let calc = PriceCalculator::new(fixed_discount(15.0));
assert_eq!(calc.calculate(100.0), 85.0);
```

## OCaml Approach

OCaml's first-class functions make the strategy pattern trivial:

```ocaml
let sort compare data = List.sort compare data
let sorter_asc = sort compare
let sorter_desc = sort (fun a b -> compare b a)

type 'a price_calc = { discount: float -> float }
let percentage_discount pct = { discount = fun p -> p *. (1.0 -. pct /. 100.0) }
let fixed_discount amt = { discount = fun p -> max 0.0 (p -. amt) }
```

OCaml's `List.sort` already accepts a comparator — no wrapper struct is needed. Records with function fields serve as lightweight strategy objects.

## Key Differences

1. **Struct vs. record**: Rust uses a `struct` with `Box<dyn Fn>` fields; OCaml uses a record with function fields — semantically identical, syntactically different.
2. **`'static` bound**: Rust's `impl Fn + 'static` prevents strategies from capturing references to local variables with finite lifetimes; OCaml's GC manages all lifetimes.
3. **Trait objects vs. closures**: The classic OOP strategy uses a trait (`trait PricingStrategy`) with separate `struct NoDiscount`, `struct PercentageDiscount` implementations; the closure approach collapses both into a single `Box<dyn Fn>`.
4. **Validator composition**: Rust's `Validator` stores a `Vec<Box<dyn Fn(&T) -> Result<(), String>>>` collecting all rule failures; OCaml would use `List.filter_map` over a list of validation functions.

## Exercises

1. **Swap strategies at runtime**: Implement `fn PriceCalculator::set_strategy(&mut self, new: impl Fn(f64)->f64 + 'static)` and verify that the new strategy takes effect immediately.
2. **Validation pipeline**: Extend `Validator<String>` with rules: `must_not_be_empty`, `max_length(n)`, `must_match_regex(pattern)` — all composed at runtime and all errors collected.
3. **Strategy registry**: Build a `HashMap<String, Box<dyn Fn(f64) -> f64>>` discount registry and implement a `apply_named_discount(name: &str, price: f64) -> Option<f64>` lookup.
