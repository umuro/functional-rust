# 708: Safe Transmute Patterns and Alternatives

**Difficulty:** 4  **Level:** Expert

Know which safe API to reach for first — `transmute` is the weapon of last resort.

## The Problem This Solves

`std::mem::transmute<T, U>` is an unchecked bitwise reinterpretation: it tells the compiler "treat this `T` as a `U`, no questions asked." The compiler only checks that `size_of::<T>() == size_of::<U>()`. It does not check alignment, validity, layout compatibility, or whether the resulting value makes sense for type `U`. Used incorrectly, it's instant undefined behaviour — a float NaN pattern reinterpreted as an integer, a `&mut T` turned into a `&mut U` that aliases, a pointer to freed memory silently extended.

The problem is that many developers reach for `transmute` as the first tool because it *looks* like it does the obvious thing. But the Rust standard library exposes safe alternatives for most common use cases: `f64::to_bits()` for float-to-integer bit casting, `slice::from_raw_parts` for viewing a struct as bytes, `try_from`/`try_into` for checked numeric casts. These alternatives are faster to write, clearer to read, and eliminate whole classes of bugs.

The discipline is: exhaust the safe alternatives first. When none exist and `transmute` is genuinely needed, write a `// SAFETY:` comment proving layout compatibility — same size, same alignment, valid bit patterns for the target type — before you touch `unsafe`.

## The Intuition

Think of memory as a raw sequence of bytes. `transmute` says "these bytes, which my program currently thinks are a `T`, are actually a `U`." That's fine when the byte patterns valid for `T` are also valid for `U` and the sizes match. It's catastrophic when they're not — like declaring the bytes of a `*mut Foo` to be a `u64` and then using that integer to compute an offset.

The safe alternatives are implementations of the same concept with the dangerous edge cases removed: `to_bits()` knows exactly which float patterns are valid integers; `slice::from_raw_parts` knows the element size and requires you to pass the length; `try_from` performs the boundary check you'd have written anyway.

## How It Works in Rust

```rust
// ✅ Float ↔ integer bits — use the standard library methods
let bits: u64 = 3.14f64.to_bits();
let back: f64 = f64::from_bits(bits);

// ✅ View a repr(C) struct as bytes — slice::from_raw_parts
#[repr(C)]
struct Vec2 { x: f32, y: f32 }

fn vec2_as_bytes(v: &Vec2) -> &[u8] {
    unsafe {
        // SAFETY: Vec2 is repr(C) so layout is defined.
        // size_of::<Vec2>() bytes are valid u8 reads.
        std::slice::from_raw_parts(
            v as *const Vec2 as *const u8,
            std::mem::size_of::<Vec2>(),
        )
    }
}

// ✅ Checked numeric cast — no unsafe needed
let n: i32 = -1;
let safe: Option<u32> = u32::try_from(n).ok();  // None, not UB

// ❌ transmute — only when no safe alternative exists
// unsafe { std::mem::transmute::<f64, u64>(3.14) }  // to_bits() is better
```

For zero-cost, safe numeric casting across multiple types, the `bytemuck` crate provides `bytemuck::cast::<T, U>(val)` with compile-time layout proofs via trait bounds.

## What This Unlocks

- **Safer numeric code**: `to_bits()`/`from_bits()` for float inspection (NaN boxing, serialisation) without any unsafe.
- **FFI serialisation**: View a `repr(C)` struct as `&[u8]` for wire encoding without copying — safe with `slice::from_raw_parts`.
- **Defensive casting**: `try_from`/`try_into` propagate overflow as `Err` instead of silently wrapping or truncating.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Float ↔ bits | `Int64.bits_of_float` | `f64::to_bits()` (safe, no unsafe) |
| Byte view of struct | `Bytes` + blit | `slice::from_raw_parts` + `repr(C)` |
| Checked cast | Explicit boundary checks | `try_from` / `try_into` |
| Unchecked reinterpret | Rare (`Obj.magic`) | `std::mem::transmute` (unsafe) |
| Safe multi-type cast | Not in stdlib | `bytemuck::cast` crate |
| Layout requirement | GC-managed, not user-controlled | Must be `repr(C)` for defined layout |
