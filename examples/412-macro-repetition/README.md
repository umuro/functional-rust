📖 **[View on hightechmind.io →](https://hightechmind.io/rust/412-macro-repetition)**

---

# 412: Macro Repetition Patterns
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Variadic functions (accepting any number of arguments) are fundamental to ergonomic APIs: `println!`, `vec!`, `format!`. In Rust, regular functions cannot be variadic — they have fixed arities. `macro_rules!` repetition syntax `$($pattern),*` matches zero or more occurrences and expands them, enabling macros that accept any number of arguments. This is how `vec![1, 2, 3]` works for any length, and how `println!("{} {}", a, b)` accepts different format argument counts.

Repetition patterns appear in every variadic macro: `assert_matches!`, `matches!`, `dbg!`, `join!` in async code, and custom test helpers.

## Learning Outcomes

- Understand `$(...),*` (zero or more) and `$(...),+` (one or more) repetition syntax
- Learn how to expand repetitions into expressions, statements, and item definitions
- See how trailing comma handling with `$(,)?` improves ergonomics
- Understand nested repetitions for multi-level variadic patterns
- Learn how repetition interacts with fragment types

## Rust Application

In `src/lib.rs`, `sum!()` returns 0 for the empty case, and `sum!($first $(, $rest)*)` expands to `$first $(+ $rest)*` — the `+` is inside the repetition, joining all values. `product!` follows the same pattern with `*`. `all_gt!` uses a semicolon separator between the threshold and the value list, with `$(,)?` enabling an optional trailing comma. The `hashmap!` macro uses `$($key:expr => $val:expr),*` to match key-value pairs.

## OCaml Approach

OCaml functions are natively variadic through list arguments or optional parameters. `List.fold_left (+) 0 values` sums any list without macros. For syntax-level repetition (like `match` arm generation), OCaml uses PPX. The `[%test_eq: int] x y` syntax from Jane Street's `ppx_jane` generates comparison code through AST transformation — similar to Rust macro repetition but through a different mechanism.

## Key Differences

1. **Native variadic**: OCaml functions can accept lists/options making many macro use cases unnecessary; Rust requires macros for variadic behavior.
2. **Compile-time expansion**: Rust's repetition macros expand at compile time with no runtime list; OCaml's list-based variadic functions use runtime list traversal.
3. **Syntax flexibility**: Rust macro repetition can match any token pattern; OCaml's variadic approaches are limited to function call syntax.
4. **Type heterogeneity**: Rust macros can accept values of different types in one repetition (via `$tt`); OCaml lists are homogeneous.

## Exercises

1. **Max of n**: Implement `max_of!(a, b, c, ...)` analogously to `min_of!` — returning the maximum of any number of comparable expressions.
2. **Cartesian product**: Implement `cartesian!(($a1, $a2, ...), ($b1, $b2, ...))` that generates a `Vec` of all pairs. Use nested repetition.
3. **Batch assert**: Implement `assert_all_eq!(($a, $b), ($c, $d), ...)` that asserts each pair is equal, with the pair index in the panic message so failures identify which pair failed.
