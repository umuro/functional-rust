📖 **[View on hightechmind.io →](https://hightechmind.io/rust/490-string-fixed-array)**

---

# String Fixed Array

`FixedString<const N: usize>` is a stack-allocated string buffer backed by `[u8; N]` with a length counter, providing `String`-like operations with zero heap allocation — essential for embedded systems, real-time code, and hot paths.

## Problem Statement

Every `String` allocation touches the heap: the allocator must find free memory, update bookkeeping, and the deallocator must run on drop. In embedded systems (no heap), real-time audio (allocation is forbidden in the audio thread), kernel code, and performance-critical parsers, heap allocation is either impossible or too expensive. A **stack-allocated string** of fixed maximum size avoids this: the bytes live in the stack frame, `Copy` semantics are possible, and there is no drop glue. This is the approach of `arrayvec`, `heapless::String`, and C's `char buf[N]`.

## Learning Outcomes

- Use const generics (`const N: usize`) to parameterise a struct by capacity at compile time
- Store string bytes in `[u8; N]` with a separate `len: usize` field
- Implement `from_str`, `push_str`, `push`, `as_str`, and `clear`
- Derive `Copy` for a fixed-size container — impossible for `String`
- Understand the `const fn new()` pattern for compile-time initialisation

## Rust Application

`FixedString<N>` is `Copy` because all fields are `Copy`:

```rust
#[derive(Clone, Copy)]
pub struct FixedString<const N: usize> {
    buffer: [u8; N],
    len: usize,
}
```

`from_str` returns `None` if the input exceeds capacity:

```rust
pub fn from_str(s: &str) -> Option<Self> {
    if s.len() > N { return None; }
    let mut fs = Self::new();
    fs.buffer[..s.len()].copy_from_slice(s.as_bytes());
    fs.len = s.len();
    Some(fs)
}
```

`push` encodes a `char` to up to 4 bytes and delegates to `push_str`, correctly handling multibyte characters without heap allocation.

## OCaml Approach

OCaml does not have stack-allocated arrays of statically known size in the same sense. `Bytes.create n` allocates on the heap. For embedded/no-alloc contexts, OCaml is typically not used; C or Rust are preferred. In standard OCaml, the `Bigarray` module can use `Bigarray.Array1.create Bigarray.int8_unsigned Bigarray.c_layout n` for C-layout buffers, but these are still heap-allocated.

```ocaml
(* Closest OCaml equivalent — heap allocated *)
let fixed_string_of n s =
  if String.length s > n then None
  else Some (Bytes.of_string s)
```

OCaml's module system can parameterise by capacity using a functor, but there is no const generic equivalent.

## Key Differences

1. **Stack allocation**: Rust's `FixedString<N>` lives entirely on the stack; OCaml's equivalent is always heap-allocated.
2. **Const generics**: Rust's `const N: usize` is a compile-time parameter — different `N` values produce distinct types; OCaml would use a runtime `n` parameter, losing the type-level capacity constraint.
3. **`Copy` derivation**: `FixedString<N>` is `Copy` because `[u8; N]` and `usize` are `Copy`; `String` cannot be `Copy` because it owns heap memory.
4. **`const fn new`**: Rust's `const fn` enables compile-time construction (`static MY_STR: FixedString<16> = FixedString::new()`); OCaml has no equivalent.

## Exercises

1. **`Display` impl**: Implement `fmt::Display` for `FixedString<N>` so it can be used in `format!` and `println!`.
2. **Const initialisation**: Declare `static EMPTY: FixedString<64> = FixedString::new()` and verify it compiles — exploring the limits of `const fn`.
3. **Comparison with `arrayvec`**: Replace `FixedString<N>` with `arrayvec::ArrayString<N>` (which has `Copy`, `Display`, and `Deref<Target=str>`) and compare the API surface.
