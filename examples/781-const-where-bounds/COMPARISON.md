# OCaml vs Rust: Const Where Bounds

## Compile-Time Constraints

### Rust
```rust
pub struct NonEmptyArray<T, const N: usize>
where
    [(); N - 1]: Sized, // Compile error if N == 0
{
    data: [T; N],
}

// Compiles:
let ok: NonEmptyArray<i32, 5> = NonEmptyArray::new();

// Won't compile:
// let bad: NonEmptyArray<i32, 0> = NonEmptyArray::new();
```

### OCaml
No compile-time numeric constraints:
```ocaml
(* Runtime check only *)
let create_non_empty n =
  if n <= 0 then invalid_arg "must be positive";
  Array.make n default
```

## Power of Two Constraint

### Rust
```rust
pub struct PowerOfTwoBuffer<const SIZE: usize>
where
    [(); (SIZE & (SIZE - 1))]: Sized, // Fails if not power of 2
{
    data: [u8; SIZE],
}

// Fast wrap using bit mask
pub const fn wrap_index(&self, idx: usize) -> usize {
    idx & (SIZE - 1)  // Same as idx % SIZE but faster
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Numeric constraints | Runtime only | Compile-time |
| Error timing | Program crash | Compile error |
| Zero-size arrays | Runtime check | Won't compile |
| Divisibility | Runtime assertion | Type-level |
