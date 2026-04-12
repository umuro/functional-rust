# OCaml vs Rust: Array.blit — Copy Subarray

## Side-by-Side Code

### OCaml
```ocaml
let src = [| 10; 20; 30; 40; 50 |]
let dst = Array.make 8 0
let () = Array.blit src 1 dst 2 3
(* dst is now [| 0; 0; 20; 30; 40; 0; 0; 0 |] *)
```

### Rust (idiomatic — in-place)
```rust
pub fn array_blit<T: Copy>(
    src: &[T], src_pos: usize,
    dst: &mut [T], dst_pos: usize,
    len: usize,
) {
    dst[dst_pos..dst_pos + len].copy_from_slice(&src[src_pos..src_pos + len]);
}
```

### Rust (functional — returns new Vec)
```rust
pub fn array_blit_functional<T: Copy>(
    src: &[T], src_pos: usize,
    dst: &[T], dst_pos: usize,
    len: usize,
) -> Vec<T> {
    let mut result = dst.to_vec();
    result[dst_pos..dst_pos + len].copy_from_slice(&src[src_pos..src_pos + len]);
    result
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val blit : 'a array -> int -> 'a array -> int -> int -> unit` | `fn array_blit<T: Copy>(src: &[T], src_pos: usize, dst: &mut [T], dst_pos: usize, len: usize)` |
| Source array | `'a array` | `&[T]` (immutable slice) |
| Destination array | `'a array` (mutable by convention) | `&mut [T]` (enforced mutable slice) |
| Sub-range | `(src_pos, len)` pair of `int` | `[src_pos..src_pos + len]` slice range |
| Element constraint | none (structural equality) | `T: Copy` (bitwise copyable) |

## Key Insights

1. **Slice ranges unify position + length:** OCaml's `(src_pos, len)` pair maps directly to Rust's `[start..start+len]` range syntax — both express the same sub-array window, but Rust's version is first-class and bounds-checked.
2. **Mutability is encoded in the type:** OCaml arrays are always mutable; Rust distinguishes `&[T]` (read-only) from `&mut [T]` (writable). The compiler ensures the borrow checker prevents using the same allocation as both source and destination without `unsafe`.
3. **`T: Copy` replaces structural equality:** OCaml's `Array.blit` works on any type because OCaml arrays are GC-managed references. Rust's `copy_from_slice` requires `T: Copy` to guarantee a bitwise memcpy is safe and correct.
4. **`copy_from_slice` is one function call:** The standard library maps to the same hardware instruction (a block memory copy) that OCaml's `Array.blit` ultimately uses — no element-by-element loop in either language.
5. **Functional alternative is a first-class pattern:** Because Rust's slice API is composable, returning a new `Vec<T>` (allocate + copy + overwrite window) is idiomatic for callers that prefer immutability. OCaml would require `Array.copy` followed by `Array.blit`.

## When to Use Each Style

**Use idiomatic Rust (in-place `&mut [T]`):** when you control the destination buffer and want zero allocation — e.g., filling a pre-allocated frame buffer, copying into a network packet, or updating a region of a large array without cloning.

**Use functional Rust (returns `Vec<T>`):** when the caller should not be responsible for allocating the destination, when you want a pure function that is easier to test, or when the destination may be referenced elsewhere and mutation would be unsafe.
