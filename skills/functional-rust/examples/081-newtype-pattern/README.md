# 081: Newtype Pattern

**Difficulty:** 2  **Level:** Intermediate

Wrap a primitive in a single-field tuple struct to create a distinct type the compiler enforces — preventing you from accidentally mixing `UserId` with `OrderId` even though both are `u64`.

## The Problem This Solves

Functions that take multiple `u64` parameters are accidents waiting to happen. `process_order(user_id, order_id)` compiles equally well as `process_order(order_id, user_id)` — you only discover the bug at runtime, or not at all.

The same problem exists with units. Adding Celsius temperatures to Fahrenheit is a type error that only shows up when your code produces obviously wrong results. The famous Mars Climate Orbiter was lost because of a Newtons vs pound-force mix-up — a bug that a type system would have caught.

Rust's newtype pattern uses a one-field tuple struct: `struct UserId(u64)`. The inner value is still a `u64`, but `UserId` and `OrderId` are different types at compile time. You must explicitly extract the inner value — accidental misuse becomes a compile error.

## The Intuition

In Python or JavaScript, you'd add a comment: `# this is a user id, not an order id`. In Haskell and OCaml, newtypes are a standard tool. In Java, you'd write a thin wrapper class (verbose but effective). In Rust, a tuple struct is zero-cost: `struct UserId(u64)` compiles down to exactly the same memory layout as a bare `u64` — no overhead, full type safety.

The key insight: the newtype doesn't just rename the type. It creates a *new* type that is not interchangeable with the original. You can't accidentally pass a `UserId` where an `OrderId` is expected.

## How It Works in Rust

```rust
// Zero-cost wrappers — same memory layout as u64
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OrderId(u64);

// Compiler now prevents mixing these up — this is a TYPE ERROR:
// process_order(user_id, order_id) ← args are in the right positions
// process_order(order_id, user_id) ← compile error: expected UserId, found OrderId
fn process_order(order: OrderId, user: UserId) -> String {
    format!("{:?} placed {:?}", user, order)
}
```

```rust
// Validation at construction time — invalid values can never exist
struct Email(String);

impl Email {
    fn new(s: &str) -> Option<Self> {
        if s.contains('@') { Some(Email(s.to_string())) } else { None }
    }
}
// Once you have an Email, you know it was validated
```

```rust
// Units — Celsius and Fahrenheit can't be accidentally added
struct Celsius(f64);
struct Fahrenheit(f64);

impl Celsius {
    fn to_fahrenheit(self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}
// let temp: Celsius = body_temp + room_temp   ← type error if units differ
```

```rust
// Deref for transparent access when appropriate
impl Deref for NonEmptyString {
    type Target = str;
    fn deref(&self) -> &str { &self.0 }
}
// Now all &str methods work on NonEmptyString automatically
```

## What This Unlocks

- **Type-safe IDs everywhere**: `UserId`, `ProductId`, `SessionId` — all `u64` underneath but never interchangeable, eliminating an entire class of "wrong argument order" bugs.
- **Validated types as values**: construct `Email`, `PhoneNumber`, `PositiveInt` only through smart constructors that return `Option` or `Result` — once you have the value, validation has already happened.
- **Unit safety in physical simulations, finance, science**: `Meters`, `Seconds`, `Dollars(Decimal)` prevent accidental unit mixing with zero runtime cost.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Newtype definition | `type user_id = UserId of int` | `struct UserId(u64)` |
| Inner value access | Pattern match: `let UserId n = id` | `id.0` or custom accessor |
| Runtime cost | Zero (optimized away) | Zero (same memory layout) |
| Auto-derive traits | Must implement manually | `#[derive(Debug, Clone, Copy, ...)]` |
| Transparent access | Automatic coercion sometimes | Explicit `Deref` impl |
