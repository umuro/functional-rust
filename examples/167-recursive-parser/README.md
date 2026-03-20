📖 **[View on hightechmind.io →](https://hightechmind.io/rust/167-recursive-parser)**

---

# Recursive Parser

## Problem Statement

Recursive grammars — lists containing lists, expressions containing sub-expressions, JSON arrays containing arrays — require parsers that call themselves. In a strict functional setting, this creates a challenge: a closure cannot easily refer to itself. Rust's solution uses `Rc<dyn Fn>` for sharing a parser across recursive calls, or function pointers for simpler cases. Recursion is the fundamental mechanism for parsing context-free grammars.

## Learning Outcomes

- Understand why recursive parsers are necessary for context-free grammars
- Learn to use `Rc<dyn Fn>` to share a parser across recursive closure calls
- See the S-expression (Lisp-style) recursive structure as a concrete example
- Understand the relationship between recursive parsers and recursive descent parsers

## Rust Application

The `Sexp` enum (`Atom(String)` or `List(Vec<Sexp>)`) is inherently recursive. The parser is built with `Rc`: `let sexp_parser: Rc<dyn Fn(&str) -> ParseResult<Sexp>>`. Inside the closure, `sexp_parser.clone()` is captured to allow recursive calls. `atom_parser` handles the base case; `list_parser` is `delimited('(', many0(sexp_parser_clone), ')')`. The `Rc` enables the parser to reference itself without `unsafe` or function pointers.

## OCaml Approach

OCaml's `let rec` enables mutually recursive function definitions naturally:
```ocaml
let rec sexp input = (atom <|> list) input
and list input = (char '(' *> many sexp <* char ')') input
```
No `Rc` or shared reference is needed — OCaml's recursive `let rec` bindings allow circular references directly. This is one area where OCaml's syntax is significantly more natural than Rust's for parser combinator code.

## Key Differences

1. **Recursive binding**: OCaml's `let rec` is the natural mechanism; Rust requires `Rc<dyn Fn>` or explicit function pointers to express recursion in closures.
2. **Verbosity**: OCaml's recursive parser is a few lines; Rust's `Rc`-based approach requires explicit cloning and is more verbose.
3. **Stack usage**: Both use the call stack for recursion — deeply nested input causes stack overflow in both; trampolining (example 197) is needed for very deep recursion.
4. **Performance**: `Rc<dyn Fn>` adds reference counting and virtual dispatch overhead; OCaml's recursive functions are called directly.

## Exercises

1. Extend the S-expression parser to handle integer literals: `(+ 1 (* 2 3))`.
2. Write a mutually recursive parser for `expr` and `term` without `Rc` using function pointers instead.
3. Add depth limiting to the recursive parser to prevent stack overflow on deeply nested input.
