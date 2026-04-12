📖 **[View on hightechmind.io →](https://hightechmind.io/rust/168-expression-parser)**

---

# Expression Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Mathematical expressions like `1 + 2 * 3` must parse according to operator precedence — the result should be `7` (multiply before add), not `9`. Encoding precedence in recursive descent requires separate functions for each precedence level, which is verbose. Pratt parsing (top-down operator precedence) solves this elegantly with a single loop and a binding power table, making it easy to add new operators and adjust precedence without restructuring the grammar.

## Learning Outcomes

- Understand Pratt parsing (top-down operator precedence) as an elegant precedence solution
- Learn how binding power encodes both precedence and associativity in a single number
- See how prefix operators (unary minus) integrate naturally into Pratt parsing
- Appreciate why Pratt parsers are used in production compilers (Rust's `rustc`, V8, Clang)

## Rust Application

The Pratt parser maintains a `min_bp` (minimum binding power). `parse_expr(input, min_bp)` parses a "nud" (null denotation, i.e., prefix expression), then loops: peek at the next operator, look up its binding power, stop if the left binding power is less than `min_bp`, otherwise parse the right operand with the right binding power. Binary operators like `*` have higher binding power than `+`, causing multiplication to bind tighter.

## OCaml Approach

OCaml's Menhir parser generator handles precedence declaratively:
```ocaml
%left PLUS MINUS
%left TIMES DIV
%nonassoc UMINUS
```
Menhir generates an LALR(1) parser that handles precedence automatically. For hand-written parsers, OCaml uses the same recursive descent or Pratt approach as Rust, often expressed more concisely via `let rec` mutual recursion.

## Key Differences

1. **Generator vs. hand-written**: OCaml commonly uses Menhir for expression grammars; Rust typically hand-writes Pratt parsers or uses the `pratt` crate.
2. **Binding power table**: Pratt parsers use a table lookup for operator binding powers; recursive descent encodes this in the function call structure.
3. **Extensibility**: Pratt parsers support adding operators at runtime (for DSLs); recursive descent parsers are fixed at compile time.
4. **Error recovery**: Pratt parsers integrate error recovery naturally by adjusting `min_bp`; recursive descent recovery is more ad hoc.

## Exercises

1. Add the `^` (power) operator with right-associativity (binding power: left=6, right=5) — verify `2^3^2` = `512`.
2. Implement the ternary operator `a ? b : c` using Pratt parsing.
3. Add a prefix `+` (unary plus, identity) operator alongside the existing unary minus.
