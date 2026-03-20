📖 **[View on hightechmind.io →](https://hightechmind.io/rust/431-macro-count-pattern)**

---

# 431: Counting Patterns in Macros
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Many macro-generated code patterns need to know the number of elements at compile time: pre-allocating arrays of the right size, generating tuple types, creating assertions about argument counts. Counting macro arguments is surprisingly non-trivial — you can't use a simple `$n` count since macros don't have builtin counters. Three techniques exist: recursive counting (O(n) expansions), array length trick (O(1) using `[()].len()`), and the substitution trick. Each has different compile-time performance characteristics.

Counting patterns appear in `static_assertions` (checking type sizes), array initialization macros, compile-time tuple generation, and any macro needing to allocate the right amount of space.

## Learning Outcomes

- Understand three approaches to counting macro arguments: recursion, array trick, expression counting
- Learn why the array trick (`[()].len()`) is O(1) compile time vs. O(n) for recursive counting
- See how `@single $_:tt => ()` converts each token to a unit value for array length counting
- Understand when compile-time counts matter for performance (deeply recursive macros can hit `recursion_limit`)
- Learn how to use counts in array initialization: `[default_val; count_array!($($x),*)]`

## Rust Application

In `src/lib.rs`, `count_recursive!` is the straightforward approach: base case returns 0, recursive case returns `1 + count!(tail)`. `count_array!` uses the `@single` helper to map each token to `()`, then `<[()]>::len(&[...])` to count at compile time without recursion. `count_exprs!` counts only expression fragments. The array trick is preferred for large counts to avoid recursion depth limits.

## OCaml Approach

OCaml counts list elements at runtime with `List.length`. At compile time, OCaml uses type-level numbers (Peano arithmetic with GADTs or type-level naturals from the `zarith` library). PPX can count AST nodes during compilation. There's no direct equivalent of Rust's token-counting trick since OCaml PPX operates on the AST, where `List.length` on fields is straightforward.

## Key Differences

1. **Compile vs. runtime**: Rust's count macros produce compile-time constants; OCaml's `List.length` is a runtime function.
2. **Technique**: Rust needs tricks (array trick, recursion) because macros don't have built-in counters; OCaml PPX can directly use `List.length fields`.
3. **Type-level**: Rust uses `const` values from macro counting; OCaml can encode lengths in types using GADTs for type-level length checking.
4. **Performance**: The array trick is O(1) expansions; the recursive approach is O(n) expansions where n is the element count.

## Exercises

1. **Fixed-size tuple macro**: Implement `tuple!(1, 2, 3)` that generates a tuple literal AND uses `count_array!` to initialize an array `[0usize; count_array!(1, 2, 3)]` of the same length.
2. **Sized collection**: Create `static_vec!(T, 1, 2, 3)` that generates `let arr: [T; COUNT] = [1, 2, 3]` where COUNT is the compile-time count. Verify that the array size matches the element count.
3. **Argument count assertion**: Write `assert_arity!(fn_name, 3)` that at compile time asserts a tuple has exactly 3 elements, generating a `static_assertions::const_assert_eq!(COUNT, 3)` check.
