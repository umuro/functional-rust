📖 **[View on hightechmind.io →](https://hightechmind.io/rust/178-gadt-length-list)**

---

# Length-Indexed Lists
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Calling `head` on an empty list is a classic source of runtime errors. Length-indexed lists — vectors where the length is part of the type — eliminate this class of error: `head` is only defined on `Vec2<T, N>` where `N > 0`. Rust achieves this with const generics (`[T; N]`), which are more ergonomic than the type-level Peano natural approach (example 129) for most practical purposes. This example demonstrates the const-generic approach to length safety.

## Learning Outcomes

- Use `const N: usize` to encode list length in the type, making `head` on empty impossible
- Understand `Vec2<T, N>` as a fixed-size wrapper with length-safe operations
- See how `append` type-safely combines two vectors: `Vec2<T, N>` + `Vec2<T, M>` → `Vec2<T, {N+M}>`
- Appreciate the practical limitation: `N+M` in const generics requires `where [(); N+M]:` bounds

## Rust Application

`Vec2<T, const N: usize>` wraps `[T; N]`. `head` returns `T` directly (not `Option<T>`) because `N > 0` is required by the `assert!` at the function boundary — in practice, this should be a compile-time constraint, not a runtime assert. `replicate(val: T) -> Vec2<T, N>` creates a vector of N copies. The `append` function: `Vec2<T, N>.append(Vec2<T, M>) -> Vec2<T, {N+M}>` requires the nightly feature `#[feature(generic_const_exprs)]` or a runtime workaround in stable Rust.

## OCaml Approach

OCaml uses GADTs for length-indexed vectors:
```ocaml
type _ vec =
  | Nil  : zero vec
  | Cons : 'a * 'n vec -> ('n succ) vec
let head : type n. (n succ) vec -> 'a = function Cons (x, _) -> x
```
`head` is only callable on non-empty vectors by construction — the `succ` in the type guarantees at least one element. There is no `assert!` and no runtime check; the guarantee is purely structural.

## Key Differences

1. **Compile-time guarantee**: OCaml's GADT `head` on `n succ vec` is guaranteed safe by the type; Rust's const-generic `head` uses a runtime `assert!` (a weaker guarantee).
2. **Arithmetic**: OCaml type-level addition requires the `Add` trait (example 129); Rust's `{N+M}` in const generics requires nightly `generic_const_exprs`.
3. **Ergonomics**: Rust's `[T; N]` is a built-in array with const generics; OCaml's length-indexed vector requires GADT definition.
4. **Practical use**: Most production Rust code uses `Vec<T>` with runtime checks; const-generic arrays are used where performance and fixed sizes matter (embedded, SIMD).

## Exercises

1. Implement `zip<T, U, const N: usize>(a: Vec2<T, N>, b: Vec2<U, N>) -> Vec2<(T,U), N>` that combines element-wise.
2. Add `tail<T, const N: usize>(v: Vec2<T, N>) -> Vec2<T, {N-1}>` (requires nightly or manual unsafe extraction).
3. Write a `map<T, U, const N: usize>(v: Vec2<T, N>, f: impl Fn(T) -> U) -> Vec2<U, N>`.
