# 129: Type-Level Natural Numbers

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Encode the length of a collection as part of its type so operations like `pop` on an empty list, or calling a function that requires exactly 3 elements with a 2-element list, are compile errors.

## The Problem This Solves

A classic runtime bug: you call `.pop()` on a stack, but the stack is empty. Or you pass a 2-element list to a function that documents "requires at least 3 elements." Both are runtime panics — and the type `Vec<T>` gives you no help, because `Vec<T>` with 2 elements and `Vec<T>` with 3 elements are the same type.

What if the length were part of the type? Then `Vec_<T, Two>` and `Vec_<T, Three>` would be different types. A function that takes `Vec_<T, Three>` simply cannot be called with a 2-element vector — the types don't match. And `pop()` could only exist on `Vec_<T, Succ<N>>` (a vector with at least one element), never on `Vec_<T, Zero>`. You can't pop an empty list because the method isn't defined for that type.

This is the core idea behind length-indexed vectors (also called "vectors" in dependently typed languages). It's a building block for safe APIs that document their preconditions in types rather than doc comments.

## The Intuition

Peano arithmetic defines natural numbers using two rules: zero is a number, and the successor of any number is also a number. In code: `Zero` and `Succ<N>`. So `Succ<Zero>` is one, `Succ<Succ<Zero>>` is two, and so on.

These are just empty structs — they hold no data. Their purpose is to be *counted by the type system*. When you have a `Vec_<T, Succ<Succ<Zero>>>`, the compiler knows at compile time that this vector contains exactly 2 elements. When you call `.push()`, it returns `Vec_<T, Succ<Succ<Succ<Zero>>>>` — a 3-element vector.

The magic is that each `push` changes the type. The old value is consumed and a new one with an incremented type is returned. The type *is* a proof of length.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Peano encoding: numbers as nested types, not values
struct Zero;
struct Succ<N>(PhantomData<N>);  // PhantomData: we don't store N, just use it as a type param

trait Nat {
    const VALUE: usize;  // optional: convert to a runtime number
}
impl Nat for Zero { const VALUE: usize = 0; }
impl<N: Nat> Nat for Succ<N> { const VALUE: usize = N::VALUE + 1; }

// Convenient aliases
type One   = Succ<Zero>;
type Two   = Succ<One>;
type Three = Succ<Two>;

// Length-indexed vector — length N is in the type, not stored at runtime
struct Vec_<T, N: Nat> {
    data: Vec<T>,
    _len: PhantomData<N>,
}

// Create: only Zero-length vectors can be created empty
impl<T> Vec_<T, Zero> {
    fn new() -> Self { Vec_ { data: vec![], _len: PhantomData } }
}

// push: consumes Vec_<T, N> and returns Vec_<T, Succ<N>>
// The length in the type increments with every push
impl<T, N: Nat> Vec_<T, N> {
    fn push(mut self, val: T) -> Vec_<T, Succ<N>> {
        self.data.push(val);
        Vec_ { data: self.data, _len: PhantomData }
    }
    fn len(&self) -> usize { N::VALUE }
}

// pop: only exists for Succ<N> — can't pop an empty Vec_<T, Zero>!
impl<T, N: Nat> Vec_<T, Succ<N>> {
    fn pop(mut self) -> (T, Vec_<T, N>) {  // returns element AND shorter vector
        let val = self.data.pop().unwrap();
        (val, Vec_ { data: self.data, _len: PhantomData })
    }
    fn head(&self) -> &T { &self.data[0] }
}

// This function requires EXACTLY 3 elements — enforced by the type
fn needs_three<T>(v: &Vec_<T, Three>) -> usize { v.len() }
```

Usage:
```rust
let v = Vec_::<i32, Zero>::new()
    .push(10)   // returns Vec_<i32, One>
    .push(20)   // returns Vec_<i32, Two>
    .push(30);  // returns Vec_<i32, Three>

needs_three(&v);  // compiles ✓

let two_elem = Vec_::<i32, Zero>::new().push(1).push(2);
// needs_three(&two_elem);  // compile error: expected Vec_<i32, Three>, found Vec_<i32, Two>
```

## What This Unlocks

- **Safe stack APIs** — a `Stack<T, N>` where `pop()` and `peek()` are absent on `Stack<T, Zero>` eliminates an entire class of underflow panics.
- **Typed matrices** — combine with const generics or type-level nats to build matrices where row/column counts are tracked in types.
- **Protocol sequencing** — model protocols where certain operations require a minimum number of prior steps to have been performed.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Peano encoding | GADT: `type _ nat = Zero : zero nat \| Succ : 'n nat -> 'n succ nat` | Structs: `struct Zero; struct Succ<N>(PhantomData<N>)` |
| Length-indexed vec | Recursive GADT type `('a, 'n) vec` with native pattern matching | Struct with `PhantomData<N>`; different `impl` blocks per length constraint |
| Safe head/tail | GADT ensures `hd` only works on `succ` vectors | `head()` and `pop()` only in `impl Vec_<T, Succ<N>>` — absent for `Zero` |
| Runtime value | `let n = match v with Succ (Succ Zero) -> 2 ...` | `N::VALUE` — reads the const from the trait impl |
