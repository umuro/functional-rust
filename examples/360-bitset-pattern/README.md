📖 **[View on hightechmind.io →](https://hightechmind.io/rust/360-bitset-pattern)**

---

# 360: BitSet Pattern
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

When you need a set of small integers (0..63), a single `u64` can represent the entire set — one bit per element. Bit manipulation turns set operations into single CPU instructions: union is `|`, intersection is `&`, difference is `& !other`, complement is `!`. This is orders of magnitude faster than `HashSet<u32>` for small universes. BitSets power compiler register allocation (tracking which registers are live), sudoku solvers (tracking which digits remain possible in each cell), chess engines (bitboards for piece positions), and graph algorithms (adjacency matrices for small graphs). The `u64.count_ones()` intrinsic maps to a single `POPCNT` CPU instruction.

## Learning Outcomes

- Implement a `BitSet64` wrapping `u64` with O(1) all operations
- Use `1u64 << i` to create a single-bit mask for position `i`
- Implement union (`|`), intersection (`&`), difference (`& !other`)
- Use `count_ones()` for population count (number of set bits) in one CPU instruction
- Iterate set bits using repeated bit extraction
- Understand why `BitSet` beats `HashSet<u32>` for small integer universes

## Rust Application

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitSet64(u64);

impl BitSet64 {
    pub fn empty() -> Self { Self(0) }

    pub fn insert(&mut self, i: u32) {
        assert!(i < 64);
        self.0 |= 1u64 << i;
    }
    pub fn remove(&mut self, i: u32) {
        self.0 &= !(1u64 << i); // clear bit i
    }
    pub fn contains(&self, i: u32) -> bool {
        i < 64 && (self.0 >> i) & 1 == 1
    }

    // Set operations — all O(1) single CPU instructions
    pub fn union(&self, other: &Self) -> Self { Self(self.0 | other.0) }
    pub fn intersection(&self, other: &Self) -> Self { Self(self.0 & other.0) }
    pub fn difference(&self, other: &Self) -> Self { Self(self.0 & !other.0) }

    pub fn count(&self) -> u32 { self.0.count_ones() } // POPCNT instruction
    pub fn is_empty(&self) -> bool { self.0 == 0 }

    pub fn to_vec(&self) -> Vec<u32> {
        (0..64).filter(|&i| self.contains(i)).collect()
    }
}
```

`!(1u64 << i)` creates a mask with all bits set except bit `i` — ANDing with it clears bit `i`. `count_ones()` compiles to a single hardware instruction on x86_64 (`POPCNT`), ARM (`CNT`), and RISC-V — no loop needed.

## OCaml Approach

OCaml integers are 63-bit tagged (one bit used for GC tag), so a plain `int` gives 62 usable bits:

```ocaml
type bitset = int

let empty = 0
let insert bs i = bs lor (1 lsl i)
let remove bs i = bs land (lnot (1 lsl i))
let contains bs i = (bs lsr i) land 1 = 1
let union a b = a lor b
let inter a b  = a land b
let diff a b   = a land (lnot b)

(* popcount: no intrinsic, but bit manipulation works *)
let count bs =
  let n = ref 0 in
  let x = ref bs in
  while !x <> 0 do incr n; x := !x land (!x - 1) done;
  !n
```

OCaml lacks a direct `POPCNT` intrinsic in the standard library (Zarith or C stubs needed). The `x & (x-1)` trick clears the lowest set bit, counting in O(count) rather than O(64). For 64-bit sets, the `Bytes` or `Bigarray` modules handle wider bitsets.

## Key Differences

| Aspect | Rust `BitSet64(u64)` | OCaml `int` bitset |
|--------|---------------------|-------------------|
| Bit width | 64 (all bits available) | 62 (63-bit ints, 1 tag bit) |
| POPCNT | `count_ones()` → hardware intrinsic | No standard intrinsic |
| Type safety | Newtype prevents misuse | Raw `int` — no type distinction |
| Wider sets | `u128` or `[u64; N]` array | `Bytes` or `Bigarray` |
| Bit scan | Manual loop or `trailing_zeros()` | Manual loop |

## Exercises

1. **Bit iteration**: Implement `fn iter(bs: BitSet64) -> impl Iterator<Item = u32>` using `u64::trailing_zeros()` to find the lowest set bit, then `self.0 & (self.0 - 1)` to clear it — O(count) iterations, not O(64).
2. **Sudoku constraint**: Use a `BitSet64` to track available digits (1–9) for each sudoku cell; implement `remove_candidates(cell_set, digit)` that clears the bit for a placed digit across a row/column/box.
3. **Graph coloring**: Represent a graph's adjacency as `Vec<BitSet64>` (one per node); implement greedy graph coloring using `BitSet64` to track used colors among neighbors.
