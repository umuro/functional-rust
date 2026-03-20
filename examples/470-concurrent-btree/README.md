📖 **[View on hightechmind.io →](https://hightechmind.io/rust/470-concurrent-btree)**

---

# Concurrent BTree

A thread-safe ordered map backed by `BTreeMap` and a single `RwLock`, providing sorted key iteration, range queries, and first/last-key lookups under concurrent access.

## Problem Statement

`HashMap` is unordered: range queries like "give me all users with scores between 80 and 100" require a full scan. B-Trees maintain sorted order, enabling O(log N) range queries and ordered iteration. Databases (PostgreSQL B-tree indexes), filesystem directories (HFS+, ext4 htree), and leaderboard services all rely on sorted key structures. A concurrent wrapper makes these operations safe under multi-threaded access while preserving the ordering guarantees.

## Learning Outcomes

- Understand when sorted order justifies the overhead of `BTreeMap` over `HashMap`
- Wrap a standard library collection with `RwLock` for concurrent access
- Implement range queries using `BTreeMap::range` with `RangeBounds<K>`
- Build `ConcurrentSortedSet` as a zero-value `BTreeMap<T, ()>` wrapper
- Recognise the trade-off between a coarse single-lock design and fine-grained sharding

## Rust Application

`ConcurrentBTree` wraps `BTreeMap<K, V>` in an `RwLock`:

```rust
pub struct ConcurrentBTree<K, V> {
    inner: RwLock<BTreeMap<K, V>>,
}
```

Reads (`get`, `contains_key`, `first_key`, `last_key`, `range`, `keys`) acquire a shared read lock, allowing multiple concurrent readers. Writes (`insert`, `remove`, `clear`) acquire an exclusive write lock. The range method accepts any `R: RangeBounds<K>` — including `..`, `a..b`, `a..=b` — and returns a cloned `Vec<(K,V)>` so the lock is released before the caller iterates. `ConcurrentSortedSet` delegates to `ConcurrentBTree<T, ()>`, providing a set API with O(log N) membership and sorted iteration.

## OCaml Approach

OCaml's standard library `Map.Make(Ord)` is a purely functional persistent map — immutable and naturally thread-safe for reads. Mutations return new maps, so concurrent writers need an `Atomic` reference or a `Mutex`:

```ocaml
module IMap = Map.Make(Int)
let map_ref = ref IMap.empty
let mu = Mutex.create ()

let insert k v =
  Mutex.lock mu;
  map_ref := IMap.add k v !map_ref;
  Mutex.unlock mu

let range lo hi =
  IMap.filter (fun k _ -> k >= lo && k <= hi) !map_ref
```

The functional map's persistent nature means readers can capture a snapshot by reading the reference once without locking.

## Key Differences

1. **Mutability model**: Rust's `BTreeMap` is mutable in-place; OCaml's `Map.Make` returns new immutable trees on every update, providing snapshot isolation for free.
2. **Range API**: Rust's `BTreeMap::range` returns a lazy iterator using `RangeBounds<K>`; OCaml's `Map.filter` scans the whole tree — there is no built-in O(log N) range iterator in the standard library.
3. **Lock granularity**: This example uses a single `RwLock`; a production concurrent B-tree (like `BwTree` or `LMDB`) uses latch coupling — locking one level at a time — for higher write throughput.
4. **`Ord` requirement**: Both Rust (`K: Ord`) and OCaml (`Map.Make(Ord)`) require a total order; the functor/trait mechanism is the same conceptually but syntactically different.

## Exercises

1. **Concurrent range scan**: Spawn 8 reader threads each running `tree.range(0..500)` while 4 writer threads insert keys 0–999. Verify correctness and measure read latency.
2. **Snapshot consistency**: Implement `snapshot(&self) -> BTreeMap<K,V>` that clones the entire tree under a read lock, providing a point-in-time consistent view for long-running analytics.
3. **Latch coupling**: Research latch-coupling B-tree algorithms and sketch (without implementing) how you would replace the single `RwLock` with per-node locks to improve write concurrency.
