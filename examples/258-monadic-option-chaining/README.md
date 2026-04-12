📖 **[View on hightechmind.io →](https://hightechmind.io/rust/258-monadic-option-chaining)**

---

# Example 258: Monadic Option Chaining
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Chain multiple partial functions (functions returning `Option`) so that a `None` at any step short-circuits the entire computation, without nesting `match` expressions.

## Learning Outcomes

- How OCaml's `>>=` (bind) maps to Rust's `Option::and_then`
- How OCaml's `>>|` (functor map) maps to Rust's `Option::map`
- How Rust's `?` operator provides ergonomic monadic chaining with explicit control flow
- Why `and_then` composes better than nested `match` for sequential fallible operations

## OCaml Approach

OCaml defines custom infix operators `>>=` and `>>|` to chain `Option` values in a pipeline style. This is the option monad: `>>=` sequences computations that may fail, `>>|` transforms successful values. The result reads left-to-right and each `None` silently terminates the chain.

## Rust Approach

Rust's `Option<T>` has `and_then` (monadic bind) and `map` (functor map) built into the standard library, so no operator definitions are needed. The `?` operator offers a third style: it desugars to early return on `None`, making control flow explicit while keeping code concise. All three styles produce identical semantics.

## Key Differences

1. **Operator syntax:** OCaml defines `>>=` as an infix operator; Rust uses method syntax `and_then` and `map` (no custom operators in stable Rust).
2. **`?` operator:** Rust's `?` is a unique construct with no OCaml equivalent — it makes monadic short-circuiting look like imperative early return.
3. **`Option::bind` in stdlib:** OCaml 4.08+ added `Option.bind`; before that, developers always defined `>>=` manually. Rust has had `and_then` from day one.
4. **Ownership:** Rust's `and_then` and `map` consume the `Option` by value; closures receive owned `T`, not a reference, matching OCaml's value semantics.

## Exercises

1. Rewrite the same computation chain using the `?` operator instead of explicit `and_then` calls and verify the results are identical.
2. Implement `option_all` that takes a `Vec<Option<T>>` and returns `Some(Vec<T>)` only if every element is `Some`, using a fold over the sequence.
3. Build a small JSON-like path traversal: given a nested `HashMap<String, Value>` structure, write a function `get_path(&self, path: &[&str]) -> Option<&Value>` using monadic chaining.
