📖 **[View on hightechmind.io →](https://hightechmind.io/rust/449-rayon-join)**

---

# 449: rayon::join for Parallel Tasks

**Difficulty:** 3  **Level:** Intermediate

Run two independent computations in parallel and get both results back — the building block of parallel divide-and-conquer.

## The Problem This Solves

Many algorithms naturally split into two independent subproblems: merge sort splits a slice in half, quicksort partitions around a pivot, Fibonacci splits into two recursive calls. Running these halves sequentially wastes half your cores.

`std::thread::spawn` works but requires `'static` lifetimes and explicit `JoinHandle` management. Writing parallel merge sort with raw threads means careful lifetime juggling that obscures the algorithm.

`rayon::join` is the clean answer: pass two closures, get two results, all within the same lifetime scope. Rayon decides whether to run them on the current thread, a sibling thread, or steal them — always correctly.

## The Intuition

You're cooking a meal with a helper. You say "you do the salad, I'll do the main." You each work simultaneously. When both are done, you serve dinner. `rayon::join` is exactly that: fork two tasks, join when both finish. No manual thread handles, no lifetime headaches.

## How It Works in Rust

1. **Call `rayon::join`** with two closures that return values of potentially different types:
   ```rust
   let (left_result, right_result) = rayon::join(|| expensive_a(), || expensive_b());
   ```
2. **Rayon schedules** — it may run one closure inline and steal the other to a worker thread, or run both inline if the pool is busy. From your perspective it's always parallel-or-better.
3. **Lifetime scoping** — unlike `spawn`, closures can borrow from the enclosing scope because `join` doesn't return until both finish.
4. **Parallel merge sort example**:
   ```rust
   fn par_sort(v: &mut [i32]) {
       if v.len() <= 1 { return; }
       let mid = v.len() / 2;
       let (left, right) = v.split_at_mut(mid);
       rayon::join(|| par_sort(left), || par_sort(right));
       merge(v);
   }
   ```

## What This Unlocks

- **Parallel divide-and-conquer** — merge sort, quicksort, tree traversal all become naturally parallel.
- **Scoped borrowing** — both closures can reference local variables; Rayon guarantees they complete before the call returns.
- **Composable parallelism** — nest `rayon::join` calls recursively; Rayon's scheduler prevents oversubscription automatically.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Fork-join | `Domainslib.Task.async`/`await` | `rayon::join(f, g)` |
| Lifetime of captured data | GC-managed | Statically verified by borrow checker |
| Oversubscription | Manual pool sizing | Rayon work-steals within existing pool |
| Return values | Futures awaited | Tuple `(A, B)` returned directly |
