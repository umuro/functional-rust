📖 **[View on hightechmind.io →](https://hightechmind.io/rust/174-arithmetic-parser)**

---

# 174: Arithmetic Expression Evaluator

**Difficulty:** 3  **Level:** Advanced

Evaluate `3 + 4 * (2 - 1)` directly during parsing — no AST, no intermediate representation, just a number.

## The Problem This Solves

Example 168 built a Pratt parser that produces an AST. Sometimes you don't need the tree — you just want the answer. A calculator, a config file with computed values, a constraint checker: they all want `eval(parse("3 + 4 * 2"))` to return `11.0`.

Classic recursive descent handles this elegantly with *grammar-level precedence*: each precedence level is a separate function. `parse_additive` calls `parse_multiplicative` to get operands, which calls `parse_unary`, which calls `parse_primary`. The call stack *is* the precedence structure. No binding power tables needed.

This approach also makes it easy to add error handling: detect division by zero, check for incomplete expressions, and return `Result<f64, String>` from every parser function. Compare this with the Pratt approach from 168-169 to see two valid paths to the same destination.

## The Intuition

Each grammar level handles one class of operators. Higher functions call lower functions to get their operands, which naturally enforces precedence: `parse_additive` gets its operands from `parse_multiplicative`, so multiplication always binds tighter.

```
"3 + 4 * 2"
parse_additive →
  lhs = parse_multiplicative("3 + 4 * 2") → 3.0 (stops at '+')
  op = '+'
  rhs = parse_multiplicative("4 * 2") → 8.0 (handles '*' internally)
  result = 3.0 + 8.0 = 11.0
```

## How It Works in Rust

```rust
// Entry point — handles + and -
fn parse_additive(input: &str) -> ParseResult<f64> {
    let (mut lhs, mut remaining) = parse_multiplicative(input)?;
    loop {
        let rest = remaining.trim_start();
        if rest.starts_with('+') {
            let (rhs, rest) = parse_multiplicative(&rest[1..])?;
            lhs += rhs;
            remaining = rest;
        } else if rest.starts_with('-') {
            let (rhs, rest) = parse_multiplicative(&rest[1..])?;
            lhs -= rhs;
            remaining = rest;
        } else {
            break;
        }
    }
    Ok((lhs, remaining))
}

// Handles * and /
fn parse_multiplicative(input: &str) -> ParseResult<f64> {
    let (mut lhs, mut remaining) = parse_unary(input)?;
    loop {
        let rest = remaining.trim_start();
        if rest.starts_with('*') {
            let (rhs, rest) = parse_unary(&rest[1..])?;
            lhs *= rhs;
            remaining = rest;
        } else if rest.starts_with('/') {
            let (rhs, rest) = parse_unary(&rest[1..])?;
            if rhs == 0.0 {
                return Err("division by zero".to_string());
            }
            lhs /= rhs;
            remaining = rest;
        } else {
            break;
        }
    }
    Ok((lhs, remaining))
}

// Handles unary minus
fn parse_unary(input: &str) -> ParseResult<f64> {
    let input = input.trim_start();
    if input.starts_with('-') {
        let (val, rest) = parse_unary(&input[1..])?;  // recursive: --- x works
        Ok((-val, rest))
    } else {
        parse_primary(input)
    }
}

// Handles numbers and parenthesized expressions
fn parse_primary(input: &str) -> ParseResult<f64> {
    let input = input.trim_start();
    if input.starts_with('(') {
        let (val, rest) = parse_additive(&input[1..])?;
        let rest = rest.trim_start()
            .strip_prefix(')')
            .ok_or("expected ')'")?;
        Ok((val, rest))
    } else {
        parse_number(input)
    }
}
```

## What This Unlocks

- **Zero-dependency calculator** — evaluate arbitrary arithmetic with `parse_additive(input)?.0`.
- **Config computed values** — let config files say `timeout = 60 * 1000` and evaluate during load.
- **Grammar-level precedence** — understand why the call stack *is* the precedence table.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutual recursion | `let rec eval_expr () = ... and eval_additive () = ...` | Separate named functions — no `rec` annotation needed |
| Left-associative loop | Tail-recursive helper `loop lhs rest` | `loop` + `break` |
| Float operators | `+.` `-.` `*.` `/.` (distinct from int ops) | `+` `-` `*` `/` (same operators for all numeric types) |
| Error value | `Error "division by zero"` | `Err("division by zero".to_string())` |
