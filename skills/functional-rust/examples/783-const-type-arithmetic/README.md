# 783: Type-Level Arithmetic with const Generics

**Difficulty:** 4  **Level:** Advanced

Express array size relationships in the type system: `concat([T; A], [T; B]) -> [T; A+B]` — the output length is computed at compile time.

## The Problem This Solves

When you concatenate two arrays, the output array's length is the sum of the inputs. In most languages this is a runtime property — you get a `Vec` and check `.len()`. With const generics in Rust, the length is a *type-level* property: `[T; A+B]` is a distinct type from `[T; 5]`, and the compiler verifies at every call site that the sizes are consistent.

This matters for embedded systems, SIMD, cryptography, and any domain where buffer sizes are contracts — stack-allocated, no heap, no size-mismatch bugs at runtime. A wrong size is a compile error, not a panic.

The same mechanism enables unit-typed quantities (`Meters<5> + Meters<3> = Meters<8>`) where adding distances of different units won't compile.

## The Intuition

`const N: usize` as a type parameter lets you parameterize over array sizes. Expressions like `{ A + B }` in type position compute the size at compile time. The compiler instantiates a new monomorphized version of the function for each concrete pair of sizes.

The curly braces `{ A + B }` are required in type position to disambiguate const expressions from other syntax. This is still stabilizing across different expression forms — but addition and multiplication work stably.

## How It Works in Rust

**Array concatenation** — output type encodes the sum:
```rust
pub fn concat<T: Copy + Default, const A: usize, const B: usize>(
    a: [T; A],
    b: [T; B],
) -> [T; { A + B }] {
    let mut out = [T::default(); { A + B }];
    out[..A].copy_from_slice(&a);
    out[A..].copy_from_slice(&b);
    out
}

let a = [1, 2, 3];   // [i32; 3]
let b = [4, 5];      // [i32; 2]
let c = concat(a, b); // [i32; 5] — length in the type
```

**Array split** — sizes must sum to N:
```rust
pub fn split<T: Copy + Default, const N: usize, const A: usize>(
    arr: [T; N],
) -> ([T; A], [T; { N - A }])
where [(); N - A]: Sized {
    // ...
}

let (left, right): ([i32; 2], [i32; 3]) = split([1, 2, 3, 4, 5]);
// Wrong split would be a compile error
```
The `where [(); N - A]: Sized` bound is a workaround for the compiler to accept the subtraction expression in type position.

**Interleave** — output is double the input:
```rust
pub fn interleave<T: Copy + Default, const N: usize>(
    a: [T; N], b: [T; N],
) -> [T; { N * 2 }] { ... }
```

**Unit-typed quantities** — wrong dimensions don't compile:
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters<const N: usize>;

pub fn add_meters<const A: usize, const B: usize>(
    _a: Meters<A>, _b: Meters<B>
) -> Meters<{ A + B }> { Meters }

let m5 = Meters::<5>;
let m3 = Meters::<3>;
let m8 = add_meters(m5, m3);  // type: Meters<8>
// add_meters(m5, Feet::<3>)  // won't compile — different type
```

**Repeat** — output size is N*K:
```rust
pub fn repeat_array<T: Copy + Default, const N: usize, const K: usize>(
    arr: [T; N],
) -> [T; { N * K }] { ... }

let r = repeat_array::<i32, 2, 3>([1, 2]);  // [i32; 6]
```

**Zip** — equal-length constraint enforced by type:
```rust
pub fn zip<A: Copy + Default, B: Copy + Default, const N: usize>(
    a: [A; N], b: [B; N],
) -> [(A, B); N] { ... }
// zip([1,2,3], [4,5]) → compile error: N=3 vs N=2
```

## What This Unlocks

- **Stack-allocated, size-verified buffers** — no heap, no bounds checks at runtime, wrong sizes are compile errors.
- **Type-safe dimensional analysis** — `Meters<N>` + `Meters<M>` = `Meters<N+M>`, incompatible units don't compile.
- **Compile-time API contracts** — functions that take two arrays of the same size, or produce an array of a known computed size.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Fixed-size array | `('a, 'a) array` with runtime length | `[T; N]` — N is a compile-time `usize` |
| Type-level size computation | Type-level integers via modules/GADTs | `const N: usize`; `{ A + B }` in type position |
| Unit-typed quantities | Phantom types / GADTs | `struct Meters<const N: usize>` |
| Size constraint at call site | Dependent types (not standard OCaml) | Monomorphization — compiler checks each instantiation |
