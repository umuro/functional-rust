# 063: Monad Transformers

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Stack `Option` inside `Result` to handle computations that can either fail hard (error) or find nothing — and compare the formal transformer pattern with Rust's idiomatic `?` + early return.

## The Problem This Solves

You're writing a database lookup function. It can fail in two distinct ways:

1. **Hard failure**: the database connection is down → `Err("DB error")`
2. **Soft failure**: the record doesn't exist → `None` (not an error, just absent)

If you use `Result<T, E>`, you lose the distinction between "not found" and "error". If you use `Option<T>`, you lose error details. If you nest them naively:

```rust
fn find_user(id: i32) -> Result<Option<User>, DbError> { ... }
fn find_email(user: &User) -> Result<Option<String>, DbError> { ... }

// Composing them is a nightmare:
fn get_user_email(id: i32) -> Result<Option<String>, DbError> {
    match find_user(id) {
        Err(e) => Err(e),
        Ok(None) => Ok(None),
        Ok(Some(user)) => match find_email(&user) {
            Err(e) => Err(e),
            Ok(None) => Ok(None),
            Ok(Some(email)) => Ok(Some(email)),
        },
    }
}
```

Every additional step adds another nested `match`. The pattern `Err(e) => Err(e)` and `Ok(None) => Ok(None)` repeats everywhere. The actual logic (call `find_user`, then `find_email`) is buried in boilerplate.

Monad transformers formalise this stacking — and Rust's `?` + early `return Ok(None)` gives you the same benefit idiomatically. This exists to solve exactly that pain.

## The Intuition

A **monad transformer** is a way to stack two monadic effects in a single computation.

Think of it layer by layer:
- `Result<T, E>` adds "can fail with an error"
- `Option<T>` adds "might be absent"
- `Result<Option<T>, E>` stacks both: can fail hard *or* find nothing

The transformer, `OptionT`, wraps one monad around another:

```
OptionT<E, A> = Result<Option<A>, E>
```

Operations on this type need to handle *three* cases:

| Case | Meaning |
|------|---------|
| `Err(e)` | Hard failure — stop everything |
| `Ok(None)` | Soft absence — propagate "not found" |
| `Ok(Some(a))` | Success — continue with `a` |

The **key insight**: `bind` (the `and_then` of this combined monad) threads through all three cases automatically:

```
bind(m, f):
    Err(e)    → Err(e)        ← propagate hard error
    Ok(None)  → Ok(None)      ← propagate soft absence
    Ok(Some a)→ f(a)          ← continue with value
```

**Rust's idiomatic answer**: the `?` operator handles `Err(e) → return Err(e)`. For `Ok(None)`, you write `return Ok(None)` explicitly. This is cleaner than formal transformers in most Rust code — but understanding transformers explains *why* `?` plus early return covers all the cases.

**Analogy:** Monad transformers are like USB adapters stacked on each other. One layer handles power (error), another handles data (option). Each layer adds one capability. But stacking too many layers (3+) becomes unwieldy — at that point, Rust's custom error enum with `?` is usually the better tool.

## How It Works in Rust

```rust
// Step 1: The type alias — OptionT is just Result<Option<A>, E>
type OptionT<A, E> = Result<Option<A>, E>;

// Step 2: The three fundamental operations
mod option_t {
    // Return a value — wraps in Some and Ok
    pub fn pure<A, E>(a: A) -> Result<Option<A>, E> { Ok(Some(a)) }

    // Return "not found" — Ok but None
    pub fn none<A, E>() -> Result<Option<A>, E> { Ok(None) }

    // bind: the key operation — threads through all three cases
    pub fn bind<A, B, E>(
        m: Result<Option<A>, E>,
        f: impl FnOnce(A) -> Result<Option<B>, E>,
    ) -> Result<Option<B>, E> {
        match m {
            Err(e)       => Err(e),     // hard error: propagate up
            Ok(None)     => Ok(None),   // soft absence: propagate up
            Ok(Some(a))  => f(a),       // success: continue with a
        }
    }

    // Lift a Result<A, E> into OptionT — wraps Ok(a) in Some
    pub fn lift_result<A, E>(r: Result<A, E>) -> Result<Option<A>, E> { r.map(Some) }

    // Lift an Option<A> into OptionT — wraps it in Ok
    pub fn lift_option<A, E>(o: Option<A>) -> Result<Option<A>, E> { Ok(o) }
}

// Step 3: Database functions that return OptionT
fn find_user(id: i32) -> Result<Option<String>, String> {
    if id > 0  { Ok(Some(format!("User_{}", id))) }   // found
    else if id == 0 { Ok(None) }                       // not found (soft)
    else       { Err("Invalid ID".into()) }            // error (hard)
}

fn find_email(name: &str) -> Result<Option<String>, String> {
    match name {
        "User_1" => Ok(Some("user1@example.com".into())),  // found
        "User_2" => Ok(None),                               // no email on record
        _        => Err("DB connection failed".into()),     // hard error
    }
}

// Step 4: Using bind for composition — explicit transformer style
fn get_user_email(id: i32) -> Result<Option<String>, String> {
    option_t::bind(
        find_user(id),         // Step 1: find user
        |name| find_email(&name),  // Step 2: find email (only if user found)
    )
    // No nested match — bind handles the three cases automatically
}

get_user_email(1);   // Ok(Some("user1@example.com"))
get_user_email(0);   // Ok(None)  — user not found (soft)
get_user_email(-1);  // Err("Invalid ID")  — hard error

// Step 5: Idiomatic Rust — same result, cleaner syntax
fn get_user_email_idiomatic(id: i32) -> Result<Option<String>, String> {
    // ? handles Err propagation (the "hard failure" case)
    let user = match find_user(id)? {     // ? propagates Err
        Some(u) => u,
        None    => return Ok(None),        // explicit: propagate "not found"
    };
    find_email(&user)
    // Both versions produce identical results — the idiomatic one is usually preferred
}

// Step 6: map — transform the inner value without unwrapping
fn map<A, B, E>(m: Result<Option<A>, E>, f: impl FnOnce(A) -> B) -> Result<Option<B>, E> {
    match m {
        Err(e)      => Err(e),
        Ok(None)    => Ok(None),
        Ok(Some(a)) => Ok(Some(f(a))),  // apply f only to the Some value
    }
}

// Usage:
let upper = map(get_user_email(1), |s| s.to_uppercase());
// Ok(Some("USER1@EXAMPLE.COM"))
```

## What This Unlocks

- **Principled error layering** — when a computation has exactly two failure modes (hard error + soft absence), `Result<Option<T>, E>` is the right type, and `bind`/`map` make composition clean without nested matches.
- **Understanding `?`** — the `?` operator is sugar for the `Err` arm of `bind`. Knowing transformer `bind` explains why `?` works, and why you need an explicit `return Ok(None)` for the `None` arm.
- **Scaling up** — if you need *three* stacked effects (state + error + logging), formal transformers become unwieldy and Rust's custom error enum with `?` wins. Understanding transformers shows *exactly where that threshold is*.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| OptionT type | Type alias `type ('a, 'e) option_t = ('a option, 'e) result` | `type OptionT<A, E> = Result<Option<A>, E>` |
| `bind` | Pattern match + `>>=` operator | `match` with three arms — no HKT so no generic `bind` |
| Method syntax | Module functions: `OptionT.bind`, `OptionT.pure` | Free functions in a `mod option_t {}` module (can't add methods to type aliases) |
| Lifting | `lift_result : ('a, 'e) result -> ('a, 'e) option_t` | `lift_result(r: Result<A,E>) -> Result<Option<A>,E>` — same logic |
| Idiomatic alternative | Formal transformers are natural in OCaml's type class style | `?` + `return Ok(None)` is cleaner in Rust — prefer it over formal transformers in production code |
