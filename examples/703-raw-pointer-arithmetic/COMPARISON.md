# OCaml vs Rust: Raw Pointer Arithmetic

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml hides pointer arithmetic entirely; arrays are always bounds-checked. *)
let strided_read (arr : 'a array) ~(start : int) ~(stride : int) : 'a list =
  let n = Array.length arr in
  let rec go i acc =
    if i >= n then List.rev acc
    else go (i + stride) (arr.(i) :: acc)
  in
  go start []
```

### Rust (idiomatic — safe slice indexing)
```rust
pub fn strided_collect_safe(slice: &[i32], stride: usize) -> Vec<i32> {
    if stride == 0 { return vec![]; }
    (0..slice.len()).step_by(stride).map(|i| slice[i]).collect()
}
```

### Rust (unsafe — raw pointer arithmetic)
```rust
pub fn strided_collect(slice: &[i32], stride: usize) -> Vec<i32> {
    if slice.is_empty() || stride == 0 { return vec![]; }
    let mut result = Vec::new();
    let base: *const i32 = slice.as_ptr();
    let len = slice.len();
    let mut offset = 0usize;
    while offset < len {
        // SAFETY: offset < len; base valid for len elements; alignment guaranteed.
        result.push(unsafe { *base.add(offset) });
        offset = offset.saturating_add(stride);
    }
    result
}
```

### Rust (in-place reversal via converging pointers)
```rust
pub fn reverse_in_place(slice: &mut [i32]) {
    let len = slice.len();
    if len < 2 { return; }
    // SAFETY: lo starts at 0, hi at len-1; loop stops before they cross; no alias.
    unsafe {
        let base: *mut i32 = slice.as_mut_ptr();
        let mut lo = base;
        let mut hi = base.add(len - 1);
        while lo < hi {
            core::ptr::swap(lo, hi);
            lo = lo.add(1);
            hi = hi.sub(1);
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Array element access | `arr.(i)` (safe, bounds-checked) | `*ptr.add(i)` (unsafe, manual proof) |
| Stride loop | tail-recursive `go (i + stride)` | `while offset < len` + `ptr.add` |
| In-place mutation | `Array.blit` or index assignment | `*mut T` + `core::ptr::swap` |
| Safety boundary | Runtime exception on OOB | `unsafe` block + `// SAFETY:` comment |
| Signed offset | integer subtraction on index | `ptr.offset(isize)` |

## Key Insights

1. **No pointer exposure in OCaml:** OCaml arrays are always accessed through safe, bounds-checked index operators. Pointer arithmetic is an implementation detail of the GC runtime, never surfaced to user code.

2. **`ptr.add(n)` advances by elements, not bytes:** Unlike C's `char *` arithmetic, Rust raw pointer arithmetic scales by `size_of::<T>()` automatically, matching OCaml's array indexing semantics while operating at the address level.

3. **`unsafe` as a proof obligation:** Rust doesn't forbid pointer arithmetic — it requires you to localise it in an `unsafe` block and document the invariant with `// SAFETY:`. OCaml enforces safety by construction; Rust enforces it by contract.

4. **Converging-pointer reversal is idiomatic C/Rust, not OCaml:** The `lo`/`hi` swap pattern has no natural OCaml equivalent. OCaml prefers `Array.blit` or functional reversal; Rust can express the in-place algorithm directly without intermediate allocation.

5. **`ptr.offset` vs `ptr.add`/`ptr.sub`:** `ptr.offset(isize)` is the signed, general form (positive = forward, negative = backward). `ptr.add` and `ptr.sub` are unsigned convenience wrappers that make intent clearer for unidirectional traversal.

## When to Use Each Style

**Use safe slice indexing when:** bounds checking overhead is negligible and code clarity matters — which is almost always.

**Use raw pointer arithmetic when:** you've already verified the range at the call site and want to avoid redundant per-element checks in a tight inner loop, or when expressing a two-pointer algorithm (convergent swap, custom stride walk) that doesn't map cleanly to Rust's iterator combinators.
