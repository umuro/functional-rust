📖 **[View on hightechmind.io →](https://hightechmind.io/rust/781-const-where-bounds)**

---

# 781: Where Bounds on const Generic Parameters

**Difficulty:** 4  **Level:** Advanced

`where [(); N - 1]: Sized` and `where [(); { assert!(...); 0 }]: Sized` — type-level constraints on const expressions.

## The Problem This Solves

Const generics let you put integers in types, but sometimes you need to *constrain* those integers at the type level: `N` must be non-zero, `N` must be a power of two, alignment must meet a minimum. Without these constraints, you'd get a panic at runtime (or worse, silent wrong behavior). With `where` bounds on const expressions, the constraint becomes a compile error.

The pattern looks strange at first: `where [(); N - 1]: Sized` — an array type of size `N-1`. If `N` is 0, `N - 1` underflows to a huge number (or panics in const context), making the bound unsatisfiable. This is a current workaround for the lack of direct const inequality constraints in stable Rust. The const block variant — `where [(); { assert!(N > 0); 0 }]: Sized` — is more explicit and gives better error messages.

This pattern appears in the `tinyvec` crate (non-zero capacity assertion), cryptographic libraries (key length constraints), and any generic data structure where certain const parameter values are semantically invalid.

## The Intuition

`[(); N]: Sized` is always true — arrays of `()` of any size are `Sized`. The trick is making the array size an expression that evaluates to an invalid value for forbidden inputs. `[(); N - 1]: Sized` fails for `N = 0` because `0 - 1` underflows in const context. The `assert!` variant is cleaner: `[(); { assert!(N.is_power_of_two()); 0 }]: Sized` — the assert panics at compile time for invalid `N`, and the array size is always `0` for valid `N`.

## How It Works in Rust

```rust
// Non-zero constraint: N-1 overflows for N=0 → compile error
pub struct NonEmpty<T, const N: usize>
where
    [(); N - 1]: Sized,   // forbids N = 0 at the type level
{
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> NonEmpty<T, N>
where
    [(); N - 1]: Sized,
{
    pub fn new() -> Self { Self { data: [T::default(); N] } }
    pub fn first(&self) -> &T { &self.data[0] }  // safe: N >= 1 guaranteed by type
    pub fn last(&self)  -> &T { &self.data[N - 1] }
}

// Power-of-two constraint: assert in const block
pub struct PowerOfTwoRing<T: Default + Copy, const N: usize>
where
    [(); { assert!(N.is_power_of_two(), "N must be a power of two"); 0 }]: Sized,
{
    buf: [T; N],
    head: usize,
    count: usize,
}

impl<T: Default + Copy, const N: usize> PowerOfTwoRing<T, N>
where
    [(); { assert!(N.is_power_of_two(), "N must be a power of two"); 0 }]: Sized,
{
    #[inline]
    fn idx(&self, i: usize) -> usize { i & (N - 1) }  // fast: N is power of two
    // ...
}

// Alignment constraint
pub struct AlignAtLeast<T, const ALIGN: usize>
where
    [(); { assert!(align_of::<T>() >= ALIGN, "alignment too small"); 0 }]: Sized,
{
    value: T,
}

// Usage
let ne: NonEmpty<i32, 3> = NonEmpty::new();   // ok
// let bad: NonEmpty<i32, 0> = NonEmpty::new(); // COMPILE ERROR — [(); 0-1] fails

let mut ring: PowerOfTwoRing<i32, 8> = PowerOfTwoRing::new();  // ok
// let bad: PowerOfTwoRing<i32, 6> = PowerOfTwoRing::new();    // COMPILE ERROR — 6 not pow2
```

Note: the `where` clause must be repeated on every `impl` block — this is a current limitation of Rust's const generic implementation. The `N - 1` trick works only when `T` is `usize` (subtraction panics in const on underflow for debug builds). Nightly Rust has `feature(generic_const_exprs)` which cleans this up significantly.

## What This Unlocks

- **Semantic constraints in types** — `NonEmpty<T, 3>` is not just `[T; 3]`; it carries the proof that it's non-empty; methods like `first()` are safe without `Option` return types.
- **Power-of-two fast modulo** — `i & (N - 1)` instead of `i % N` is only correct when `N` is a power of two; the where bound makes the optimization safe by preventing misuse.
- **Alignment guarantees at the type level** — `AlignAtLeast<T, 16>` cannot be instantiated with a type whose natural alignment is less than 16; useful for SIMD buffers and DMA regions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type-level natural constraints | GADTs + phantom types | `where [(); expr]: Sized` — const expression trick |
| Non-zero type | Existential with private constructor | `NonEmpty<T, N> where [(); N - 1]: Sized` |
| Power-of-two enforcement | Runtime `assert` | Compile-time via `assert!` in const block in where clause |
| Nightly improvements | N/A | `feature(generic_const_exprs)` — direct `where N > 0` |
