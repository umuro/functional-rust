# Comparison: Recursive Descent Parser — OCaml vs Rust

## Core Insight

The parser structure is nearly identical: `parse_expr` calls `parse_term` calls `parse_atom`, with each level consuming tokens and returning (AST, remaining_tokens). The key Rust difference is `Box<Expr>` — recursive enum variants must be heap-allocated because Rust needs compile-time size. OCaml's GC handles this transparently.

## OCaml

```ocaml
type expr = Num of int | Add of expr * expr | Mul of expr * expr

let rec parse_expr tokens = 
  let left, rest = parse_term tokens in
  match rest with
  | "+" :: rest' -> let right, rest'' = parse_expr rest' in (Add (left, right), rest'')
  | _ -> (left, rest)
and parse_term tokens =
  let left, rest = parse_atom tokens in
  match rest with
  | "*" :: rest' -> let right, rest'' = parse_term rest' in (Mul (left, right), rest'')
  | _ -> (left, rest)
```

## Rust

```rust
fn parse_expr<'a>(tokens: &'a [&str]) -> Result<(Expr, &'a [&str]), String> {
    let (left, rest) = parse_term(tokens)?;
    if let Some(("+", rest)) = rest.split_first() {
        let (right, rest) = parse_expr(rest)?;
        Ok((Expr::Add(Box::new(left), Box::new(right)), rest))
    } else { Ok((left, rest)) }
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Recursive type | `expr * expr` directly | `Box<Expr>` required |
| Mutual recursion | `and` keyword | Functions call each other |
| Token consumption | `"+" :: rest'` list match | `split_first()` on slice |
| Error handling | `failwith` exception | `Result<T, String>` |
| Tuple return | `(ast, rest)` | `(Expr, &[&str])` with lifetime |
| Parse int | `int_of_string` | `str::parse::<i64>()` |

## Learner Notes

- **`Box<Expr>`**: The #1 surprise for OCaml devs. Rust enums are stack-allocated, so recursive variants need indirection
- **Lifetimes in parsers**: `&'a [&str]` — the output slice borrows from the input, and Rust tracks this
- **`split_first()`**: Returns `Option<(&T, &[T])>` — Rust's equivalent of OCaml's list head/tail destructuring
- **No `and` keyword**: Rust doesn't need it. Forward references work naturally for functions (not for types — use `Box`)
- **`?` operator**: Propagates parse errors elegantly — each `?` is like OCaml's `match ... with Error -> ...`
