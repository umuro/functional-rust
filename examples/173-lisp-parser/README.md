📖 **[View on hightechmind.io →](https://hightechmind.io/rust/173-lisp-parser)**

---

# Lisp / S-expression Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

S-expressions (S-exprs) are the syntax of Lisp, Scheme, and Clojure, and are used as a data format by Emacs configuration, WebAssembly's text format, and many build systems. Their uniform recursive structure (atoms and lists) makes them both easy to parse and pleasant to work with programmatically. Parsing S-exprs demonstrates recursive parser combinators with multiple primitive types: atoms, numbers, strings, booleans, nil, and nested lists.

## Learning Outcomes

- Parse a recursive, self-similar structure (S-expressions) using recursive parser combinators
- Handle multiple literal types: numbers, strings (with escape sequences), booleans, atoms
- See how quote notation `'x` is syntactic sugar for `(quote x)` at the parser level
- Build a complete, usable Lisp reader in under 100 lines of combinator code

## Rust Application

The `Sexp` enum has six variants: `Atom(String)`, `Number(f64)`, `Str(String)`, `Bool(bool)`, `Nil`, `List(Vec<Sexp>)`. The parser uses `choice` to try each type in order: number first (so `3` is not parsed as atom `"3"`), then `#t`/`#f` booleans, then `nil`, then quoted strings, then atoms, then lists. Lists are `delimited('(', many0(sexp), ')')` — recursive via `Rc<dyn Fn>`. The quote shorthand `'x` maps to `(quote x)` in the parser.

## OCaml Approach

OCaml's Lisp parsers use `let rec` naturally:
```ocaml
let rec sexp () =
  ws *> choice [number; boolean; nil; string_; atom; list ()]
and list () = char '(' *> many (sexp ()) <* char ')'
```
The `()` arguments break the value recursion (OCaml does not allow `let rec` over values, only over functions). This is a well-known OCaml pattern for recursive parsers.

## Key Differences

1. **Recursive binding**: OCaml uses `let rec ... and ...`; Rust uses `Rc<dyn Fn>` — both accomplish mutual recursion for parsers.
2. **Atom definition**: What counts as an atom varies by Lisp dialect; both parsers accept any non-whitespace, non-parenthesis sequence.
3. **Quote shorthand**: `'x` → `(quote x)` is a syntactic transformation done at parse time — identical in both languages.
4. **String escaping**: Both handle `\"` and `\\` escapes; full Lisp string escape sequences (`\n`, `\t`, `\uXXXX`) require additional work.

## Exercises

1. Add `quasiquote` (`` ` ``), `unquote` (`,`), and `unquote-splicing` (`,@`) as syntactic sugar handled in the parser.
2. Implement a simple evaluator for the parsed S-expressions that handles `(+ 1 2)` and `(if true 1 0)`.
3. Add line/column tracking to the parser so errors report the position in the source code.
