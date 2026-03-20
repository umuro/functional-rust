📖 **[View on hightechmind.io →](https://hightechmind.io/rust/174-arithmetic-parser)**

---

# Arithmetic Expression Evaluator

## Problem Statement

An arithmetic evaluator combines parsing and immediate evaluation: it reads an expression like `(3 + 4) * 2 - 1` and produces `13.0` without constructing an AST. This is the classic recursive descent evaluator, foundational to expression evaluation in spreadsheets, calculator apps, scripting engines, and configuration processors. Understanding it requires grasping how grammar levels correspond to function call levels.

## Learning Outcomes

- Implement a complete recursive descent arithmetic evaluator with correct precedence
- Understand how grammar rules map to mutually recursive functions: `expr`, `term`, `factor`, `number`
- See how parentheses are handled naturally by recursion (`factor` calls `expr`)
- Appreciate why parsing-and-evaluating together is simpler than building an AST first

## Rust Application

The four-level grammar: `expr = term (('+' | '-') term)*`, `term = factor (('*' | '/') factor)*`, `factor = '-'? (number | '(' expr ')')`. Each grammar rule is one Rust function. `parse_expr` and `parse_term` use a loop to handle left-associative chains. `parse_factor` handles unary minus and delegates to `parse_number` or `parse_expr` for parenthesized sub-expressions. No AST is built — values are folded as the expression is parsed.

## OCaml Approach

OCaml's recursive descent evaluator is structurally identical:
```ocaml
let rec expr () = ...
and term () = ...
and factor () = ...
```
`let rec ... and ...` provides mutual recursion naturally. OCaml's reference-based input scanning (`let input_ref = ref s`) or functional threading (`let (v, rest) = expr s`) mirrors the Rust approach. The evaluator is a classic exercise in OCaml courses.

## Key Differences

1. **Input threading**: Rust threads `&str` slices through functions explicitly; OCaml can use a `ref` for the input position (imperative style) or tuples (functional style).
2. **Float arithmetic**: Both use floating-point (`f64`/`float`) by default; integer arithmetic requires more careful parsing of the grammar.
3. **Error handling**: Rust propagates `Result`; OCaml typically raises exceptions for malformed input in simple evaluators.
4. **AST vs. direct evaluation**: Both examples evaluate directly; building an AST (examples 168-169) separates the concerns.

## Exercises

1. Add the modulo operator `%` between `term` and `factor` precedence levels.
2. Support named constants: `pi` → `3.14159...`, `e` → `2.71828...` in the `factor` rule.
3. Add function calls: `sin(x)`, `cos(x)`, `sqrt(x)` as valid `factor` expressions.
