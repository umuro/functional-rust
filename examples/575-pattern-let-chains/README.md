📖 **[View on hightechmind.io →](https://hightechmind.io/rust/575-pattern-let-chains)**

---

# Let Chains (&&)

## Problem Statement

Multiple sequential pattern checks create nesting: `if let Some(x) = a { if let Ok(y) = b { if condition { ... } } }`. This pyramid of doom is hard to read and hard to maintain. Let chains (stabilized in Rust 1.88) allow combining multiple `let` bindings and Boolean conditions in a single flat `if` expression using `&&`. This is the Rust equivalent of Haskell's `do` notation or OCaml's `match` nesting, enabling linear "happy path" code for multi-step extraction.

## Learning Outcomes

- How `if let P = x && let Q = y && cond { }` chains multiple pattern checks
- How let chains short-circuit: if a pattern fails, later patterns are not evaluated
- How to mix `let` pattern bindings with Boolean conditions in one chain
- How let chains replace nested `if let` without requiring `let-else`
- Where let chains improve: argument validation, multi-field config extraction, parser steps

## Rust Application

`process(s: &str) -> Option<i32>` uses:
```rust
if let Ok(n) = s.parse::<i32>() && n > 0 && n % 2 == 0 { Some(n * 2) } else { None }
```
Three conditions combined: parse succeeds, result is positive, result is even. The chain short-circuits — if parsing fails, `n > 0` is never evaluated. Variables bound in earlier `let` patterns are available in later conditions and the body.

Key patterns:
- `if let P = x && let Q = y { ... }` — two pattern bindings chained
- `if let P = x && condition { ... }` — pattern binding plus boolean
- Short-circuit evaluation left to right
- All bound variables visible in body

## OCaml Approach

OCaml achieves the same with `Option.bind` chains or nested `match`:

```ocaml
let process s =
  let open Option in
  let* n = int_of_string_opt s in
  let* () = if n > 0 && n mod 2 = 0 then Some () else None in
  Some (n * 2)
```

OCaml 4.08+ `let*` (monadic bind) provides a similar linear chaining style.

## Key Differences

1. **Stabilization**: Rust let chains were stabilized in 1.88 — a relatively recent addition; OCaml's `let*` notation for monadic chaining has been available since 4.08.
2. **Mix of patterns and conditions**: Rust lets you freely mix `let P = x` and `cond` in the same chain; OCaml's `let*` is purely monadic, requiring conditions to be lifted into `Option`.
3. **Short-circuit**: Both Rust let chains and OCaml `Option.bind` short-circuit on the first failure.
4. **Scope**: Rust let-chain bindings are visible to subsequent parts of the same chain and the body; OCaml `let*` bindings are visible to the continuation.

## Exercises

1. **Config extraction**: Write `fn get_server_addr(config: &Config) -> Option<String>` using let chains to extract and validate `host`, `port`, and `max_conn` from an `Option<DbConfig>`.
2. **Multi-parse**: Implement `fn parse_coord_pair(s: &str) -> Option<(f64, f64)>` using let chains to split on comma, parse both parts, and check they are in valid range.
3. **Pre-1.88 equivalent**: Rewrite the `process` function using nested `if let` and explain why let chains improve readability — count the nesting levels.
