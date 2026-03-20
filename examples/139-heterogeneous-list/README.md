📖 **[View on hightechmind.io →](https://hightechmind.io/rust/139-heterogeneous-list)**

---

# HList — Heterogeneous List

## Problem Statement

A `Vec<T>` requires all elements to have the same type. A `(i32, &str, bool)` tuple preserves each element's type but has fixed arity — you cannot write generic code over tuples of different lengths. An HList (heterogeneous list) bridges this gap: it is a recursive type `HCons<H, T>` where each element's type is preserved in the overall type signature `HCons<i32, HCons<&str, HCons<bool, HNil>>>`. This enables type-safe row polymorphism, variadic generics simulation, and database record types.

## Learning Outcomes

- Understand the recursive type structure of HLists: `HCons<H, T>` and `HNil`
- Learn how compile-time length is tracked via the `HLength` trait
- See how type-safe head extraction and tail access work without runtime checks
- Understand HList applications: Diesel ORM's query rows, type-safe printf, variadic functions

## Rust Application

`HCons<H, T>(pub H, pub T)` is the list cell; `HNil` is the terminator. `HLength` computes the length at compile time: `HNil::LEN = 0`, `HCons<H, T>::LEN = 1 + T::LEN`. `head<H, T>(list: &HCons<H, T>) -> &H` can only be called on non-empty lists — `HNil` has no `head` method. The full type `HCons<i32, HCons<&str, HCons<bool, HNil>>>` documents the exact structure of every stored value.

## OCaml Approach

OCaml does not have HLists in its standard library, but they can be encoded with GADTs:
```ocaml
type _ hlist =
  | HNil : unit hlist
  | HCons : 'a * 'b hlist -> ('a * 'b) hlist
```
The OCaml encoding is more constrained than Rust's because the spine type `unit`, `('a * 'b)` grows with each element. Libraries like `ppx_fields_conv` generate record-based alternatives. Haskell's `HList` library inspired both encodings.

## Key Differences

1. **Type encoding**: Rust uses nested `HCons<H, T>` naturally via structs; OCaml's GADT encoding requires a spine type that grows with each element.
2. **Compile-time length**: Rust's `const LEN: usize` via the `HLength` trait computes length at compile time; OCaml's approach requires similar trait-level computations via recursive GADT types.
3. **Ergonomics**: Both encodings are verbose compared to tuples; Rust macros (`hlist![1, "hi", true]`) reduce syntactic overhead; OCaml uses ppx macros similarly.
4. **Real-world use**: Rust's `frunk` crate provides production-quality HLists; OCaml's `ppx_deriving` provides similar row-polymorphism for records.

## Exercises

1. Implement `HLength` for `HNil` and `HCons<H, T>` and verify `HCons<i32, HCons<bool, HNil>>::LEN == 2`.
2. Write a `hmap` function that applies a function `F: HMap` to every element of an HList, returning a new HList of mapped values.
3. Build a type-safe function `first<H, T>(list: HCons<H, T>) -> H` that extracts the head and demonstrate it works with `i32`, `String`, and `bool` heads.
