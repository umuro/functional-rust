**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐⭐  

[bloom-filter on hightechmind.io](https://hightechmind.io/posts/functional-rust/bloom-filter)

---

## Problem Statement

Implement a Bloom filter — a space-efficient probabilistic data structure that tests set membership with no false negatives but controllable false positives. Use three independent hash functions and a bit array of `u64` words. Implement `insert` (set k bits) and `contains` (check k bits). Demonstrate that `contains` never returns false for inserted items but may return true for non-inserted items.

## Learning Outcomes

- Implement three independent hash functions (djb2, sdbm, fnv-like) using wrapping arithmetic
- Map hash values to bit positions: `hash % num_bits` → word index + bit offset
- Implement `set_bit(pos)` and `test_bit(pos)` using bitwise operations on `u64` words
- Implement `insert` (set all k bit positions) and `contains` (test all k positions)
- Understand the false-positive probability and why `contains` can be wrong but `insert` makes `contains` always correct for inserted items

## Rust Application

```rust
fn hash1(s: &str) -> usize {
    s.bytes().fold(5381usize, |h, b| h.wrapping_mul(31).wrapping_add(b as usize))
}

fn hash2(s: &str) -> usize {
    s.bytes().fold(0usize, |h, b|
        (b as usize).wrapping_add(h.wrapping_shl(6))
                    .wrapping_add(h.wrapping_shl(16))
                    .wrapping_sub(h))
}

fn hash3(s: &str) -> usize {
    s.bytes().fold(0usize, |h, b| h.wrapping_mul(33) ^ (b as usize))
}

pub struct BloomFilter {
    bits: Vec<u64>,
    num_bits: usize,
}

impl BloomFilter {
    pub fn new(num_bits: usize) -> Self {
        let words = num_bits.div_ceil(64);
        BloomFilter { bits: vec![0u64; words], num_bits }
    }

    fn set_bit(&mut self, pos: usize) {
        let pos = pos % self.num_bits;
        self.bits[pos / 64] |= 1u64 << (pos % 64);
    }

    fn test_bit(&self, pos: usize) -> bool {
        let pos = pos % self.num_bits;
        self.bits[pos / 64] & (1u64 << (pos % 64)) != 0
    }

    pub fn insert(&mut self, item: &str) {
        self.set_bit(hash1(item));
        self.set_bit(hash2(item));
        self.set_bit(hash3(item));
    }

    pub fn contains(&self, item: &str) -> bool {
        self.test_bit(hash1(item))
            && self.test_bit(hash2(item))
            && self.test_bit(hash3(item))
    }
}
```

`wrapping_mul/add/sub` are used to avoid overflow panics in debug mode — hash functions intentionally overflow to mix bits. `div_ceil(64)` computes the number of `u64` words needed without a separate `+63)/64` expression.

The false-positive rate for k=3 hash functions and m bits with n insertions is approximately `(1 - e^(-kn/m))^k`. With 1024 bits and 100 insertions: ~2% false positive rate.

## OCaml Approach

```ocaml
let hash1 s =
  String.fold_left (fun h b ->
    ((h * 31) + Char.code b) land max_int
  ) 5381 s

type bloom_filter = {
  mutable bits: bytes;
  num_bits: int;
}

let create num_bits =
  { bits = Bytes.make ((num_bits + 7) / 8) '\000'; num_bits }

let set_bit bf pos =
  let pos = pos mod bf.num_bits in
  let byte_idx = pos / 8 in
  let bit_off  = pos mod 8 in
  let current  = Char.code (Bytes.get bf.bits byte_idx) in
  Bytes.set bf.bits byte_idx (Char.chr (current lor (1 lsl bit_off)))

let test_bit bf pos =
  let pos = pos mod bf.num_bits in
  let byte_idx = pos / 8 in
  let bit_off  = pos mod 8 in
  Char.code (Bytes.get bf.bits byte_idx) land (1 lsl bit_off) <> 0
```

OCaml's `bytes` type provides mutable byte arrays; Rust uses `Vec<u64>` with 64-bit words for 8× fewer memory operations per bit access. OCaml lacks `wrapping_mul` — `land max_int` truncates to 62-bit positive integers.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Bit array | `Vec<u64>` — 64 bits per word | `bytes` — 8 bits per byte |
| Overflow | `wrapping_mul/add` | `land max_int` or unchecked |
| Bit operations | `\|=`, `&`, `1u64 <<` | `lor`, `land`, `lsl` |
| `div_ceil` | Built-in method | `(n + 63) / 64` manually |

Bloom filters trade memory for exact membership. They cannot remove elements (clearing a bit might affect other entries). `BloomFilter::contains` returning `true` means "possibly inserted"; returning `false` means "definitely not inserted".

## Exercises

1. Compute the false-positive rate for your filter: insert 100 items, test 10,000 random strings, count false positives.
2. Implement a `union` of two same-size Bloom filters by ORing their bit arrays.
3. Add a constructor `BloomFilter::with_error_rate(n_items, error_rate)` that computes optimal bit count and hash count.
4. Implement a Counting Bloom Filter using `Vec<u8>` counters instead of bits, supporting delete operations.
5. Benchmark Bloom filter lookup vs `HashSet::contains` for 10,000 items — measure memory usage of each.
