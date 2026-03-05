# OCaml vs Rust: Const Generic Struct

## Ring Buffer with Const Capacity

### Rust
```rust
pub struct RingBuffer<T, const CAP: usize> {
    data: [Option<T>; CAP],
    head: usize,
    tail: usize,
}

let rb: RingBuffer<i32, 16> = RingBuffer::new();
```

### OCaml
```ocaml
(* No compile-time capacity - runtime parameter *)
type 'a ring_buffer = {
  data: 'a option array;
  mutable head: int;
  mutable tail: int;
}

let create cap = {
  data = Array.make cap None;
  head = 0;
  tail = 0;
}
```

## BitSet with Const Size

### Rust
```rust
pub struct BitSet<const BITS: usize>
where
    [(); (BITS + 63) / 64]: Sized,
{
    data: [u64; (BITS + 63) / 64],
}

let bs: BitSet<100> = BitSet::new(); // Exactly 2 u64s
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Capacity param | Runtime | Compile-time |
| Memory layout | Heap array | Stack-allocated |
| Size known | Runtime | Compile-time |
| Type safety | Same type any size | Different types |
