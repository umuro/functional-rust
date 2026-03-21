📖 **[View on hightechmind.io →](https://hightechmind.io/rust/573-pattern-let-else)**

---

# let-else Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Deeply nested `if let` expressions create the "pyramid of doom" — code that drifts rightward with each additional unwrap. `let-else` (stabilized in Rust 1.65) provides early return on pattern mismatch: if the pattern does not match, the `else` block must diverge (return, break, continue, or panic). This enables "railway-oriented" linear code — extract what you need at the top, return on failure, use the value in the rest of the function. It is the idiomatic Rust replacement for chains of nested `if let`.

## Learning Outcomes

- How `let Pattern = expr else { return; }` extracts a value or exits early
- How `let-else` flattens `if let` nesting to linear code
- How `let-else` works with `Option`, `Result`, slice patterns, and struct destructuring
- Why the `else` block must diverge: `return`, `break`, `continue`, `panic!`, or `loop { ... }`
- Where `let-else` is most useful: input validation, argument parsing, early-out functions

## Rust Application

`get_first(v: &[i32])` uses `let [first, ..] = v else { return -1; }` — slice pattern with early exit. `process_option(opt: Option<i32>)` uses `let Some(value) = opt else { return 0; }` — unwrap or return. `process_result(res: Result<i32, &str>)` uses `let Ok(value) = res else { return -1; }`. The pattern and expression are evaluated; if they match, the bound variable is available for the rest of the function; if not, the `else` branch runs.

Key patterns:
- `let Some(x) = opt else { return default; };` — Option early exit
- `let Ok(v) = res else { return Err(...); };` — Result unwrap-or-return
- `let [first, ..] = slice else { return None; };` — slice head extraction
- `else` must diverge — compile error if it can fall through

## OCaml Approach

OCaml's equivalent uses `match` with early return via `Option.value` or explicit pattern:

```ocaml
let process_option opt =
  match opt with
  | None -> 0
  | Some value -> value * 2
```

OCaml does not have `let-else` syntax — the `match` expression achieves the same semantics.

## Key Differences

1. **Syntax**: Rust `let Pattern = expr else { ... }` is linear code; OCaml requires a `match` expression, potentially creating nesting.
2. **Divergence requirement**: Rust's `else` block must diverge — the compiler enforces this; OCaml's `None` arm can return any value (not just diverging ones).
3. **Scope of binding**: Rust's `let-else` makes the bound value available for the rest of the function (not just inside a then-block); OCaml `match` scopes the binding to each arm.
4. **Readability impact**: `let-else` is praised for enabling "happy path" linear code; OCaml `match` at the top of a function achieves similar clarity with explicit arms.

## Exercises

1. **Parse int**: Write `fn parse_positive(s: &str) -> Option<u32>` using `let-else` to unwrap `str::parse::<u32>()` and return `None` if parsing fails or the result is zero.
2. **Multi-level extraction**: Write a function with three sequential `let-else` statements extracting from a `Config` struct — show that each extraction can use variables from previous ones.
3. **Slice parsing**: Implement `fn parse_rgb(parts: &[&str]) -> Option<(u8, u8, u8)>` using `let [r_str, g_str, b_str] = parts else { return None; }` followed by individual parse calls.
