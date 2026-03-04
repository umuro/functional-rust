# 292: Option Combinators

**Difficulty:** 1  **Level:** Beginner

Work with optional values using `.map()`, `.filter()`, `.and_then()`, and `.unwrap_or()` — without checking for `None` every single time.

## The Problem This Solves

Every programmer knows the pain of null checks. In Python you write `if user is not None:`, in Java you get `NullPointerException` at runtime, in JavaScript `undefined` quietly propagates through your code for three function calls before you notice.

Rust's `Option<T>` is its solution: a value is either `Some(x)` (it exists) or `None` (it doesn't). The compiler forces you to handle both cases. But if you use `match` every time you want to touch an optional value, your code becomes verbose:

```rust
match user.get("alice") {
    Some(age) => match age.checked_mul(*age) {
        Some(squared) => println!("{}", squared),
        None => println!("overflow"),
    },
    None => println!("user not found"),
}
```

Option combinators flatten this into a single readable chain. The `None` case propagates automatically — you only describe the happy path.

## The Intuition

If you've used JavaScript's optional chaining (`user?.age`) or Python's walrus operator, you're already thinking the right way. Option combinators just make it explicit and type-safe.

Think of `Option` like a box that might be empty:
- `.map(f)` — if the box has something, transform it; if empty, stay empty
- `.filter(pred)` — if the box has something but it doesn't pass the test, empty it out
- `.and_then(f)` — if the box has something, run `f` (which itself might return an empty box)
- `.unwrap_or(default)` — open the box; if empty, use the default instead

`.and_then()` is the key one: it's for when the next step *also* might return nothing (like looking up a config value that might not exist, then parsing it as a number that might not be valid).

## How It Works in Rust

```rust
fn safe_sqrt(x: f64) -> Option<f64> {
    if x >= 0.0 { Some(x.sqrt()) } else { None }
}

let some5: Option<i32> = Some(5);
let none: Option<i32> = None;

// .map(): transform Some, leave None alone
some5.map(|x| x * 2)  // Some(10)
none.map(|x| x * 2)   // None — the closure never runs

// .filter(): keep Some only if the value passes the test
Some(5i32).filter(|&x| x % 2 == 0)  // None  — 5 is odd
Some(6i32).filter(|&x| x % 2 == 0)  // Some(6) — 6 is even

// .and_then(): chain optional computations
// Each step can return None, short-circuiting the rest
let result = Some("4.0")
    .and_then(|s| s.parse::<f64>().ok())  // parse string → None if invalid
    .and_then(safe_sqrt);                  // sqrt → None if negative
// Some(2.0)

// None short-circuits the whole chain
let result2: Option<f64> = None
    .and_then(|s: &str| s.parse::<f64>().ok())
    .and_then(safe_sqrt);
// None — nothing runs after the first None

// .or() and .or_else(): provide fallbacks
none.or(Some(42))              // Some(42)
none.or_else(|| Some(99))      // Some(99) — or_else is lazy (closure runs only if needed)

// Real example: HashMap lookup → transform
use std::collections::HashMap;
let users = HashMap::from([("alice", 30u32)]);
let age_squared = users.get("alice").map(|&age| age * age);
// Some(900)
let missing    = users.get("bob").map(|&age| age * age);
// None
```

The `&` inside `.filter(|&x| ...)` pattern-matches the reference — it's pulling the value out of the `&i32`. You'll see this often with Option and iterator closures.

## What This Unlocks

- **Safe config access** — `config.get("port").and_then(|s| s.parse().ok()).unwrap_or(8080)` in one line
- **Database lookups** — look up a user, get their settings, get a specific field — each step might return None, and the chain handles all cases
- **Input validation** — `.filter()` rejects values that don't meet a condition without extra `if let` noise

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map Some value | `Option.map f opt` | `opt.map(f)` |
| Chain optional ops | `Option.bind opt f` | `opt.and_then(f)` |
| Filter by predicate | Manual `match` | `opt.filter(pred)` |
| Default value | `Option.value ~default opt` | `opt.unwrap_or(default)` |
| Lazy fallback | `Option.value_or_thunk` | `opt.unwrap_or_else(f)` |
