📖 **[View on hightechmind.io →](https://hightechmind.io/rust/771-pratt-parser)**

---

# 771-pratt-parser — Pratt Parser
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Recursive descent parsers encode operator precedence as a grammar rule hierarchy, requiring a separate parsing function per precedence level. Vaughan Pratt's 1973 "Top-Down Operator Precedence" technique encodes precedence as integer "binding powers" on operators, enabling a single compact parsing loop to handle any precedence and associativity. It is used in rustc's expression parser, TypeScript's compiler, and the Crafting Interpreters book. Pratt parsing is often the first thing compiler engineers reach for when hand-writing expression parsers.

## Learning Outcomes

- Understand binding power as a mechanism for encoding precedence and associativity
- Implement `infix_binding_power` returning `(left_bp, right_bp)` pairs
- Implement `prefix_binding_power` for unary operators
- Write a single `parse_expr(min_bp)` loop that handles all binary and unary operators
- See how left-associativity (`1+2+3` = `(1+2)+3`) and right-associativity (`2^3^4` = `2^(3^4)`) emerge naturally from the binding power values

## Rust Application

`Token` includes `Number(f64)`, `Ident(String)`, arithmetic operators, and `Caret` for exponentiation. `Expr` is an AST enum with `Number`, `Ident`, `Prefix { op, expr }`, and `Infix { op, left, right }`. `infix_binding_power` returns `(1,2)` for `+/-`, `(3,4)` for `*/`, and `(6,5)` for `^` (right-associative: left > right). The main `parse_expr(min_bp)` function calls itself recursively, consuming operators only when their binding power exceeds `min_bp`.

## OCaml Approach

OCaml's `Menhir` handles precedence via `%left`, `%right`, `%nonassoc` declarations — equivalent to Pratt's binding power pairs. Hand-written Pratt parsers in OCaml look similar to Rust: a recursive `parse_expr min_bp` function. The `angstrom` library's `chainl1` and `chainr1` combinators implement left- and right-associative infix parsing, abstracting over Pratt's core idea.

## Key Differences

1. **Binding power encoding**: Rust's `(left_bp, right_bp)` tuple is the standard Pratt encoding; OCaml's `Menhir` uses declaration-based precedence at the grammar level.
2. **Nud/Led terminology**: Pratt's original paper used "nud" (null denotation) and "led" (left denotation); modern presentations use "prefix_bp" and "infix_bp" as in this example.
3. **Flexibility**: Pratt parsers trivially add new operators with custom precedence; Menhir requires grammar modifications and recompilation.
4. **Ternary and postfix**: Pratt handles ternary (`?:`) and postfix (function calls `f(x)`) naturally by treating them as infix/postfix operators with appropriate binding powers.

## Exercises

1. Add the ternary `?:` operator (lowest precedence, right-associative) using Pratt's technique: treat `?` as an infix operator that reads the `:` and right expression inside its `led`.
2. Extend the parser to support function calls `f(a, b)` by treating `(` after an identifier as a postfix operator with high precedence.
3. Add a `Postfix { op, expr }` AST node for the unary postfix `!` (factorial) operator with the highest binding power of all.
