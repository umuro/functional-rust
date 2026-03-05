# Example 1078: Visitor Pattern via Fold — Expression Evaluator

**Difficulty:** ⭐⭐
**Category:** Monadic Patterns / Higher-Order Functions
**OCaml Source:** https://cs3110.github.io/textbook/chapters/interp/substitution.html

## Problem Statement

Implement fold as a functional replacement for the visitor pattern. Define an expression tree and use fold to create both an evaluator and a pretty-printer without modifying the tree structure.

## Learning Outcomes

- Fold as the universal eliminator for algebraic data types
- How closures replace the Visitor trait pattern in functional style
- Box-based recursive enums in Rust vs OCaml's built-in recursive types
- Multiple interpretations of the same data structure via different fold parameters

## OCaml Approach

OCaml defines `fold` with labeled arguments (`~lit`, `~add`, `~mul`, `~neg`) — one function per variant. Creating new operations (eval, to_string) is just calling fold with different closures. No trait needed, no boilerplate.

## Rust Approach

Rust uses `&dyn Fn` trait objects as parameters to fold, mirroring OCaml's approach. Helper constructors (`lit()`, `add()`) reduce the `Box::new` noise. A trait-based `ExprVisitor` is also shown for comparison with the OOP approach.

## Key Differences

1. **Labeled arguments:** OCaml uses `~lit ~add ~mul ~neg`; Rust uses positional `&dyn Fn` parameters
2. **Heap allocation:** OCaml's recursive types are heap-allocated transparently; Rust needs explicit `Box<Expr>`
3. **Visitor pattern:** OCaml doesn't need it — fold is more natural. Rust can use either fold or a Visitor trait
4. **Pattern matching:** Both languages pattern match on the enum/variant; Rust requires `&` for references
