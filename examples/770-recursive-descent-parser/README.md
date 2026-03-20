📖 **[View on hightechmind.io →](https://hightechmind.io/rust/770-recursive-descent-parser)**

---

# 770-recursive-descent-parser — Recursive Descent Parser

## Problem Statement

Recursive descent parsing is the simplest technique for parsing context-free grammars. It was invented in the 1960s and remains the dominant approach for hand-written parsers in production compilers: Clang, GCC, rustc, and Go's parser are all recursive descent. The idea is elegant: each grammar rule becomes a function; parsing is calling that function. Operator precedence is handled by a hierarchy of mutually recursive functions (or the Pratt technique in the next example).

## Learning Outcomes

- Implement a lexer that tokenizes an arithmetic expression into `Token` variants
- Build a recursive descent parser for `expr → term ('+' | '-' term)*` grammar
- Handle operator precedence via grammar rule hierarchy: expr → term → factor → atom
- Evaluate expressions directly during parsing (no separate AST phase)
- Understand why left-recursive grammars cannot be directly parsed by recursive descent

## Rust Application

`Lexer<'a>` scans `input: &'a str` character by character, producing `Token` variants: `Number(f64)`, `Plus`, `Minus`, `Star`, `Slash`, `LParen`, `RParen`, `Eof`. `Parser` holds a `Lexer` and the current token. `parse_expr` → `parse_term` → `parse_factor` → `parse_primary` implements the precedence hierarchy. Each level handles one precedence class. `parse_primary` handles numbers and parenthesized expressions.

## OCaml Approach

OCaml is an excellent language for recursive descent parsers because functions are first-class and tail-call optimization prevents stack overflow on deep recursion. `Menhir` is OCaml's standard LALR parser generator, used in the OCaml compiler itself. Hand-written recursive descent parsers in OCaml look nearly identical to Rust: `let rec parse_expr () = ... and parse_term () = ...`. The `angstrom` combinator library provides an alternative that avoids hand-rolling lexers.

## Key Differences

1. **Mutual recursion**: Both languages handle mutual recursion naturally; Rust requires `fn parse_expr(&mut self)` calling `self.parse_term()`, while OCaml uses `let rec ... and ...`.
2. **Error recovery**: Rust parsers typically panic on errors in simple implementations; production parsers return `Result` and synchronize on recovery tokens.
3. **Parser generators**: Rust has `lalrpop`, `pest`, and `nom`; OCaml has `Menhir` (LALR), `sedlex` (lexer), and `angstrom` (combinator).
4. **Grammar expression**: OCaml's `Menhir` uses BNF-like rules; Rust's `lalrpop` uses a similar notation with action code in Rust.

## Exercises

1. Add a `^` power operator (right-associative, highest precedence) by adding a `parse_power` level between `parse_factor` and `parse_primary`.
2. Extend the parser to build an AST (`Expr` enum) instead of evaluating directly, then write a separate `eval(expr: &Expr) -> f64` function.
3. Add error recovery: instead of panicking on unexpected tokens, emit an error message and try to continue parsing from the next `+` or `)` token.
