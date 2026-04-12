📖 **[View on hightechmind.io →](https://hightechmind.io/rust/896-clone-copy)**

---

# 896-clone-copy — Clone and Copy Traits
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Rust's ownership model means that values are moved by default. For types where copying is cheap and always valid — integers, booleans, simple structs — implicit copying is desirable. For types with heap allocation — `String`, `Vec`, complex structs — copying is potentially expensive and must be explicit. Rust encodes this distinction in two traits: `Copy` (implicit bitwise copy, zero-cost) and `Clone` (explicit `.clone()`, potentially expensive). Seeing `.clone()` in code signals a potentially costly heap duplication. Not seeing it means the copy is stack-only. This visual distinction makes performance characteristics readable from the code.

## Learning Outcomes

- Understand the semantic difference between `Copy` (implicit) and `Clone` (explicit)
- Derive `Copy` and `Clone` for simple structs and recognize when Copy is appropriate
- Understand why `String`, `Vec`, and other heap-owning types cannot be `Copy`
- Recognize `.clone()` as a visible signal of heap allocation in code reviews
- Compare with OCaml's uniform representation where all values are implicitly shareable

## Rust Application

`copy_integer(x: i32)` shows that `let y = x` silently copies — `x` remains valid. `copy_tuple((f64, f64))` similarly copies the tuple. `Point` derives both `Copy` and `Clone` — struct members are `f64`, which is `Copy`. `translate(self, dx, dy) -> Self` takes `self` by value (moves for non-Copy or copies for Copy) and returns a new `Point` — original preserved via Copy. `clone_string(s: &String)` demonstrates `s.clone()` for explicit heap duplication. `clone_vec` shows `v.clone()` producing an independent copy of the entire vector.

## OCaml Approach

OCaml has no `Copy`/`Clone` distinction. All values (including strings and lists) are implicitly sharable — passing a value to a function passes a pointer, not a copy. Immutability makes sharing safe: OCaml strings used to be mutable (a historical mistake); `Bytes.t` is now the mutable string type. For actual copying: `String.copy` (deprecated), `Bytes.copy`, `Array.copy`. OCaml's structural sharing means no distinction between "move" and "share" — the GC handles it.

## Key Differences

1. **Implicit vs explicit**: Rust `Copy` types copy silently; `Clone` types require `.clone()`. OCaml passes a shared pointer always — no per-value choice.
2. **Performance visibility**: In Rust, `.clone()` signals potential allocation; in OCaml, copying is never visible in the source — profiling is required to detect it.
3. **Composability**: A struct is `Copy` only if all its fields are `Copy`; OCaml values are always shareable regardless of field types.
4. **Heap types**: `String`, `Vec`, `Box` cannot be `Copy` (they own heap memory); in OCaml, all heap-allocated values are shareable via GC.

## Exercises

1. Create a `Matrix2x2` struct with four `f64` fields, derive `Copy + Clone`, and implement matrix multiplication returning a new matrix.
2. Write a function `cloned_pipeline(data: &[String]) -> Vec<String>` that clones elements conditionally — only those passing a filter — and explain why `.clone()` is needed.
3. Implement a `Config` struct with `String` fields and `#[derive(Clone)]`, then write `apply_override(base: &Config, override: Config) -> Config` using struct update syntax.
