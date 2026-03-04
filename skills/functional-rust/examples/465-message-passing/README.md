# 465: Message Passing vs Shared Memory

**Difficulty:** 3  **Level:** Intermediate

Compare two correct approaches to concurrent state: channels (message passing) vs `Arc<Mutex<T>>` (shared memory) — and know when to choose each.

## The Problem This Solves

You need multiple threads to update a shared data structure — say, a word-frequency counter. Both approaches are correct, but they have very different characteristics. Picking the wrong one either kills performance or makes reasoning about your code unnecessarily hard.

With shared memory (`Arc<Mutex<HashMap>>`), every thread grabs the lock for every word. This is fast when updates are tiny and frequent, but the mutex becomes a bottleneck if threads spend significant time holding it. Under high contention, threads spend more time waiting than working.

With message passing, each thread accumulates a partial result and sends it all at once. There's no contention during computation — threads work independently. The merge step at the end is the only coordination point. The downside: you're allocating and sending intermediate `HashMap`s, which has overhead for very fine-grained updates.

## The Intuition

Message passing says "share data by communicating a result once"; shared memory says "communicate by accessing shared data many times" — message passing is easier to reason about and scales better under high contention; shared memory is faster for high-frequency tiny updates. The Go proverb applies: *do not communicate by sharing memory; share memory by communicating.*

## How It Works in Rust

```rust
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

let words = vec!["foo", "bar", "foo", "baz", "bar", "foo"];

// --- Message Passing Approach ---
let (tx, rx) = mpsc::channel::<HashMap<&str, usize>>();

let words_mp = words.clone();
thread::spawn(move || {
    let mut local: HashMap<&str, usize> = HashMap::new();
    for word in words_mp {
        *local.entry(word).or_insert(0) += 1;  // accumulate locally
    }
    tx.send(local).unwrap();                    // send result once
});

let result_mp = rx.recv().unwrap();             // no contention during processing

// --- Shared Memory Approach ---
let shared: Arc<Mutex<HashMap<&str, usize>>> = Arc::new(Mutex::new(HashMap::new()));
let shared2 = Arc::clone(&shared);

thread::spawn(move || {
    for word in &words {
        let mut map = shared2.lock().unwrap();  // lock per update — contention here
        *map.entry(word).or_insert(0) += 1;
    }
});
```

## What This Unlocks

- **High-contention aggregation**: message passing wins — threads compute independently, merge once at the end.
- **Fine-grained counters**: shared memory wins — a per-key `AtomicUsize` is faster than round-tripping through a channel for every increment.
- **Actor design**: message passing naturally leads to the actor model — no shared state at all.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Message passing | Manual queue + result accumulation | `mpsc::channel` + partial `HashMap` |
| Shared memory | `ref` + `Mutex` | `Arc<Mutex<HashMap>>` |
| Lock granularity | Manual | Per `lock()` call |
| Ownership transfer | Copied or GC-managed | Moved into channel — zero-copy |
| Best for | Batch, independent workers | Fine-grained, high-frequency updates |
| Reasoning difficulty | Easier (no shared state) | Harder (must reason about lock ordering) |
