# Example 011: Modified Run-Length Encoding

**Difficulty:** ⭐⭐
**Category:** Lists, Pattern Matching, Algebraic Data Types
**Concept:** Modify standard RLE so that singletons are represented differently from runs. This requires a sum type (OCaml variant / Rust enum) to distinguish the two cases — a natural use of algebraic data types.
**OCaml → Rust key insight:** OCaml's `type 'a rle = One of 'a | Many of int * 'a` maps directly to Rust's `enum RleItem<T> { One(T), Many(usize, T) }`.
