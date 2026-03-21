[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 096 — Recursive Descent Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Parse arithmetic expressions with addition and multiplication into an AST, respecting operator precedence (`*` binds tighter than `+`). Implement a three-function recursive descent parser (`parse_expr`, `parse_term`, `parse_atom`) on a token slice, returning `(Expr, remaining_tokens)`. Compare with OCaml's mutually recursive `and`-joined functions.

## Learning Outcomes

- Represent ASTs with recursive enums `Expr::Num | Add(Box<Expr>, Box<Expr>) | Mul(...)`
- Use `slice.split_first()` to consume one token at a time without mutation
- Return `(Expr, &[&str])` — the parsed subtree plus the unconsumed token slice
- Apply lifetime annotations `'a` to express that the returned slice borrows the input
- Map Rust's separate function definitions to OCaml's `let rec … and …` mutual recursion
- Understand how operator precedence is encoded in the call hierarchy

## Rust Application

`parse_expr` calls `parse_term` for the left side, then checks if the next token is `"+"`. If so, it recursively calls `parse_expr` for the right side — right-associative addition. `parse_term` does the same for `"*"`. `parse_atom` reads one number token. The return type `Result<(Expr, &'a [&'a str]), String>` threads the unconsumed tokens through the recursion. Lifetime `'a` ties the output slice lifetime to the input slice lifetime.

## OCaml Approach

OCaml uses `let rec parse_expr tokens = … and parse_term tokens = … and parse_atom = …` for mutually recursive functions. Pattern matching on `"+" :: rest'` consumes tokens from the list. The list-based approach is natural in OCaml — cons `::` deconstruction maps directly to the recursive descent pattern. The `eval` function is a separate catamorphism over the `expr` AST.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Token consumption | `slice.split_first()` | List pattern `token :: rest` |
| Mutual recursion | Separate `fn` definitions | `let rec … and …` |
| Error handling | `Result<…, String>` | `failwith` exception |
| Remaining tokens | `&'a [&'a str]` with lifetime | `string list` (by value) |
| AST node allocation | `Box::new(left)` | Native recursive type |
| Precedence encoding | Call hierarchy | Same call hierarchy |

The call hierarchy encodes precedence: `parse_expr` calls `parse_term`, which calls `parse_atom`. Higher-priority operators are parsed deeper in the call stack — multiplication is resolved before addition because `parse_term` is called from `parse_expr`.

## Exercises

1. Add subtraction and division to the parser, keeping `+ -` at lower precedence than `* /`.
2. Add parentheses support: `parse_atom` should handle `"("` by calling `parse_expr` and then expecting `")"`.
3. Add a unary negation operator: `parse_atom` should handle `"-"` followed by another atom.
4. Write a tokeniser `tokenise(s: &str) -> Vec<String>` that splits an arithmetic expression string into tokens.
5. In OCaml, extend the parser with `let x = e in e'` expressions and implement a substitution-based evaluator.
