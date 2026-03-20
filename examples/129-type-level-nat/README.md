📖 **[View on hightechmind.io →](https://hightechmind.io/rust/129-type-level-nat)**

---

# Type-Level Natural Numbers — Peano Arithmetic

## Problem Statement

Type-level naturals extend the type system to encode numbers as types: `Zero`, `Succ<Zero>` (= 1), `Succ<Succ<Zero>>` (= 2). This enables length-indexed containers where the compiler rejects out-of-bounds access, type-safe matrix operations where dimension mismatches are compile errors, and statically checked protocol sequences. The technique originates from dependent type theory (Agda, Coq, Idris) and is approximated in Rust and OCaml using type-level programming.

## Learning Outcomes

- Understand Peano arithmetic and how it encodes numbers as nested types
- Learn how `PhantomData<N>` carries type-level information with zero runtime cost
- See how type-level addition and comparison enable compile-time arithmetic
- Understand the limitation compared to true dependent types (Rust/OCaml vs. Idris/Agda)

## Rust Application

`Zero` and `Succ<N>` are marker structs with `PhantomData<N>` to carry the type information. The `Nat` trait exposes `const VALUE: usize` for reflection to runtime values. `Add<B>` is a trait with `type Sum: Nat` — type-level addition via associated types. A `TypedStack<T, N: Nat>` uses the length `N` in its type, so `pop` on an empty stack is a compile error because `pop` is only defined when `N: Succ<_>` (i.e., non-zero).

## OCaml Approach

OCaml with GADTs can encode similar length-indexed types:
```ocaml
type zero = Zero
type 'n succ = Succ
type ('a, 'n) vec =
  | Nil  : ('a, zero) vec
  | Cons : 'a * ('a, 'n) vec -> ('a, 'n succ) vec
```
This is more ergonomic than the Rust encoding because GADTs provide direct type refinement in pattern matches, while Rust must use trait bounds and associated types.

## Key Differences

1. **GADTs vs. traits**: OCaml's GADT encoding is direct and ergonomic; Rust's requires a chain of `Add` trait impls with associated types.
2. **Const generics alternative**: Rust's `const N: usize` (example 126) is a simpler alternative for most practical uses; type-level Peano naturals are needed only when arithmetic over type parameters is required.
3. **Compile-time errors**: Both produce compile errors for invalid operations, but OCaml's GADT error messages are often more informative than Rust's trait-bound errors.
4. **Runtime reflection**: Rust's `Nat::VALUE` reflects to `usize`; OCaml's GADT length is similarly reflected through a `length` function returning an integer.

## Exercises

1. Implement `type Three = Succ<Two>` and verify `Three::VALUE == 3` in a test.
2. Add a `Sub` trait for type-level subtraction, defining `Succ<N> - Succ<M> = N - M` and `Succ<N> - Zero = Succ<N>`.
3. Implement a `SafeStack<T, N: Nat>` where `push` returns `SafeStack<T, Succ<N>>` and `pop` returns `(T, SafeStack<T, N>)`, preventing pop on empty stack.
