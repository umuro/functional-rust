# 575: let Chains (&&)

**Difficulty:** 3  **Level:** Intermediate

Chain multiple pattern checks with `&&` — combine pattern matching and boolean conditions without nesting.

## The Problem This Solves

Nested `if let` is an anti-pattern that you fall into when you need to match a pattern *and* check a condition *and* match another pattern. Each check adds a level of indentation. Three conditions in: you're three levels deep, the actual logic is at column 20, and the whole thing is hard to read.

The alternative — pulling values out with `match` or `unwrap`, then checking conditions — scatters the validation across multiple statements. You've extracted `host` and `port` already; now you check if they're valid with separate `if` guards. The single logical "is this config valid?" check is fragmented.

`let` chains (stabilized in Rust 1.88) solve this: all the pattern matches and boolean conditions sit in a single `if` condition, flat, left to right. The bindings from earlier `let` in the chain are available in later conditions and in the body.

## The Intuition

A `let` chain is an `if` condition that interleaves `let pattern = expr` with `bool_expr`, all joined by `&&`. Like short-circuit evaluation: if any part fails, the whole condition is false and the else branch runs. But the bound variables from successful `let` bindings carry forward.

```rust
if let Ok(n) = parse(s) && n > 0 && n % 2 == 0 {
    // n is in scope here
}
```

This reads naturally: "if parsing succeeds, giving us `n`, AND `n` is positive, AND `n` is even." Three checks, one line, one binding.

OCaml achieves this with `let*` (monadic binding): `let* n = parse s in let* _ = guard (n > 0) in ...`. That's clean too, but requires a custom `let*` operator. Rust's `let` chains are built-in syntax available in any `if` expression.

The restriction: `let` chains can only appear in `if` and `while` conditions, not in standalone `let` statements (that's what `let-else` is for).

## How It Works in Rust

```rust
// Three checks in one condition — bindings available throughout
fn process(s: &str) -> Option<i32> {
    if let Ok(n) = s.parse::<i32>()  // pattern match
        && n > 0                      // boolean condition using n
        && n % 2 == 0                 // another boolean condition
    {
        Some(n * 2)
    } else {
        None
    }
}

// Multiple pattern lets in one chain
struct Config { host: Option<String>, port: Option<u16> }

fn make_addr(cfg: &Config) -> Option<String> {
    if let Some(ref host) = cfg.host   // bind host
        && let Some(port) = cfg.port   // bind port (note: second let)
        && !host.is_empty()            // use host from earlier
        && port > 0                    // use port from earlier
    {
        Some(format!("{}:{}", host, port))
    } else {
        None
    }
}

// while let chains — loop until mismatch
fn first_positive_even(data: &[&str]) -> Option<i32> {
    for &s in data {
        if let Ok(n) = s.parse::<i32>()
            && n > 0
            && n % 2 == 0
        {
            return Some(n);
        }
    }
    None
}
```

## What This Unlocks

- **Flat multi-condition validation** — combine up to N pattern matches and boolean checks in one readable line.
- **Sequential bindings in one condition** — `let Some(host) = ... && let Some(port) = ...` with both bindings available in the body.
- **Replace nested if-let pyramids** — three levels of nesting become three terms in a `&&` chain.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Chained conditions | `let* x = opt in let* _ = guard cond in ...` | `if let Some(x) = opt && cond { }` |
| Multiple let bindings | Requires `let*` monad | `let Some(a) = e1 && let Some(b) = e2` |
| Boolean mix-in | Guards after binding | Inline `&&` between pattern and bool |
| Scope of bindings | Inside `in` chain | In the body of the `if` |
| Availability | Via ppx or custom operators | Stable since Rust 1.88 |
