📖 **[View on hightechmind.io →](https://hightechmind.io/rust/544-lifetime-closures)**

---

# 544: Lifetimes in Closures

**Difficulty:** 4  **Level:** Intermediate-Advanced

Closures that capture references inherit those references' lifetimes. The `+ 'a` bound on `impl Fn` tells callers how long the returned closure is valid.

## The Problem This Solves

When a closure captures a reference, that reference's lifetime becomes a constraint on the closure itself. Without expressing this constraint, you'd get dangling closures — callbacks that try to access freed data:

```rust
fn make_logger(prefix: &str) -> impl Fn(&str) -> String {
    move |s| format!("{}: {}", prefix, s)
    // ERROR: cannot infer an appropriate lifetime
    // The closure captures `prefix` — but how long is it valid?
}
```

The function doesn't say how long `prefix` lives. The caller might pass a temporary. The closure would then hold a reference to freed stack data. The `+ 'a` bound closes this gap.

## The Intuition

A closure is just a struct with a `call` method. Every captured reference becomes a field. A captured `&'a str` makes the closure's struct contain a `&'a str` field — so the closure itself is only valid for `'a`.

The `impl Fn(&str) -> String + 'a` return type says: "I'm giving you a closure, and that closure borrows data that lives for `'a`. Keep the source data alive as long as you use the closure."

`move` closures move their captures — no reference, no lifetime constraint. But `move` with a reference just moves the reference, not the data, so the lifetime still applies.

## How It Works in Rust

**Capturing a `&str` — must express the bound:**

```rust
fn make_prefixer<'a>(prefix: &'a str) -> impl Fn(&str) -> String + 'a {
    //                                                              ^^^
    // + 'a: this closure borrows `prefix` which lives for 'a
    move |s| format!("{}: {}", prefix, s)
}

// Usage:
let prefix = String::from("INFO");
let log = make_prefixer(&prefix);
println!("{}", log("server started")); // fine — prefix alive
drop(prefix);
// println!("{}", log("again")); // would ERROR — prefix dropped, log invalid
```

**Closure capturing multiple references:**

```rust
fn make_formatter<'a>(prefix: &'a str, suffix: &'a str) -> impl Fn(&str) -> String + 'a {
    // Both captures have lifetime 'a — closure valid for min(prefix, suffix)
    move |s| format!("{}{}{}", prefix, s, suffix)
}
```

**Closure in a struct:**

```rust
struct Filter<'a> {
    predicate: Box<dyn Fn(i32) -> bool + 'a>,
}

impl<'a> Filter<'a> {
    fn from_slice(allowed: &'a [i32]) -> Self {
        Filter {
            // Closure captures &'a [i32] — Box must be + 'a
            predicate: Box::new(move |x| allowed.contains(&x)),
        }
    }

    fn check(&self, x: i32) -> bool { (self.predicate)(x) }
}

let allowed = vec![2, 4, 6, 8];
let filter = Filter::from_slice(&allowed);
assert!(filter.check(4));
assert!(!filter.check(3));
```

**Closures that capture by value — no lifetime issue:**

```rust
fn make_sum_adder(data: &[i32]) -> impl Fn(i32) -> i32 + '_ {
    let sum: i32 = data.iter().sum(); // sum is i32 — Copy
    move |x| x + sum  // captures sum (owned i32), not data — but still borrows data
}
// If we only capture owned Copies, we could return impl Fn + 'static
```

## What This Unlocks

- **Composable, zero-allocation predicates** — build `Filter`, `Mapper`, `Predicate` structs that capture borrowed context. No `Arc` or `'static` requirement when the context is known to outlive the filter.
- **Builder patterns with borrowed config** — a builder that captures a `&Config` can return closures for later execution, as long as the config outlives the closure.
- **Callback APIs without heap allocation** — return `impl Fn + 'a` for callbacks that reference local data, avoiding `Box<dyn Fn + 'static>` heap allocation when the lifetime scope is known.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Closure captures | GC manages all captured values — no lifetime annotation | Captured references become lifetime constraints on the closure |
| Returning closures | Free — GC handles any captured data | Must express `+ 'a` if closure captures borrowed data |
| Stored callbacks | `'a ref` pattern or first-class closures with GC | `Box<dyn Fn + 'a>` — explicit lifetime on the stored closure |
| Partial application | Currying — clean, GC-managed | Closure captures `&'a T` — valid for `'a`. Move closures capture copies when possible |
| Lambda hoisting | GC: closures can outlive their creation scope | `'a` bound enforces: closure can't outlive its captured references |
