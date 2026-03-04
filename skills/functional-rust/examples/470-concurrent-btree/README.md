# 470: Concurrent B-tree

**Difficulty:** 3  **Level:** Intermediate

A sharded sorted map approximates a concurrent ordered index — multiple threads read and write without blocking each other.

## The Problem This Solves

`HashMap` gives O(1) operations but loses ordering. `BTreeMap` gives sorted iteration, range queries, and `O(log n)` operations — but a single `Mutex<BTreeMap>` serialises everything. For workloads that need both order *and* concurrency, you need something smarter.

The key insight for ordered maps is *range sharding*: instead of hashing keys to shards, assign each shard a key range (e.g. shard 0: keys A–F, shard 1: G–M, ...). A `BTreeMap` within each shard stays sorted. Range queries that span shards merge sorted results from multiple shards in order.

This is how distributed databases partition sorted indexes — the same pattern scales from a single machine to a cluster.

## The Intuition

A library's card catalogue divided alphabetically across 26 drawers. Looking up a specific book? Go to the right drawer immediately. Browsing all books by authors starting with 'M'? Open just that drawer. Browsing 'L' through 'N'? Open three drawers and merge. Different drawers can be accessed simultaneously by different patrons.

## How It Works in Rust

1. **Range shards** — define key boundaries and a `Vec<RwLock<BTreeMap<K, V>>>`:
   ```rust
   const SHARDS: usize = 16;
   // Each shard covers 1/16th of the key space
   ```
2. **Shard routing** — map a key to its shard using the key's hash range or explicit boundaries:
   ```rust
   fn shard_for<K: Ord + Hash>(key: &K) -> usize { /* hash-based partition */ }
   ```
3. **Point operations** — acquire only the relevant shard's lock:
   ```rust
   let shard = &self.shards[shard_for(&key)];
   shard.write().unwrap().insert(key, value);
   ```
4. **Range scan** — iterate affected shards in order, merge results:
   ```rust
   for shard in &self.shards {
       let guard = shard.read().unwrap();
       for (k, v) in guard.range(lo..=hi) { /* ... */ }
   }
   ```

## What This Unlocks

- **Concurrent ordered access** — range queries and sorted iteration alongside concurrent writes.
- **Reduced lock contention** — N shards means N-fold reduction in expected lock contention.
- **Stepping stone** — understanding sharded B-trees prepares you for distributed databases (CockroachDB, TiKV) that use the same pattern at scale.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sorted concurrent map | `Map` + `Mutex` | Sharded `RwLock<BTreeMap>` |
| Range queries | Sequential only | Parallel shard scan + merge |
| Crate option | — | `evmap` for eventually consistent, custom for strong consistency |
| Ordering guarantee | Total order on inserts | Per-shard sorted order, merge for global |
