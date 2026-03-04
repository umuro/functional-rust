# 096: Recursive Descent Parser

**Difficulty:** 3  **Level:** Intermediate

Parse arithmetic expressions into an AST using hand-written recursive descent — no parser libraries needed.

## The Problem This Solves

Every language, config format, and query engine needs a parser. You could reach for a parser generator (nom, pest, lalrpop), but understanding recursive descent first makes you a better user of those tools — and sometimes a hand-written parser is the right choice for simple grammars.

Recursive descent is the most readable parsing technique: the grammar rules map directly to functions. One function per grammar rule, functions call each other, and operator precedence falls out naturally from the call hierarchy.

The key insight: `*` binds tighter than `+` because `parse_expr` calls `parse_term`, not the other way around. Higher precedence = deeper in the call stack.

## The Intuition

The grammar `expr = term ('+' expr)?` translates almost word-for-word into code:

```
parse_expr → call parse_term, then check for '+', then call parse_expr again
parse_term → call parse_atom, then check for '*', then call parse_term again
parse_atom → parse a number
```

Each function consumes a prefix of the token list and returns `(parsed_node, remaining_tokens)`. The slice `split_first()` method mirrors OCaml's list head/tail pattern match.

## How It Works in Rust

```rust
#[derive(Debug)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),  // Box for heap allocation — recursive type
    Mul(Box<Expr>, Box<Expr>),
}

// parse_expr handles lowest precedence (+)
fn parse_expr<'a>(tokens: &'a [&str]) -> Result<(Expr, &'a [&str]), String> {
    let (left, rest) = parse_term(tokens)?;  // parse higher-precedence first
    if let Some(("+", rest)) = rest.split_first() {
        let (right, rest) = parse_expr(rest)?;  // right-recursive for +
        Ok((Expr::Add(Box::new(left), Box::new(right)), rest))
    } else {
        Ok((left, rest))
    }
}

// parse_term handles * (tighter than +)
fn parse_term<'a>(tokens: &'a [&str]) -> Result<(Expr, &'a [&str]), String> {
    let (left, rest) = parse_atom(tokens)?;
    if let Some(("*", rest)) = rest.split_first() {
        let (right, rest) = parse_term(rest)?;
        Ok((Expr::Mul(Box::new(left), Box::new(right)), rest))
    } else {
        Ok((left, rest))
    }
}

// parse_atom handles numbers (highest precedence — leaves of the AST)
fn parse_atom<'a>(tokens: &'a [&str]) -> Result<(Expr, &'a [&str]), String> {
    match tokens.split_first() {
        Some((token, rest)) => Ok((Expr::Num(token.parse()?), rest)),
        None => Err("unexpected end of input".into()),
    }
}
```

Evaluate the AST recursively:

```rust
pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}
```

`"2" + "3" * "4"` → `2 + (3 * 4)` = 14. Precedence is correct by construction.

## What This Unlocks

- **DSL parsers** — query languages, config formats, template engines
- **Arithmetic evaluators** — calculator apps, spreadsheet engines
- **Teaching tool** — understand what parser generators generate for you

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutual recursion | `let rec ... and ...` keyword | Functions simply call each other |
| Recursive types | Implicit heap allocation | `Box<Expr>` — explicit heap |
| Token consumption | Pattern match on list head | `split_first()` on slice |
| Error handling | `raise` / `failwith` | `Result<T, String>`, `?` operator |
| Lifetime | Not needed | `'a` on slice refs — ties output to input |
