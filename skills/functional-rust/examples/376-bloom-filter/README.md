# 376: Probabilistic Bloom Filter

**Difficulty:** 3  **Level:** Advanced

Space-efficient probabilistic set: false positives possible, false negatives impossible.

## The Problem This Solves

You're building a cache and want to avoid expensive lookups for keys that definitely aren't cached. Or you're a database checking whether a record exists before hitting disk. Or you're a web crawler tracking visited URLs. In each case, you don't need certainty — you need a fast "probably yes / definitely no" answer.

A `HashSet<T>` gives you exact answers but uses memory proportional to the number of elements. A bloom filter gives you probabilistic answers using a fixed, tiny amount of memory regardless of how many elements you add. A standard bloom filter with 1% false positive rate uses about 9.6 bits per element — compare that to storing a 64-byte URL string (512 bits) in a hash set.

The constraint: you can never remove elements from a standard bloom filter (removing would introduce false negatives). Counting bloom filters and cuckoo filters solve this but at higher cost.

## The Intuition

A bloom filter is a bit array of M bits plus K hash functions. To add an element: compute K hashes, set those K bit positions to 1. To check membership: compute K hashes, check if all K bits are 1. If any bit is 0, the element is definitely not in the set. If all bits are 1, it's *probably* in the set — but a false positive is possible if those bits were set by other elements.

The math: false positive probability ≈ `(1 - e^(-kn/m))^k` where n = elements inserted. You tune M (bits) and K (hash functions) based on your desired false positive rate and expected element count.

## How It Works in Rust

```rust
use bloomfilter::Bloom;

// Create a bloom filter for 1000 items with 1% false positive rate
let mut bloom = Bloom::new_for_fp_rate(1000, 0.01);

bloom.set("alice@example.com");
bloom.set("bob@example.com");

// Definitely not in set
assert!(!bloom.check("unknown@example.com"));

// Probably in set (could be false positive, never false negative)
assert!(bloom.check("alice@example.com"));

println!("Bitmap size: {} bytes", bloom.number_of_bits() / 8);
// Much smaller than storing the strings directly
```

Add to `Cargo.toml`: `bloomfilter = "1"`

For a manual implementation: allocate a `Vec<u64>` as a bit array, apply 2 hash functions with `k` seeds using double hashing: `hash_i = (h1 + i * h2) % m`.

## What This Unlocks

- **Cache guards** — check bloom filter before expensive DB/network lookup; skip on definite miss.
- **Deduplication at scale** — web crawlers track billions of visited URLs in megabytes of bloom filter memory.
- **Distributed systems** — CRDTs, gossip protocols, and anti-entropy use bloom filters to sync state efficiently.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Exact set | `Set` (balanced BST) or `Hashtbl` | `HashSet<T>` |
| Probabilistic set | No stdlib equivalent | `bloomfilter::Bloom` or manual bit array |
| Memory use | O(n) regardless | O(1) fixed size — independent of element count |
| False negatives | Impossible in exact set | Impossible in bloom filter |
| False positives | Impossible in exact set | Tunable probability in bloom filter |
