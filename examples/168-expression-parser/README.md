# 168: Expression Parser

**Difficulty:** 3  **Level:** Advanced

Parse `1 + 2 * 3` correctly — the Pratt parser, used in every real language implementation.

## The Problem This Solves

Expression parsing looks simple until operator precedence enters the picture. `1 + 2 * 3` must parse as `1 + (2 * 3)` = 7, not `(1 + 2) * 3` = 9. `a ^ b ^ c` must parse as `a ^ (b ^ c)` (right-associative). Unary minus in `-x + y` must bind tightly.

Naive recursive descent needs one grammar rule per precedence level: `expr → additive`, `additive → multiplicative +/- multiplicative`, `multiplicative → unary */÷ unary`, and so on. That's a lot of boilerplate — and adding a new operator means rewriting the grammar.

The Pratt parser solves this elegantly with *binding powers*. Every operator gets a left binding power and a right binding power. The parser loops, consuming operators whose left binding power is strong enough to "pull in" what's already been parsed. It's compact, extensible, and handles any associativity naturally.

## The Intuition

Give each operator a "stickiness" number. Higher stickiness = binds more tightly. `*` has stickiness 30, `+` has stickiness 20. When parsing `1 + 2 * 3`: parse `1`, see `+` (stickiness 20), parse the right side — but the right side sees `2 * 3` and `*` is stickier than `+`, so it grabs both `2` and `3` first, giving `1 + (2 * 3)`.

Right-associativity (`^`) is a trick: give the right binding power one less than the left. That makes the parser "prefer" to recurse right.

## How It Works in Rust

```rust
#[derive(Debug)]
enum Expr {
    Num(f64),
    BinOp { op: char, left: Box<Expr>, right: Box<Expr> },
    Unary { op: char, operand: Box<Expr> },
}

// Returns (left_bp, right_bp) for infix operators
fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    match op {
        '+' | '-' => Some((20, 21)),   // left-assoc: left < right
        '*' | '/' => Some((30, 31)),   // left-assoc, higher precedence
        '^'       => Some((40, 39)),   // right-assoc: right < left (!)
        _         => None,
    }
}

fn pratt_expr(input: &str, min_bp: u8) -> ParseResult<Expr> {
    let input = input.trim_start();

    // Parse prefix: unary minus or atom
    let (mut lhs, mut remaining) = if input.starts_with('-') {
        let (operand, rest) = pratt_expr(&input[1..], 50)?; // tight unary bind
        (Expr::Unary { op: '-', operand: Box::new(operand) }, rest)
    } else if input.starts_with('(') {
        let (inner, rest) = pratt_expr(&input[1..], 0)?;
        let rest = rest.trim_start().strip_prefix(')').ok_or("expected ')'")?;
        (inner, rest)
    } else {
        // parse number
        let (n, rest) = parse_number(input)?;
        (Expr::Num(n), rest)
    };

    // Infix loop: keep consuming operators that bind tightly enough
    loop {
        let rest = remaining.trim_start();
        let op = match rest.chars().next() {
            Some(c) => c,
            None => break,
        };
        let (left_bp, right_bp) = match infix_binding_power(op) {
            Some(bp) => bp,
            None => break,
        };
        if left_bp < min_bp { break; }  // operator doesn't bind tightly enough

        let (rhs, rest) = pratt_expr(&rest[op.len_utf8()..], right_bp)?;
        lhs = Expr::BinOp { op, left: Box::new(lhs), right: Box::new(rhs) };
        remaining = rest;
    }

    Ok((lhs, remaining))
}
```

## What This Unlocks

- **Any expression language** — arithmetic, boolean, bitwise, comparison — all handled by one table.
- **Correct operator precedence** — `1 + 2 * 3 == 7`, not 9, with zero extra grammar rules.
- **Right-associativity** — `a ^ b ^ c == a ^ (b ^ c)` with a one-number tweak.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| AST type | `type expr = Num of float \| BinOp of char * expr * expr` | `enum Expr { Num(f64), BinOp { op: char, ... } }` |
| Heap allocation | Automatic (GC) | `Box::new(...)` required for recursive variants |
| Mutual recursion | `let rec pratt_expr ... and pratt_loop ...` | Two regular functions — no `rec` needed |
| Binding powers | `(int * int)` tuples | `(u8, u8)` tuples — same idea |
