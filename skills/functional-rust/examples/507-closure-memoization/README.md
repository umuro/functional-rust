# 507: Closure Memoization

**Difficulty:** 3  **Level:** Intermediate

Cache expensive computation results inside a closure — compute once, return instantly on repeat calls.

## The Problem This Solves

Some computations are expensive: database lookups, API calls, recursive calculations like Fibonacci, or heavy string processing. If you call the same function repeatedly with the same arguments, you're wasting cycles recomputing known results.

A naive global cache requires unsafe static state or a mutex wrapping a global `HashMap`. A struct-based cache works but adds boilerplate — you need a new type for every function you want to memoize.

Closures solve this elegantly: wrap the expensive function in a closure that owns a `HashMap`. The closure's captured state *is* the cache. Call it like a normal function, but it transparently caches results. The entire memoization logic is self-contained.

## The Intuition

A memoized closure is a function with memory. The first time you call it with argument `5`, it computes the result and writes `{5: result}` in its internal notebook. The second time you call it with `5`, it checks the notebook first — instant answer.

In Python, you'd use `@functools.lru_cache` as a decorator. In JavaScript, you'd wrap the function in a closure with a `Map`. Rust's version is structurally identical: a closure capturing a `HashMap<K, V>`.

The key insight: the closure *must* be `FnMut`, not `Fn`. Every cache miss mutates the `HashMap`. A closure that only reads its captures is `Fn`; one that writes must be `FnMut`.

## How It Works in Rust

```rust
use std::collections::HashMap;
use std::hash::Hash;

// Generic memoizer: wraps any Fn(K)->V in an FnMut(K)->V with caching
fn memoize<K, V, F>(mut f: F) -> impl FnMut(K) -> V
where
    K: Eq + Hash + Clone,   // K must be hashable and cloneable for storage
    V: Clone,               // V must be cloneable to return from cache
    F: FnMut(K) -> V,
{
    let mut cache: HashMap<K, V> = HashMap::new();
    move |key: K| {
        if let Some(val) = cache.get(&key) {
            return val.clone();    // cache hit — return immediately
        }
        let val = f(key.clone());  // cache miss — compute
        cache.insert(key, val.clone());
        val
    }
}

// Usage: the cache lives inside the closure
let mut expensive = memoize(|n: i32| {
    println!("computing {}^2...", n);  // only prints on first call
    n * n
});

expensive(5);  // "computing 5^2..." → 25
expensive(5);  // silent → 25  (cached)
expensive(6);  // "computing 6^2..." → 36

// Fibonacci with memoization — wraps the recursive logic
fn make_fib() -> impl FnMut(u64) -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::new();
    cache.insert(0, 0);
    cache.insert(1, 1);

    fn fib_inner(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        if let Some(&v) = cache.get(&n) { return v; }
        let v = fib_inner(n-1, cache) + fib_inner(n-2, cache);
        cache.insert(n, v);
        v
    }
    move |n| fib_inner(n, &mut cache)  // cache is owned by the closure
}
let mut fib = make_fib();
println!("{}", fib(40)); // instant — all sub-results cached
```

## What This Unlocks

- **Expensive pure functions** — database lookups, template rendering, and cryptographic operations benefit from result caching without changing their call sites.
- **Recursive algorithms** — Fibonacci, shortest paths, and dynamic programming benefit enormously from memoization; the closure pattern keeps the cache private.
- **Rate-limited APIs** — memoize API call results to avoid hitting rate limits on repeated identical requests within a session.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable cache | `Hashtbl.t` with `ref` | `HashMap` captured by `FnMut` closure |
| Cache lookup | `Hashtbl.find_opt` | `HashMap::get(&key)` |
| First call | Compute + `Hashtbl.add` | Compute + `HashMap::insert` |
| Closure trait | N/A — all functions are `FnMut`-like | Must be `FnMut` (cache mutation) |
| Thread-safe version | `Mutex` + shared state | `Arc<Mutex<HashMap>>` in the closure |
