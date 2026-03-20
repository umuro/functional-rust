📖 **[View on hightechmind.io →](https://hightechmind.io/rust/466-concurrent-hashmap)**

---

# Concurrent HashMap
**Difficulty:** ⭐  
**Category:** Functional Programming  


A sharded thread-safe hashmap that reduces lock contention by partitioning keys across multiple independent RwLock-guarded buckets.

## Problem Statement

A single `Mutex<HashMap<K,V>>` serialises every read and write behind one global lock. Under high concurrency this becomes a bottleneck: readers that could run in parallel are forced to queue. The classic solution is **sharding** — split the keyspace into N buckets, each with its own lock. Threads touching different buckets never contend. This pattern underlies Java's `ConcurrentHashMap`, Redis cluster slots, and every high-throughput cache server.

## Learning Outcomes

- Understand why a single lock limits concurrent read throughput
- Implement consistent hash-based shard selection
- Use `RwLock` to allow multiple concurrent readers per shard
- Build a `CounterMap` that increments values atomically within a shard lock
- Apply the `entry().or_insert()` pattern under a held write guard

## Rust Application

The implementation uses 16 shards, each a `RwLock<HashMap<K,V>>`. A helper `shard_index` hashes the key with `DefaultHasher` and takes the result modulo `NUM_SHARDS`:

```rust
fn shard_index(&self, key: &K) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() as usize) % NUM_SHARDS
}
```

Reads acquire a shared `read()` lock; writes acquire an exclusive `write()` lock on only the relevant shard. The `update` method holds a write lock for the duration of the read-modify-write to prevent lost updates. `CounterMap` wraps the shard lock directly to implement an atomic increment without a separate read then write step.

## OCaml Approach

OCaml's `Hashtbl` is not thread-safe. Idiomatic concurrent maps use either:
- A `Mutex.t` wrapping a plain `Hashtbl.t` (simple, low concurrency)
- A functor-based sharded design with `Array` of `Mutex.t * ('k,'v) Hashtbl.t`
- The `Saturn` lock-free library's `Saturn.Htbl` for high-performance cases

```ocaml
let num_shards = 16
let shards = Array.init num_shards (fun _ ->
  (Mutex.create (), Hashtbl.create 16))

let shard_of key =
  (Hashtbl.hash key) mod num_shards

let insert key value =
  let (mu, tbl) = shards.(shard_of key) in
  Mutex.lock mu;
  Hashtbl.replace tbl key value;
  Mutex.unlock mu
```

## Key Differences

1. **Lock granularity**: Rust's `RwLock` allows concurrent reads within a shard; OCaml's `Mutex` is exclusive even for reads unless you implement a reader-writer lock manually.
2. **Type safety**: Rust enforces `K: Hash + Eq + Clone` and `V: Clone` at compile time; OCaml `Hashtbl` is polymorphic but unchecked.
3. **Poisoning**: Rust's `RwLock::write().unwrap()` panics if a thread died while holding the lock; OCaml mutexes do not have this concept.
4. **Entry API**: Rust's `entry().or_insert()` performs an atomic lookup-or-insert under one lock acquisition; OCaml requires a separate `mem`/`add` sequence.

## Exercises

1. **Resize shards**: Add a constructor `ConcurrentHashMap::with_shards(n: usize)` and benchmark shard counts 4, 16, 64 under mixed read/write workloads.
2. **Snapshot iterator**: Implement a `snapshot(&self) -> Vec<(K, V)>` that acquires all shard read locks in order and returns a consistent point-in-time view.
3. **LRU eviction**: Add a `capacity` per shard and evict the least-recently-used key when the shard is full, using a `VecDeque` as in the LRU cache example.
