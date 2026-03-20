[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 083 — Display Trait
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Implement `std::fmt::Display` for custom types — `Color`, `Point`, `Person`, and a generic `Tree<T>` — to enable `format!`, `println!`, and `to_string()` without deriving `Debug`. Compare with OCaml's `Printf.sprintf`-based `to_string` functions for the same types.

## Learning Outcomes

- Implement `fmt::Display` using `write!(f, "...", ...)` in the `fmt` method
- Understand `fmt::Formatter` as the sink that `write!` targets
- Use format specifiers like `{:.1}` for floating-point precision inside `Display`
- Implement `Display` for generic types with `T: fmt::Display` bound
- Distinguish `Display` (user-facing) from `Debug` (developer-facing)
- Map Rust's trait-based formatting to OCaml's explicit `to_string` functions

## Rust Application

Every `impl fmt::Display` supplies `fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`. For `Color`, a match arm calls `write!(f, "Red")` etc. For `Point`, `write!(f, "({:.1}, {:.1})", self.x, self.y)` formats with one decimal place. For the recursive `Tree<T: Display>`, the `Display` bound on `T` lets `write!(f, "({} {} {})", l, v, r)` recursively format the subtrees — each sub-format triggers the `fmt` method of the child. Implementing `Display` automatically provides `to_string()` via the blanket `impl ToString for T where T: Display`.

## OCaml Approach

OCaml does not have a single `Display` trait; each type gets its own `to_string` function. `Printf.sprintf "(%.1f, %.1f)" p.x p.y` formats a `point`. For recursive `tree`, a higher-order `tree_to_string to_s` takes the element formatter as an argument, since OCaml has no trait-bound system. The result is the same string — the mechanism is different: explicit function passing vs trait dispatch.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Interface | `impl fmt::Display` trait | `to_string` function per type |
| Generic elements | `T: fmt::Display` bound | `to_s : 'a -> string` parameter |
| Debug format | `#[derive(Debug)]` | `Printf.sprintf "%S"` / ppx |
| `to_string()` | Auto from `Display` | Explicit function |
| Sink type | `fmt::Formatter` | Returns `string` directly |
| Format spec | `{:.1}`, `{:>10}`, etc. | `%.1f`, `%10s`, etc. |

Rust's `Display` trait integrates with the entire `format!`/`println!`/`write!` machinery. Any type that implements `Display` can be used in any format string `{}` position. OCaml's approach is more direct but requires explicitly threading the `to_string` function through generic code.

## Exercises

1. Implement `fmt::Display` for a `Matrix(Vec<Vec<f64>>)` newtype that prints rows separated by newlines.
2. Add an `impl fmt::Display for Tree<T>` variant that prints in indented format (each level adds two spaces).
3. Implement `fmt::Debug` manually for `Person` to show `Person { name: "Alice", age: 30, email: "…" }`.
4. Write a `Wrapper<T: Display>(Vec<T>)` that displays its items as `[a, b, c]`.
5. In OCaml, define a `Printable` module type with `val to_string : t -> string` and a functor `PrintList(P: Printable)` with `val print_list : P.t list -> string`.
