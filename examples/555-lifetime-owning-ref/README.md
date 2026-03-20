📖 **[View on hightechmind.io →](https://hightechmind.io/rust/555-lifetime-owning-ref)**

---

# Owning References Pattern

## Problem Statement

Sometimes you want a type that owns its data and simultaneously exposes a view into it — a buffer that knows which window of its bytes is "active," a string that knows where its meaningful content starts and ends. Storing both the owner and a reference to its contents in the same struct leads to self-referential problems. The owning-reference pattern solves this by storing indices rather than pointers, computing views on demand, or using separate types for owner and view.

## Learning Outcomes

- How `OwnedSlice` stores indices instead of references to avoid self-referential issues
- How `fn slice(&self) -> &[u8]` computes the view from stored indices at access time
- How `fn narrow(&mut self, start, end)` adjusts the active window without copying data
- How this pattern enables zero-copy processing of large buffers
- Where owning references appear: network buffers, text editors (rope data structures), binary parsers

## Rust Application

`OwnedSlice` stores `data: Vec<u8>` with `start: usize` and `end: usize`. `slice(&self) -> &[u8]` returns `&self.data[self.start..self.end]` — a zero-copy view into the owned buffer. `narrow` adjusts the window bounds. This enables processing pipelines where a buffer is progressively consumed: each stage narrows the window to indicate how much it has processed, without copying or reallocating.

Key patterns:
- `data: Vec<u8>, start: usize, end: usize` — owned data with index window
- `fn slice(&self) -> &[u8] { &self.data[self.start..self.end] }` — view computed from indices
- `fn narrow(&mut self, s, e)` — adjust window without reallocation

## OCaml Approach

OCaml's `Bytes` or `Bigarray` slices are reference-counted or GC-managed, so owning references are natural:

```ocaml
type owned_slice = { data: bytes; mutable start: int; mutable end_: int }
let slice s = Bytes.sub s.data s.start (s.end_ - s.start)  (* copies *)
(* Zero-copy requires Bigarray.Array1 sub-views *)
```

True zero-copy sub-views in OCaml require `Bigarray.Array1` with explicit layout management.

## Key Differences

1. **Zero-copy views**: Rust `&self.data[s..e]` is a true zero-copy view into the `Vec`; OCaml `Bytes.sub` copies — zero-copy needs `Bigarray`.
2. **Index invalidation**: Rust's borrow checker ensures `slice()` result cannot outlive `self`; OCaml's GC keeps `Bigarray` slices alive but does not prevent use-after-mutation.
3. **Window mutation**: Rust `narrow(&mut self)` requires `&mut self` — the compiler prevents concurrent reads of the slice while narrowing; OCaml allows concurrent access through `ref`.
4. **Rope data structure**: Text editors in Rust (like `ropey`) use owning-reference patterns extensively; OCaml text editors use GC-managed trees of `string` chunks.

## Exercises

1. **Packet parser**: Implement a `struct Packet { data: Vec<u8>, pos: usize }` with a `fn read_u16(&mut self) -> Option<u16>` that reads two bytes at `pos` and advances it.
2. **Ring buffer view**: Extend `OwnedSlice` to wrap around: `fn wrap_slice(&self) -> (&[u8], &[u8])` returning the two parts when `start > end` (ring buffer state).
3. **Zero-copy chains**: Implement `fn split_at(&self, mid: usize) -> (OwnedSliceView<'_>, OwnedSliceView<'_>)` returning two views into the same `OwnedSlice` without copying.
