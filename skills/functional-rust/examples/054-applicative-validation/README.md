# 054: Applicative Validation

**Difficulty:** 2  **Level:** Intermediate

Collect *all* validation errors in one pass instead of stopping at the first failure.

## The Problem This Solves

You're building a signup form. User submits: empty name, invalid age, malformed email — three mistakes. Your backend validates with `?` chaining and returns: "Name cannot be empty." User fixes the name, resubmits. Now you return: "Age must be between 0 and 150." User fixes that, resubmits. Now you return: "Email must contain @."

Three round trips for three errors that were all present from the start. This is the `?` operator's fault — it *short-circuits*. The moment it hits the first `Err`, it returns early. It never sees the other fields.

```rust
// This is the problem — ? short-circuits:
fn validate_user(name: &str, age: i32, email: &str) -> Result<User, String> {
    let name  = validate_name(name)?;   // Stops here if name is bad
    let age   = validate_age(age)?;     // Never reached if name failed
    let email = validate_email(email)?; // Never reached if name or age failed
    Ok(User { name, age, email })
}
// Input: ("", -5, "bad") → only returns the name error. Age and email errors are lost.
```

The fix requires a different type that knows how to *accumulate* errors instead of stopping. `Result` can't do this — `?` is its entire design. You need `Validated<T, E>`: either a valid value, or a *list* of errors. When you combine two `Invalid` values, you get back *all* their errors merged together.

This is the killer use case for applicatives. Applicative combination is exactly the right tool here because the validations are *independent* — name doesn't depend on age, email doesn't depend on name. Independent effects → applicative. This exists to solve exactly that pain.

## The Intuition

Think of `Result` as a train that derails at the first problem. Once it derails, the journey stops. `Validated` is a checklist — you go through every item, tick the passes, collect the failures, and only at the end do you decide: "good to go" or "here are all the problems."

The key insight: when you combine two `Invalid` values applicatively, you *merge the error lists*. When one is `Valid` and the other is `Invalid`, the errors win. Only when *both* are `Valid` does the combination succeed.

```rust
// Invalid("name empty") combined with Invalid("age bad")
// → Invalid(["name empty", "age bad"])   ← BOTH errors kept

// Valid("Alice") combined with Invalid("age bad")
// → Invalid(["age bad"])                 ← error wins

// Valid("Alice") combined with Valid(30)
// → Valid(("Alice", 30))                 ← success only when all pass
```

**Jargon decoded:**
- *Monadic error handling* — `Result` with `?`: stops at first error (fast, great for "bail early" logic)
- *Applicative error handling* — `Validated`: collects all errors (slower, great for "tell user everything" logic)
- *Accumulating* — instead of returning on first failure, we keep going and collect failures
- *Independent effects* — each validation doesn't depend on another's result (that's why applicative works here)

## How It Works in Rust

```rust
#[derive(Debug, PartialEq, Clone)]
enum Validated<T, E> {
    Valid(T),
    Invalid(Vec<E>),  // A LIST of errors, not just one
}
```

```rust
// lift3: validate three independent fields, collect ALL errors
fn lift3<A, B, C, D, E, F: FnOnce(A, B, C) -> D>(
    f: F,
    a: Validated<A, E>,
    b: Validated<B, E>,
    c: Validated<C, E>,
) -> Validated<D, E> {
    let mut errors = Vec::new();

    // Evaluate ALL fields — no short-circuiting
    let a = match a {
        Validated::Valid(v)   => Some(v),
        Validated::Invalid(e) => { errors.extend(e); None }  // collect, keep going
    };
    let b = match b {
        Validated::Valid(v)   => Some(v),
        Validated::Invalid(e) => { errors.extend(e); None }  // collect, keep going
    };
    let c = match c {
        Validated::Valid(v)   => Some(v),
        Validated::Invalid(e) => { errors.extend(e); None }  // collect, keep going
    };

    // Only now decide: succeed or return ALL collected errors
    if errors.is_empty() {
        Validated::Valid(f(a.unwrap(), b.unwrap(), c.unwrap()))
    } else {
        Validated::Invalid(errors)
    }
}
```

```rust
// Individual validators — each returns Validated, not Result
fn validate_name(s: &str) -> Validated<String, String> {
    if !s.is_empty() {
        Validated::Valid(s.to_string())
    } else {
        Validated::Invalid(vec!["Name cannot be empty".to_string()])
    }
}

fn validate_age(n: i32) -> Validated<i32, String> {
    if (0..=150).contains(&n) {
        Validated::Valid(n)
    } else {
        Validated::Invalid(vec!["Age must be between 0 and 150".to_string()])
    }
}

fn validate_email(s: &str) -> Validated<String, String> {
    if s.contains('@') {
        Validated::Valid(s.to_string())
    } else {
        Validated::Invalid(vec!["Email must contain @".to_string()])
    }
}
```

```rust
// Combining them with lift3 — all three are evaluated regardless
fn validate_user(name: &str, age: i32, email: &str) -> Validated<User, String> {
    lift3(
        |name, age, email| User { name, age, email },
        validate_name(name),
        validate_age(age),
        validate_email(email),
    )
}

// Results:
validate_user("Alice", 30, "alice@example.com")
// → Valid(User { name: "Alice", age: 30, email: "alice@example.com" })

validate_user("", 30, "alice@example.com")
// → Invalid(["Name cannot be empty"])

validate_user("", -5, "bad")
// → Invalid(["Name cannot be empty", "Age must be between 0 and 150", "Email must contain @"])
// ALL THREE errors in one shot
```

## What This Unlocks

- **Form validation APIs:** Return all field errors in a single response instead of one per request. Your frontend can highlight every broken field at once.
- **Config file parsing:** Validate a TOML/YAML config and report every missing or malformed field in one error message instead of forcing the user to fix-rerun-fix-rerun.
- **Batch data import:** Validate a CSV row's fields independently and collect all column errors before deciding whether to reject the row.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Short-circuit errors | `Result` with bind | `Result` with `?` |
| Accumulating errors | Custom `Validated` type with `Invalid of 'e list` | Custom `Validated<T, E>` with `Invalid(Vec<E>)` |
| Combining validations | `pure f <*> a <*> b <*> c` (infix, curried) | `lift3(f, a, b, c)` (explicit function) |
| Error merging | `@` (list concatenation) | `Vec::extend` (move and append) |
| In stdlib? | No — must implement | No — must implement (`Result` can't accumulate) |
| When to use `?` | Fast-fail pipelines where order matters | Same — `Validated` is not a replacement, it's a different tool |
