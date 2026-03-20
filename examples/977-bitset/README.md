**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

[bitset on hightechmind.io](https://hightechmind.io/posts/functional-rust/bitset)

---

## Problem Statement

Implement a bitset — a fixed-size set of bits stored compactly in `Vec<u64>` words. Support set, clear, toggle, test, union (OR), intersection (AND), difference (AND NOT), and population count (popcount). Compare with OCaml's 63-bit integer bitset (one GC tag bit is reserved) and with Rust's full 64-bit words.

## Learning Outcomes

- Map bit index `i` to word index `i/64` and bit offset `i%64`
- Implement `set`, `clear`, `toggle`, `test` using `|=`, `&= !`, `^=`, `& != 0`
- Implement set operations: `union` (`|`), `intersection` (`&`), `difference` (`& !`)
- Implement `count_ones` (population count) using `u64::count_ones()` summed over words
- Understand why OCaml uses 63-bit integers (one bit reserved for the GC tag) while Rust uses full 64-bit words

## Rust Application

```rust
pub struct Bitset {
    bits: Vec<u64>,
    size: usize,
}

impl Bitset {
    pub fn new(size: usize) -> Self {
        let words = (size + 63) / 64;
        Bitset { bits: vec![0u64; words], size }
    }

    fn word(&self, i: usize) -> usize { i / 64 }
    fn bit(&self, i: usize) -> u64 { 1u64 << (i % 64) }

    pub fn set(&mut self, i: usize) {
        assert!(i < self.size);
        self.bits[self.word(i)] |= self.bit(i);
    }

    pub fn clear(&mut self, i: usize) {
        assert!(i < self.size);
        self.bits[self.word(i)] &= !self.bit(i);
    }

    pub fn toggle(&mut self, i: usize) {
        assert!(i < self.size);
        self.bits[self.word(i)] ^= self.bit(i);
    }

    pub fn test(&self, i: usize) -> bool {
        assert!(i < self.size);
        self.bits[self.word(i)] & self.bit(i) != 0
    }

    pub fn union(&self, other: &Bitset) -> Bitset {
        assert_eq!(self.size, other.size);
        Bitset {
            bits: self.bits.iter().zip(&other.bits).map(|(a, b)| a | b).collect(),
            size: self.size,
        }
    }

    pub fn count_ones(&self) -> usize {
        self.bits.iter().map(|w| w.count_ones() as usize).sum()
    }
}
```

The word/bit helpers centralize the index arithmetic. `(size + 63) / 64` rounds up to the nearest word count — equivalent to `size.div_ceil(64)`. All per-bit operations are O(1).

`u64::count_ones()` uses the hardware `POPCNT` instruction on x86-64 — extremely fast. Summing popcount over all words gives the total set bit count in O(n/64) time.

Set operations work word-by-word: `union` zips the two word vectors and applies `|`. This processes 64 bits per iteration.

## OCaml Approach

```ocaml
(* OCaml int is 63 bits on 64-bit systems (1 bit for GC tag) *)
type bitset = {
  words: int array;
  size: int;
}

let create size =
  { words = Array.make ((size + 62) / 63) 0; size }

let set bs i =
  let w = i / 63 and b = i mod 63 in
  bs.words.(w) <- bs.words.(w) lor (1 lsl b)

let test bs i =
  let w = i / 63 and b = i mod 63 in
  bs.words.(w) land (1 lsl b) <> 0

(* Using Bytes for 8-bit words — avoids GC tag issue *)
let create_bytes size = Bytes.make ((size + 7) / 8) '\000'
```

OCaml's `int` is 63 bits on 64-bit platforms (one bit is the GC's tag bit). Using `int` for bitsets means 63 bits per word, not 64. `Bytes` (byte arrays) avoids this issue at the cost of 8 bits per word instead of 63.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Word size | 64 bits (`u64`) — full hardware word | 63 bits (`int`) — one GC tag bit | 
| `count_ones` | `u64::count_ones()` — hardware POPCNT | No stdlib popcount; loop manually |
| NOT operator | `!` | `lnot` |
| Bitwise AND | `&` | `land` |
| Performance | Direct 64-bit ops | 63-bit bounds, slightly lower density |

Bitsets are useful for dense boolean arrays, Sieve of Eratosthenes, graph adjacency matrices (for small graphs), and flag sets. The `bit-vec` and `fixedbitset` crates provide production-grade implementations.

## Exercises

1. Implement `Bitset::from_indices(size, indices: &[usize])` that sets all specified bits.
2. Implement `symmetric_difference(a, b)` using XOR (`^`).
3. Implement `is_subset(a, b) -> bool` — returns true if every set bit in `a` is also set in `b`.
4. Implement `first_set() -> Option<usize>` — find the lowest set bit using `trailing_zeros()`.
5. Implement the Sieve of Eratosthenes using a `Bitset` for an upper bound of 1,000,000.
