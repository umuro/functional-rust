📖 **[View on hightechmind.io →](https://hightechmind.io/rust/623-functional-rust-grand-tour)**

---

# 623: Grand Tour — Functional Programming Patterns in Production Rust

**Difficulty:** 5  **Level:** Master

A complete worked example synthesising the entire functional Rust curriculum: domain types, pure functions, validation, error propagation, iterator pipelines, and composable transformation chains.

## The Problem This Solves

Learning individual patterns in isolation is necessary but not sufficient. The real skill is combining them: algebraic data types that model the domain, pure functions that transform values without side effects, `Result` and `Option` chains that propagate errors without exceptions, and iterator pipelines that process collections without intermediate allocations — all working together in a coherent system.

This example builds an order processing pipeline that a real production system might use. It touches every major functional pattern: `enum`-based error types with `Display`, pure transformation functions, `Result`-based validation, iterator chaining with `flat_map` and `fold`, currency conversion as a pure function, and `HashMap` aggregation. No `unsafe`, no `Arc<Mutex<>>`, no side effects in the core logic — functional purity throughout.

This is where the entire series converges. Every pattern from examples 001–622 exists to make code like this possible: correct, composable, testable without mocks, and readable without tracing control flow through mutations.

## The Intuition

Functional programming in Rust is not about avoiding `mut` or pretending Rust is Haskell. It's about a discipline of design:

1. **Model the domain in types.** If an illegal state is unrepresentable in the type system, you can't create it by accident.
2. **Separate pure computation from side effects.** Functions that transform data are easy to test; functions that print, write to disk, or mutate global state are not.
3. **Use `Result` and `Option` for control flow.** The `?` operator threads errors through call chains without `try/catch` or `null` checks.
4. **Compose with iterators.** An iterator pipeline is a data transformation expressed as a sequence of small, named, testable steps.

## How It Works in Rust

```rust
// ── Domain types: illegal states are unrepresentable ─────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
enum Currency { USD, EUR, GBP }

#[derive(Debug, Clone)]
struct Order { id: String, items: Vec<OrderItem>, discount: f64 }

// ── Error type: all failure modes named and structured ───────────────────
#[derive(Debug)]
enum OrderError {
    InsufficientStock { product: String, requested: u32, available: u32 },
    InvalidDiscount(f64),
    EmptyOrder,
}

// ── Pure functions: no side effects, fully testable ──────────────────────
fn item_subtotal(item: &OrderItem) -> f64 {
    item.product.price.amount * item.qty as f64
}

fn order_total(order: &Order) -> f64 {
    order.items.iter().map(item_subtotal).sum::<f64>() * (1.0 - order.discount)
}

// ── Validation: Result propagation with ? ────────────────────────────────
fn validate_order(order: &Order) -> Result<(), OrderError> {
    if order.items.is_empty() { return Err(OrderError::EmptyOrder); }
    if order.discount < 0.0 || order.discount > 1.0 {
        return Err(OrderError::InvalidDiscount(order.discount));
    }
    for item in &order.items {
        if item.qty > item.product.stock {
            return Err(OrderError::InsufficientStock {
                product: item.product.name.clone(),
                requested: item.qty,
                available: item.product.stock,
            });
        }
    }
    Ok(())
}

// ── Pipeline: validate → transform → aggregate ───────────────────────────
fn process_orders(orders: &[Order]) -> (Vec<&Order>, Vec<(&Order, OrderError)>) {
    let (valid, invalid): (Vec<_>, Vec<_>) = orders.iter()
        .map(|o| validate_order(o).map(|_| o).map_err(|e| (o, e)))
        .partition(Result::is_ok);
    (valid.into_iter().map(|r| r.unwrap()).collect(),
     invalid.into_iter().map(|r| r.unwrap_err()).collect())
}

// ── Aggregation: iterator fold into HashMap ──────────────────────────────
fn revenue_by_currency(orders: &[&Order]) -> HashMap<String, f64> {
    orders.iter().fold(HashMap::new(), |mut acc, order| {
        let currency = order.items.first()
            .map(|i| format!("{:?}", i.product.price.currency))
            .unwrap_or_default();
        *acc.entry(currency).or_insert(0.0) += order_total(order);
        acc
    })
}
```

Each function is: small, named, pure (no side effects), and testable in isolation. The pipeline combines them without ceremony — no class hierarchies, no dependency injection, no mocks needed.

## What This Unlocks

- **Production business logic** — order systems, pricing engines, inventory management, billing pipelines — all naturally modelled as pure transformations over domain types.
- **Concurrent processing** — pure functions are trivially parallelisable with `rayon::par_iter()`: no shared state, no locking, no races.
- **Property-based testing** — pure functions invite `proptest` and `quickcheck`: generate arbitrary inputs, verify algebraic properties hold without writing specific test cases.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Algebraic data types | `type error = InsufficientStock of ...` | `enum OrderError { InsufficientStock { ... } }` |
| Pattern matching | `match e with` | `match e { ... }` — exhaustive, same semantics |
| Error propagation | `Result.bind`, `let*` | `?` operator — desugar to `match` + early return |
| Iterator pipeline | `List.filter_map`, `List.fold_left` | `.filter_map()`, `.fold()`, `.partition()` |
| Pure functions | Default (immutable by default) | Default (move semantics; `mut` is explicit opt-in) |
