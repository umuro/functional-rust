📖 **[View on hightechmind.io →](https://hightechmind.io/rust/015-replicate-n-times)**

---

# 015 — Replicate Elements N Times
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Replicating each element `n` times (OCaml 99 Problems #15) generalizes duplication: `replicate([a, b, c], 3)` produces `[a, a, a, b, b, b, c, c, c]`. This is a direct application of `flat_map` with `std::iter::repeat`, combining two fundamental iterator patterns: structure expansion and repetition.

Replicate-then-flatten appears in data pipelines (upsampling time series), image processing (nearest-neighbor scaling), protocol encoding (preamble repetition), and test data generation. `std::iter::repeat(x).take(n)` is Rust's idiomatic way to generate n identical values — understanding this combination unlocks a large class of iterator transformations.

## Learning Outcomes

- Combine `flat_map` with `std::iter::repeat(x).take(n)` for efficient replication
- Pre-allocate output with `Vec::with_capacity(list.len() * n)` when n is known
- Understand that `replicate(n=1)` is identity, `replicate(n=0)` is filter-out-all
- Implement recursive replication with a nested helper
- Recognize this as the generalization of `duplicate` (example 014)

## Rust Application

`replicate` uses `.flat_map(|x| std::iter::repeat(x.clone()).take(n))` — the canonical idiom. Each element maps to a lazy iterator of `n` copies, and `flat_map` concatenates them. `replicate_prealloc` uses `Vec::with_capacity(list.len() * n)` and a nested loop — faster in practice because it avoids intermediate allocations. The recursive version uses a helper `repeat_elem` that builds `n` copies of one element via recursion, then appends the replication of the tail.

## OCaml Approach

OCaml's version: `let replicate lst n = List.concat_map (fun x -> List.init n (fun _ -> x)) lst`. `List.init n f` creates `[f 0; f 1; ...; f (n-1)]`; using `fun _ -> x` ignores the index and produces n copies. The tail-recursive version accumulates copies reversed: `List.rev (List.fold_left (fun acc x -> let copies = List.init n (fun _ -> x) in List.rev_append copies acc) [] lst)`.

## Key Differences

1. **`repeat().take(n)`**: Rust's `std::iter::repeat` produces an infinite iterator; `.take(n)` limits it. OCaml's `List.init n (fun _ -> x)` is finite by construction.
2. **Laziness**: Rust's `repeat(x).take(n)` is lazy — no allocation until consumed by `flat_map`. OCaml's `List.init` is eager.
3. **Clone cost**: Rust clones `x` once per copy. OCaml shares the same GC pointer for all `n` copies — O(1) space overhead per run in OCaml vs O(n) in Rust for heap-allocated types.
4. **n=0 behavior**: Both correctly produce an empty output for n=0. `repeat(x).take(0)` yields nothing; `List.init 0 f` produces `[]`.

## Exercises

1. **Benchmark**: Compare `replicate` (with `flat_map`) vs `replicate_prealloc` (with `with_capacity` + loop) on a `Vec<String>` of 1000 elements with n=100. Which is faster and why?
2. **Replicate with index**: Write `replicate_indexed(list: &[i32], n: usize) -> Vec<(usize, i32)>` that produces `n` copies of each element tagged with their original index: `[(0, a), (0, a), (1, b), (1, b), ...]`.
3. **Variable replication**: Write `replicate_var(list: &[i32], counts: &[usize]) -> Vec<i32>` where `counts[i]` specifies how many times to repeat `list[i]`. Use `zip` and `flat_map`.
