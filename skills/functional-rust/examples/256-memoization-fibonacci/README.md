# 256: Memoization — Fibonacci with Hashtable Cache

**Difficulty:** 2  **Level:** Intermediate

Cache computed results to avoid repeated work — apply transparent memoization to recursive functions.

## The Problem This Solves

Naive recursive Fibonacci recomputes `fib(n-1)` and `fib(n-2)` from scratch every call. Computing `fib(40)` makes billions of redundant calls. The fix is memoization: the first time you compute `fib(n)`, store the result. Every subsequent call for the same `n` returns the cached value instantly.

The FP ideal is *transparent* memoization: wrap any function and get a cached version back, with no changes to the original. OCaml achieves this elegantly — a `memoize` higher-order function captures a `Hashtbl` in a closure. The recursive Fibonacci uses `let rec ... and` mutual recursion to wire the cache in automatically.

Rust's ownership model makes this harder. Closures can't easily be both recursive and mutable. This example shows three clean patterns: struct-based (owns the cache explicitly), HOF with `RefCell` (interior mutability inside a closure), and thread-local (truly transparent call site, like OCaml).

## The Intuition

Memoization is a lookup table for function calls: "have I computed this input before? If yes, return the stored answer. If no, compute it, store it, and return it."

The challenge in Rust is that memoization needs mutable state (the cache) inside a function. Rust's borrow checker doesn't allow a shared mutable reference inside a recursive closure by default. `RefCell` solves this by moving the borrow check to runtime — safe but requires discipline.

`thread_local!` goes furthest: it hides the cache entirely behind a module-level slot. The function looks like a pure function from the call site. The cache is real but invisible.

## How It Works in Rust

```rust
// Style 1: Struct-based — mutable state is explicit in &mut self
pub struct FibMemo {
    cache: HashMap<u64, u64>,
}

impl FibMemo {
    pub fn fib(&mut self, n: u64) -> u64 {
        if let Some(&v) = self.cache.get(&n) { return v; }  // cache hit
        let v = if n <= 1 { n } else {
            self.fib(n - 1) + self.fib(n - 2)  // recursive — works with &mut self
        };
        self.cache.insert(n, v);
        v
    }
}

// Style 2: HOF with RefCell — mirrors OCaml's memoize, recursion via named function
pub fn memoize<T, U, F>(f: F) -> impl FnMut(T) -> U
where T: Eq + Hash + Clone, U: Clone, F: Fn(T) -> U {
    let cache = RefCell::new(HashMap::new());
    move |x: T| {
        if let Some(v) = cache.borrow().get(&x) { return v.clone(); }
        let v = f(x.clone());  // f must not itself use the cache (not recursive)
        cache.borrow_mut().insert(x, v.clone());
        v
    }
}

// Style 3: thread_local — truly transparent, identical call site to OCaml
thread_local! {
    static FIB_CACHE: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::new());
}
pub fn fib_tl(n: u64) -> u64 {
    FIB_CACHE.with(|c| {
        if let Some(&v) = c.borrow().get(&n) { return v; }
        let v = if n <= 1 { n } else { fib_tl(n-1) + fib_tl(n-2) };
        c.borrow_mut().insert(n, v);
        v
    })
}
```

## What This Unlocks

- **Dynamic programming** — memoized recursion is top-down DP; the cache is automatically populated in dependency order.
- **Expensive pure functions** — HTTP responses, database lookups, heavy computations — wrap with memoize for automatic caching.
- **Test isolation** — the struct-based pattern makes the cache an owned value; create a fresh `FibMemo::new()` per test, no shared state between tests.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Transparent cache | `memoize` HOF captures `Hashtbl` | `RefCell<HashMap>` inside closure, or `thread_local!` |
| Recursive memoization | `let rec ... and` mutual binding | Struct `&mut self`, or explicit cache param |
| Interior mutability | GC handles aliasing | `RefCell` — runtime borrow check |
| Thread safety | `Hashtbl` not thread-safe | `thread_local!` isolates per thread; `Mutex<HashMap>` for sharing |
| Call-site transparency | HOF: identical to non-memoized | `thread_local!` style only |
