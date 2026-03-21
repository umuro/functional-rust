📖 **[View on hightechmind.io →](https://hightechmind.io/rust/843-memoization-generic)**

---

# Generic Memoization
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Recursive functions with overlapping subproblems waste exponential time recomputing the same results. Memoization wraps a function with a cache: on the first call with given arguments, compute and store the result; on subsequent calls, return the cached value. While specific DP algorithms hard-code their memoization tables, generic memoization provides a reusable cache wrapper for any pure function. This pattern is fundamental in functional languages (Haskell's `memoize`, OCaml's `Hashtbl`-based wrappers) and enables transparent DP without restructuring code. Real-world uses: web request caching, computed property memoization in UI frameworks, and expensive query result caching.

## Learning Outcomes

- Implement a `HashMap`-backed memoizer that wraps any `Fn(K) -> V` where K is hashable
- Handle recursive memoization using `RefCell<HashMap>` for interior mutability
- Understand the challenge: recursive memoized functions cannot easily call themselves through the cache
- Apply memoization to Fibonacci, LCS, and graph shortest paths to see exponential → polynomial speedup
- Recognize the difference from tabulation: memoization is top-down and lazy; tabulation is bottom-up and eager

## Rust Application

```rust
use std::collections::HashMap;
use std::cell::RefCell;

pub struct Memoize<K, V> {
    cache: RefCell<HashMap<K, V>>,
    func: Box<dyn Fn(K, &Self) -> V>,
}
impl<K: Eq + std::hash::Hash + Clone, V: Clone> Memoize<K, V> {
    pub fn call(&self, key: K) -> V {
        if let Some(v) = self.cache.borrow().get(&key) {
            return v.clone();
        }
        let v = (self.func)(key.clone(), self);
        self.cache.borrow_mut().insert(key, v.clone());
        v
    }
}
```

`RefCell<HashMap>` provides interior mutability — the cache can be mutated through a shared reference, enabling recursive calls. The function receives `&Self` as a second argument, allowing it to call `self.call(subproblem)` for recursive memoization. `K: Clone` is needed to insert the key after the computation (the borrow check prevents holding a borrow during computation). This pattern avoids the "cannot borrow as mutable because it's behind a shared reference" compile error that naive approaches hit.

## OCaml Approach

OCaml memoization is simpler: `let memoize f = let tbl = Hashtbl.create 16 in fun x -> match Hashtbl.find_opt tbl x with Some v -> v | None -> let v = f x in Hashtbl.add tbl x v; v`. Recursive memoization uses a `ref` for the function: `let rec_memo f = let r = ref (fun _ -> assert false) in let m = memoize (fun x -> f !r x) in r := m; m`. OCaml's first-class mutability makes this straightforward. The `Weak.t` module enables cache eviction for memory-constrained situations.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Interior mutability | `RefCell<HashMap>` | `Hashtbl` (inherently mutable) |
| Recursive wrapper | `&Self` second arg | `ref` to function |
| Key constraint | `Eq + Hash + Clone` | `Hashtbl` equality |
| Thread safety | Use `Mutex<HashMap>` instead | Not needed for single-threaded |
| Eviction | Manual `HashMap::retain` | `Weak.t` for GC integration |
| Lazy vs eager | Lazy (top-down) | Same |

## Exercises

1. Implement thread-safe memoization using `Arc<Mutex<HashMap>>` and measure the synchronization overhead.
2. Add a cache eviction policy: LRU (evict least recently used) or LFU (evict least frequently used).
3. Implement `memoize_recursive` that allows the function to call itself via the memoized wrapper without `&Self` threading.
4. Benchmark memoized Fibonacci(50) vs. tabulated DP and compare code clarity.
5. Implement memoization with TTL (time-to-live): entries expire after a configurable duration.
