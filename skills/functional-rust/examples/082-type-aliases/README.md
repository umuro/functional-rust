# 082: Type Aliases

**Difficulty:** 1  **Level:** Beginner

Give a long or complex type a shorter name with `type` — purely for readability, no new type is created.

## The Problem This Solves

`HashMap<String, Vec<(f64, f64)>>` is correct but exhausting to read and repeat. `Result<T, Box<dyn std::error::Error>>` appears in nearly every async function signature. Without aliases, you copy-paste these types everywhere and hope they stay in sync.

Type aliases let you name complex types once: `type AppResult<T> = Result<T, String>`. Every place you write `AppResult<User>` is exactly the same as writing `Result<User, String>` — the compiler sees them as identical.

The important caveat: `type UserId = u64` does *not* create a new type. `UserId` and `u64` are the same type — you can pass one where the other is expected without any conversion. If you actually need compile-time separation, use the newtype pattern from example 081. Aliases are documentation for humans; newtypes are enforcement for the compiler.

## The Intuition

In Python it's `UserId = int` — a comment more than a constraint. In Java/TypeScript it's `typedef` or `type UserId = number`. In OCaml, `type user_id = int` is also transparent (unless you use a module signature to make it abstract). Rust's `type` alias works the same way: pure documentation, zero enforcement.

Think of aliases as meaningful variable names for types. They make code more readable without changing behavior.

## How It Works in Rust

```rust
// Readable names for common types
type UserId = u64;
type AppResult<T> = Result<T, String>;
type Point = (f64, f64);
type Polygon = Vec<Point>;

// Now function signatures read like English
fn find_user(id: UserId) -> AppResult<User> { ... }
fn area_of(poly: &Polygon) -> f64 { ... }
```

```rust
// Type aliases are transparent — UserId IS u64
let id: UserId = 42;
let raw: u64 = id;   // no conversion needed — they're the same type
```

```rust
// Generic aliases clean up function types
type Validator<T> = fn(&T) -> bool;
type Transform<A, B> = fn(A) -> B;

fn validate_positive(x: &i32) -> bool { *x > 0 }
let v: Validator<i32> = validate_positive;  // reads cleanly
```

```rust
// Box<dyn Fn> aliases reduce repetition
type Predicate<T> = Box<dyn Fn(&T) -> bool>;

fn make_gt(n: i32) -> Predicate<i32> {
    Box::new(move |x| *x > n)
}
```

## What This Unlocks

- **Readable domain types**: `type Timestamp = u64`, `type Matrix = Vec<Vec<f64>>` — the type tells you what the value *means*, not just its structure.
- **Consistent error types**: `type AppResult<T> = Result<T, AppError>` used across a whole crate ensures all functions use the same error type without repetition.
- **Documentation in signatures**: `fn compute(polygon: &Polygon) -> Area` reads like a specification; `fn compute(polygon: &Vec<(f64, f64)>) -> f64` does not.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Transparent alias | `type user_id = int` | `type UserId = u64` |
| Alias vs newtype | `type t = int` (transparent) vs `type t = T of int` (opaque) | `type T = u64` (transparent) vs `struct T(u64)` (newtype) |
| Generic alias | `type ('a, 'b) result = ...` | `type Result<T, E> = ...` |
| Interchangeability | Fully interchangeable | Fully interchangeable |
| New type enforcement | Abstract module type (opaque) | Newtype pattern |
