# 466: Concurrent HashMap

**Difficulty:** 3  **Level:** Intermediate

A sharded map lets multiple threads read and write simultaneously without global locking.

## The Problem This Solves

A single `Mutex<HashMap>` serialises every access — inserts, lookups, and deletions all queue behind the same lock. Under high concurrency this becomes a bottleneck: 16 threads fighting over one lock achieve barely more throughput than 1.

The standard fix is *sharding*: split the map into N independent `RwLock<HashMap>` buckets. Each key hashes to exactly one shard. Operations on different shards proceed in parallel, with no contention. Operations on the same shard still serialize, but the probability of collision drops by N.

This is the pattern behind DashMap, Java's `ConcurrentHashMap`, and most production-grade concurrent maps.

## The Intuition

Imagine a library with 16 shelves. Instead of one librarian who handles all requests, 16 librarians each manage their shelf. You can all check out books simultaneously as long as you're at different shelves. Only if two people need the same shelf do they have to take turns.

## How It Works in Rust

1. **Choose shard count** — a power of two (16–64) is typical:
   ```rust
   const SHARDS: usize = 16;
   let shards: Vec<RwLock<HashMap<K, V>>> = (0..SHARDS).map(|_| RwLock::new(HashMap::new())).collect();
   ```
2. **Hash to shard**:
   ```rust
   fn shard_index<K: Hash>(key: &K) -> usize {
       let mut h = DefaultHasher::new();
       key.hash(&mut h);
       h.finish() as usize % SHARDS
   }
   ```
3. **Read with shared lock**:
   ```rust
   let idx = shard_index(&key);
   let guard = shards[idx].read().unwrap();
   guard.get(&key).cloned()
   ```
4. **Write with exclusive lock**:
   ```rust
   let idx = shard_index(&key);
   let mut guard = shards[idx].write().unwrap();
   guard.insert(key, value);
   ```
5. **Wrap in `Arc`** to share across threads: `Arc<ShardedMap<K, V>>`.

## What This Unlocks

- **Parallel reads at no cost** — `RwLock` lets N readers hold locks simultaneously; contention only occurs for writers.
- **Near-linear write scaling** — with 16 shards, expected lock contention is 1/16th of an unsharded map.
- **Foundation for DashMap** — the popular `dashmap` crate is a battle-tested, production-ready version of exactly this pattern.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Concurrent map | `Hashtbl` + `Mutex` per thread | Sharded `RwLock<HashMap>` |
| Read concurrency | Blocked by any lock | Multiple readers via `RwLock` |
| Crate option | None in stdlib | `dashmap::DashMap` |
| Thread safety | Runtime exception risk | Compile-time `Send`/`Sync` |
