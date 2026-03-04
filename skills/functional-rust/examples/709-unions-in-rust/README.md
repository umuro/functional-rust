# 709: Unions in Rust

**Difficulty:** 4  **Level:** Expert

Use `union` for C-compatible shared-memory layouts, always wrapped in a safe tagged-union abstraction.

## The Problem This Solves

A Rust `union` places all its fields at the same memory address. Writing `union.int_val = 42` and then reading `union.float_val` is undefined behaviour — you've reinterpreted the bits of an integer as a float without any type-checking. The compiler allows it, but only inside `unsafe`. This is deliberate: unions exist specifically for C interop and for building the kind of variant types the compiler itself uses internally.

The raw `union` is dangerous because nothing tracks which field was last written. The safe pattern is the *tagged union*: pair the `union` with an enum discriminant in an outer struct, and expose only methods that read the correct field based on the tag. The enum `Tag` carries no extra memory on modern architectures (it's an `i8` or similar), and the accessor methods return `Option<T>` — `None` if you ask for the wrong variant.

This is exactly what OCaml's algebraic data types are at the hardware level, except OCaml hides the tag and the dispatch from you. In Rust you write it explicitly, which is more work but gives you full control over the memory layout — essential for `repr(C)` FFI structs that must match a C `union` definition byte-for-byte.

## The Intuition

Imagine a car park with only one space but three different cars that could park there: a sedan, a truck, or a motorcycle. The `union` is the parking space — it can hold any of them. The `Tag` enum is the sign outside: "SEDAN CURRENTLY PARKED." You must check the sign before you interact with the car, because if you try to drive a motorcycle that's actually a truck, something will break badly.

The safety rule is simple: only read the field that corresponds to the last-written value. Everything else is undefined behaviour.

## How It Works in Rust

```rust
#[repr(C)]
union RawValue {
    int_val:   i64,
    float_val: f64,
    bool_val:  u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag { Int, Float, Bool }

pub struct Value {
    tag:  Tag,
    data: RawValue,
}

impl Value {
    pub fn int(n: i64) -> Self {
        Value { tag: Tag::Int, data: RawValue { int_val: n } }
    }

    pub fn as_int(&self) -> Option<i64> {
        if self.tag == Tag::Int {
            Some(unsafe {
                // SAFETY: tag == Int means int_val was last written.
                self.data.int_val
            })
        } else {
            None
        }
    }
}
```

For FFI, add `#[repr(C)]` to both the `union` and the outer struct to match the C layout exactly. Use `cbindgen` to generate the C header.

## What This Unlocks

- **C FFI unions**: Match C `union` definitions byte-for-byte with `#[repr(C)]` — the only way to correctly model C APIs that use unions in their structs.
- **Custom tagged values**: Build interpreter value types, JSON nodes, or wire-protocol variants where every variant shares the same stack footprint.
- **Explicit layout control**: Unlike Rust enums (which have an unspecified discriminant size), a `union` + explicit tag gives you full control over the memory layout for serialisation or MMIO register banks.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Tagged union | Built-in variant types | `union` + enum discriminant (manual) |
| Reading a variant | Automatic via pattern match | `unsafe` — must check tag manually |
| Memory layout | GC-managed, unspecified | `#[repr(C)]` for C compatibility |
| Pattern matching | `match` exhaustive | Manual discriminant check required |
| FFI compatible | No (boxed heap values) | Yes with `#[repr(C)]` |
| Variant tracking | Compiler enforces | Programmer responsibility |
