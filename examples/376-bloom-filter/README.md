📖 **[View on hightechmind.io →](https://hightechmind.io/rust/376-bloom-filter)**

---

# 376: Bloom Filter
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Checking exact membership in a large set requires storing all elements, which is expensive when the set contains billions of items (URLs, email addresses, IP addresses). Burton Howard Bloom introduced a probabilistic approach in 1970: use a bit array and multiple hash functions to answer "is this element in the set?" with guaranteed no false negatives but a tunable false positive rate. A Bloom filter can represent 1 billion URLs using only ~1.2 GB at 1% false positive rate — orders of magnitude less than a hash set.

Bloom filters appear in database join optimization (BigTable, Cassandra use them to avoid disk reads), network packet routing, malware URL databases (Chrome's Safe Browsing), spell checkers, and distributed cache pre-screening.

## Learning Outcomes

- Understand the mathematical relationship between bit array size, hash function count, and false positive rate
- Learn how to implement multiple independent hash functions from a single hash function using seeding
- Understand the no-false-negative guarantee and why deletion is not supported in basic Bloom filters
- See how bit manipulation with `u64` arrays efficiently represents the bit array in Rust
- Understand how to compute optimal parameters from desired capacity and false positive rate

## Rust Application

The implementation in `src/lib.rs` computes optimal bit array size `m` and hash function count `k` using the standard formulas derived from probability theory. The bit array is stored as `Vec<u64>`, using bitwise operations (`|=` and `>>`) to set and test individual bits. The `hash_val` method generates `k` independent hash values by seeding `DefaultHasher` with a sequence number, simulating multiple independent hash functions. The `contains` method uses `.all()` over an iterator range — idiomatic Rust for checking all k bits.

## OCaml Approach

In OCaml, a Bloom filter would use a `Bytes.t` or `Bigarray` for the bit array and the standard `Hashtbl` hashing infrastructure. OCaml's `Hashtbl.hash` is seeded via `Hashtbl.seeded_hash`. The functional style would express the k-hash check as `List.for_all` or a fold over hash seeds, keeping the core logic declarative.

## Key Differences

1. **Bit manipulation**: Rust uses `Vec<u64>` with explicit bitwise indexing (`i / 64`, `1u64 << (i % 64)`); OCaml would use `Bytes` with `Bytes.get_uint8`/`lor`/`land` operations.
2. **Hash seeding**: Rust seeds `DefaultHasher` with an integer via the `Hash` trait; OCaml uses `Hashtbl.seeded_hash` which accepts a seed directly.
3. **Generic bounds**: Rust requires `T: Hash` at compile time; OCaml uses polymorphic hashing (`Hashtbl.hash : 'a -> int`) without trait bounds.
4. **Memory layout**: Rust's `Vec<u64>` is contiguous stack-friendly memory; OCaml's `Bytes` is a heap-allocated boxed value with GC overhead.

## Exercises

1. **Counting Bloom filter**: Add a counter array alongside the bit array so items can be deleted. Each insert increments counters; delete decrements them and clears bits when count reaches zero.
2. **False positive measurement**: Insert N random strings, then query N different random strings and count actual false positives. Plot how the rate changes with `fp_rate` parameter values of 0.01, 0.05, and 0.10.
3. **Scalable Bloom filter**: Implement a filter that grows by adding new sub-filters when the current one exceeds capacity, maintaining the overall false positive guarantee by tightening each sub-filter's target rate.
