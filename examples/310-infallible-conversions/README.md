📖 **[View on hightechmind.io →](https://hightechmind.io/rust/310-infallible-conversions)**

---

# 310: Infallible Conversions

## Problem Statement

Some conversions are always valid (`u8` to `u32`), while others might fail (`u32` to `u8` — might overflow). Rust encodes this distinction in the type system: `From/Into` for infallible conversions, `TryFrom/TryInto` for fallible ones. Using `TryFrom` for a conversion that might fail makes failure handling explicit and visible, unlike C-style implicit narrowing casts that silently truncate. This mirrors OCaml's explicit type coercions.

## Learning Outcomes

- Understand `TryFrom<T>` as the fallible counterpart to `From<T>`
- Implement `TryFrom<u32>` for a newtype wrapper with a validity constraint
- Use `try_into()` for conversions that can fail with a specific error type
- Recognize the relationship: infallible `From` implies `Into`; `TryFrom` implies `TryInto`

## Rust Application

`TryFrom<T>` defines fallible construction from a raw value:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonZeroU32(u32);

#[derive(Debug, PartialEq)]
pub struct ZeroError;

impl TryFrom<u32> for NonZeroU32 {
    type Error = ZeroError;
    fn try_from(n: u32) -> Result<Self, ZeroError> {
        if n == 0 { Err(ZeroError) }
        else { Ok(NonZeroU32(n)) }
    }
}

// Usage: 5u32.try_into() or NonZeroU32::try_from(5)
let valid: Result<NonZeroU32, ZeroError> = 5u32.try_into();
let invalid: Result<NonZeroU32, ZeroError> = 0u32.try_into();
```

## OCaml Approach

OCaml uses explicit conversion functions — there is no standard `TryFrom` trait equivalent:

```ocaml
type non_zero = NonZero of int

exception ZeroError

let non_zero_of_int n =
  if n = 0 then Error `ZeroError
  else Ok (NonZero n)
```

OCaml's lack of numeric implicit conversion makes this less critical — explicit function calls are always required.

## Key Differences

1. **Trait unification**: `TryFrom`/`TryInto` provide a standard interface for all fallible conversions; OCaml uses ad-hoc `*_of_*` naming conventions.
2. **Symmetric pair**: `impl TryFrom<A> for B` automatically provides `impl TryInto<B> for A` — symmetry is built in.
3. **std numeric impls**: The standard library implements `TryFrom<u64>` for `u8`, `u16`, etc. — the idiomatic way to do checked narrowing.
4. **Orphan rule**: You can implement `TryFrom<ThirdPartyType>` for your type but not for third-party types — same as `From`.

## Exercises

1. Implement a `BoundedI32(i32)` newtype that wraps integers in range `[MIN, MAX]`, implementing `TryFrom<i32>` with a `RangeError` for out-of-range values.
2. Use the standard library's `TryFrom<i64>` for `i32` to safely narrow a value, handling the error case explicitly.
3. Implement `TryFrom<&str>` for a `Color` enum with `Red`, `Green`, `Blue` variants, parsing case-insensitively.
