📖 **[View on hightechmind.io →](https://hightechmind.io/rust/507-closure-memoization)**

---

# Closure Memoization
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



Memoization wraps a function in a cache layer that stores previously computed results, returning the cached value on repeated calls instead of recomputing — a classic time-space trade-off applicable to any pure function.

## Problem Statement

Recursive algorithms like Fibonacci without memoization recompute exponentially many subproblems. `fib(40)` calls `fib(39)` and `fib(38)`, which each call their children — the same subtree computed millions of times. Memoization eliminates this redundancy by storing results in a `HashMap`. More generally, any **referentially transparent** (pure) function can be memoized: database query caches, HTTP response caches, and compiler analysis passes all apply this pattern.

## Learning Outcomes

- Build a generic `Memoize<F, A, R>` wrapper that caches results by input
- Use `HashMap<A, R>` where `A: Eq + Hash + Clone` as the cache store
- Implement recursive memoization by passing the cache as a separate parameter to inner functions
- Understand why a closure cannot call itself directly (no self-reference in closures)
- Apply `cache_size()` to verify cache hit behaviour

## Rust Application

`Memoize::call` checks the cache first; on a miss, calls the inner function and stores the result:

```rust
pub fn call(&mut self, arg: A) -> R {
    if let Some(result) = self.cache.get(&arg) { return result.clone(); }
    let result = (self.func)(arg.clone());
    self.cache.insert(arg, result.clone());
    result
}
```

Recursive memoization for Fibonacci uses a named inner function that borrows the cache mutably:

```rust
fn fib_inner(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) { return result; }
    let result = if n <= 1 { n } else { fib_inner(n-1, cache) + fib_inner(n-2, cache) };
    cache.insert(n, result);
    result
}
```

A named function is used for the recursive case because closures cannot reference themselves.

## OCaml Approach

OCaml's `let rec` makes self-referential closures trivial:

```ocaml
let memoize f =
  let tbl = Hashtbl.create 16 in
  fun x -> match Hashtbl.find_opt tbl x with
    | Some v -> v
    | None -> let v = f x in Hashtbl.add tbl x v; v

let fib =
  let tbl = Hashtbl.create 64 in
  let rec go n = match Hashtbl.find_opt tbl n with
    | Some v -> v
    | None -> let v = if n <= 1 then n else go (n-1) + go (n-2) in
              Hashtbl.add tbl n v; v
  in go
```

OCaml's `let rec` allows the closure to reference itself directly, simplifying recursive memoization.

## Key Differences

1. **Self-referential closures**: Rust closures cannot reference themselves; a named function or explicit `Y combinator` is needed for recursive memoization. OCaml's `let rec` closure can.
2. **`Clone` requirement**: Rust's `Memoize` requires `R: Clone` to return cached values (since `HashMap::get` returns `&R`); OCaml's polymorphic `Hashtbl` has no such constraint.
3. **`FnMut` requirement**: `Memoize::call` takes `&mut self` because it modifies the cache — callers need `mut memo`; OCaml's `tbl` is a mutable reference captured by the closure.
4. **Thread safety**: `Memoize` is single-threaded; thread-safe memoization requires `Arc<Mutex<HashMap>>` or `dashmap`. OCaml's `Mutex.protect` serves the same purpose.

## Exercises

1. **TTL cache**: Extend `Memoize` with a `Duration` TTL per entry — store `(Instant, R)` and re-compute when the entry expires.
2. **LRU memoize**: Combine memoization with an LRU eviction policy (from example 375) to bound memory usage.
3. **Thread-safe memoize**: Implement `ThreadSafeMemoize<F, A, R>` using `Arc<Mutex<HashMap<A, R>>>` and verify correctness under concurrent calls with `thread::scope`.
