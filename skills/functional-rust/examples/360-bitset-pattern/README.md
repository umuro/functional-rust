# 360: Bitset Pattern — Bitmask as Dense Boolean Set

**Difficulty:** 3  **Level:** Advanced

Represent a set of small integers as a single integer where each bit is a membership flag — set union, intersection, and membership test in one CPU instruction.

## The Problem This Solves

You're tracking which of 64 possible features are enabled for a user, which cells in a grid are alive (Game of Life), or which nodes in a small graph have been visited. A `HashSet<u8>` works, but it allocates heap memory, adds pointer chasing, and turns every membership test into a hash computation.

A bitset compresses all of this. With 64 elements, the entire set fits in one `u64` — a single 8-byte integer. Membership test is a bitwise AND (`(mask >> i) & 1`). Union is bitwise OR. Intersection is bitwise AND. Complement is bitwise NOT. These are single CPU instructions, and modern CPUs can process 64-element sets in a single clock cycle.

The pattern scales to larger sets by using arrays of `u64` (or the `bitvec` crate for arbitrary sizes). Even a 1024-element bitset fits in 128 bytes — entirely in L1 cache — where a `HashSet<u16>` would use kilobytes of heap memory with pointer indirection.

## The Intuition

Python doesn't have built-in bitsets, but you can replicate the pattern: `mask | (1 << i)` to set bit i, `mask & (1 << i)` to test it. The pattern is identical in Rust, just with explicit types.

The key insight: if your universe of elements is small (≤ 64) and represented as integers, a bitset is almost always the right choice over `HashSet`. You trade generality (arbitrary types) for speed (single-instruction operations) and density (8 bytes vs. heap allocation).

## How It Works in Rust

```rust
// A bitset for elements 0..63 using a single u64
struct BitSet64(u64);

impl BitSet64 {
    fn new() -> Self { BitSet64(0) }

    fn insert(&mut self, i: u32) {
        assert!(i < 64);
        self.0 |= 1u64 << i; // set bit i
    }

    fn remove(&mut self, i: u32) {
        self.0 &= !(1u64 << i); // clear bit i
    }

    fn contains(&self, i: u32) -> bool {
        (self.0 >> i) & 1 == 1 // test bit i
    }

    // Set union — one CPU instruction
    fn union(&self, other: &Self) -> Self {
        BitSet64(self.0 | other.0)
    }

    // Set intersection — one CPU instruction
    fn intersection(&self, other: &Self) -> Self {
        BitSet64(self.0 & other.0)
    }

    // Count elements — popcount, hardware-accelerated
    fn len(&self) -> u32 {
        self.0.count_ones()
    }

    // Iterate over set bits
    fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        (0u32..64).filter(move |&i| self.contains(i))
    }
}

// Usage
let mut a = BitSet64::new();
a.insert(0); a.insert(3); a.insert(7);

let mut b = BitSet64::new();
b.insert(3); b.insert(5); b.insert(7);

let both = a.intersection(&b);
for elem in both.iter() {
    println!("{elem}"); // 3, 7
}

// Flags pattern — named bits via constants
const FLAG_READ:  u32 = 0;
const FLAG_WRITE: u32 = 1;
const FLAG_EXEC:  u32 = 2;

let mut perms = BitSet64::new();
perms.insert(FLAG_READ);
perms.insert(FLAG_WRITE);
println!("can exec: {}", perms.contains(FLAG_EXEC)); // false
```

## What This Unlocks

- **Permission systems and feature flags**: represent user permissions as a `u64`, check any permission with one bitwise AND, combine role permissions with bitwise OR.
- **Graph algorithms**: visited sets for small graphs, adjacency matrices for dense graphs under 64 nodes — all as simple integer arithmetic.
- **SIMD-friendly data**: bitset operations over large arrays can be auto-vectorized, processing 256 or 512 elements per instruction with AVX2/AVX-512.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Dense boolean set | `Bytes` / `Bigarray` | `u64` bitmask or `[u64; N]` |
| Membership test | array index + compare | bitwise AND (`mask & (1 << i)`) |
| Set union | loop + OR | bitwise OR (`a \| b`) |
| Set intersection | loop + AND | bitwise AND (`a & b`) |
| Element count | manual loop | `.count_ones()` (hardware popcount) |
| Arbitrary size | `Bigarray` | `bitvec` crate or `Vec<u64>` |
