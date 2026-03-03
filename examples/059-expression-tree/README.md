# Example 059: Recursive Variant — Expression Tree

**Difficulty:** ⭐⭐
**Category:** Algebraic Data Types
**OCaml Source:** Cornell CS3110 — Data chapter (variants with payloads)

## Problem Statement

Define a recursive algebraic data type for arithmetic expressions (Num, Add, Sub, Mul, Div). Implement `eval` to compute the result and `to_string` to pretty-print with parentheses.

## Learning Outcomes

- Model recursive data types in both languages
- Understand why Rust needs `Box` for recursive enums (known size requirement)
- Write structural recursion over tree-shaped data
- Add safe error handling for division by zero (Rust improvement)
- Use convenience constructors to reduce `Box::new` boilerplate

## OCaml Approach

OCaml's recursive variants are natural — `type expr = Num of float | Add of expr * expr | ...` — with no explicit heap allocation needed. Pattern matching destructures directly.

## Rust Approach

1. **Idiomatic:** `enum Expr` with `Box<Expr>` for recursive fields, methods via `impl`
2. **Free functions:** Standalone `eval()` and `to_string()` mirroring OCaml
3. **Safe division:** `eval_safe()` returns `Result<f64, String>` for divide-by-zero

## Key Differences

1. **Box requirement:** Rust enums must have known size → recursive fields need `Box<T>` (heap indirection); OCaml allocates implicitly
2. **Constructors:** Rust benefits from helper functions (`Expr::new_add(l, r)`) to hide Box boilerplate; OCaml constructors work directly
3. **Display trait:** Rust's `Display` impl replaces OCaml's `to_string` function — enables `format!("{expr}")`
4. **Error handling:** Rust can return `Result` for safe division; OCaml's version silently produces `infinity`
5. **Ownership in recursion:** Rust's `eval(&self)` borrows the tree; OCaml pattern matching doesn't distinguish owned vs borrowed
