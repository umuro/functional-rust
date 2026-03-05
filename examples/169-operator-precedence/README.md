# 169: Operator Precedence

**Difficulty:** 3  **Level:** Advanced

Build a full operator table — multi-character operators, multiple precedence levels, and two equivalent parsing algorithms.

## The Problem This Solves

Real languages have many operators: `+`, `*`, `==`, `!=`, `<=`, `>=`, `&&`, `||`. They all have different precedence levels. `&&` binds more tightly than `||`. Comparison operators bind more tightly than `&&`. Arithmetic binds more tightly than comparison. Getting this wrong causes subtle bugs: `a && b || c` parsed as `a && (b || c)` instead of `(a && b) || c`.

Multi-character operators introduce another problem: `>=` must be matched before `>`, or you'll parse `>=` as `>` followed by `=` — which is wrong. Longest-match-first is the rule.

This example extends the Pratt parser from example 168 with a proper operator table, two-character operators, and also shows *precedence climbing* — a different algorithm for the same problem, proving they're equivalent.

## The Intuition

Replace the hardcoded `match op { '+' => (20, 21), ... }` with a table of `OpInfo` structs. Each entry has the symbol, precedence level, and associativity. Computing binding powers from precedence + associativity is straightforward: left-associative at level N → `(2*N, 2*N+1)`, right-associative → `(2*N+1, 2*N)`.

For multi-character operators: scan for 2-char operators before 1-char operators.

## How It Works in Rust

```rust
#[derive(Clone, Copy, Debug)]
enum Assoc { Left, Right }

#[derive(Debug)]
struct OpInfo {
    symbol: &'static str,
    precedence: u8,
    assoc: Assoc,
}

const OPERATORS: &[OpInfo] = &[
    OpInfo { symbol: "||", precedence: 1, assoc: Assoc::Left },
    OpInfo { symbol: "&&", precedence: 2, assoc: Assoc::Left },
    OpInfo { symbol: "==", precedence: 3, assoc: Assoc::Left },
    OpInfo { symbol: "!=", precedence: 3, assoc: Assoc::Left },
    OpInfo { symbol: "<=", precedence: 4, assoc: Assoc::Left },
    OpInfo { symbol: ">=", precedence: 4, assoc: Assoc::Left },
    OpInfo { symbol: "<",  precedence: 4, assoc: Assoc::Left },
    OpInfo { symbol: ">",  precedence: 4, assoc: Assoc::Left },
    OpInfo { symbol: "+",  precedence: 5, assoc: Assoc::Left },
    OpInfo { symbol: "-",  precedence: 5, assoc: Assoc::Left },
    OpInfo { symbol: "*",  precedence: 6, assoc: Assoc::Left },
    OpInfo { symbol: "/",  precedence: 6, assoc: Assoc::Left },
    OpInfo { symbol: "^",  precedence: 7, assoc: Assoc::Right },
];

// Try 2-char operators first, then 1-char — longest match wins
fn find_op(input: &str) -> Option<&'static OpInfo> {
    // 2-char pass
    for op in OPERATORS {
        if op.symbol.len() == 2 && input.starts_with(op.symbol) {
            return Some(op);
        }
    }
    // 1-char pass
    for op in OPERATORS {
        if op.symbol.len() == 1 && input.starts_with(op.symbol) {
            return Some(op);
        }
    }
    None
}

// Convert precedence + associativity to (left_bp, right_bp)
fn binding_power(op: &OpInfo) -> (u8, u8) {
    let p = op.precedence * 2;
    match op.assoc {
        Assoc::Left  => (p, p + 1),  // right bp slightly higher → left-assoc
        Assoc::Right => (p + 1, p),  // left bp slightly higher → right-assoc
    }
}
```

## What This Unlocks

- **Language-grade operator tables** — extend to 20+ operators by adding rows to the table.
- **Two parsing algorithms** — Pratt and precedence climbing produce identical ASTs; pick whichever reads more clearly to you.
- **Multi-character operators** — `==`, `!=`, `<=`, `>=`, `&&`, `||` all handled correctly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Operator table | `op_info list` with record syntax | `&[OpInfo]` constant slice |
| Record | `{ symbol: string; precedence: int; assoc: assoc }` | `struct OpInfo { symbol: &'static str, ... }` |
| Associativity | `type assoc = Left \| Right` | `enum Assoc { Left, Right }` with `#[derive(Clone, Copy)]` |
| Longest-match | Try longer strings first in list | Same — scan `OPERATORS` for 2-char before 1-char |
