📖 **[View on hightechmind.io →](https://hightechmind.io/rust/520-closure-higher-order)**

---

# Higher-Order Functions

## Problem Statement

Higher-order functions (HOFs) — functions that take or return other functions — are the backbone of functional programming. They emerged from lambda calculus (Alonzo Church, 1930s) and appeared in practical form in LISP, ML, and Haskell. Rust's iterator API is built entirely on HOFs: `map`, `filter`, `fold`, `flat_map`, `zip`. Beyond the standard library, custom HOFs like `zip_with`, `scan_left`, and `group_by` allow expressing complex data transformations as pipelines without intermediate allocations or imperative loops.

## Learning Outcomes

- How to write custom higher-order functions using generic `Fn` bounds
- The difference between `map`/`filter` (element-wise) and `fold`/`scan` (accumulating)
- How `zip_with` generalizes `zip + map` into a single combining step
- How `scan_left` produces all intermediate fold values (useful for running totals)
- How to implement `partition_by` and `group_by` using closures as classifiers

## Rust Application

`zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>` combines two slices element-wise using any combining function. `scan_left` mirrors Haskell's `scanl` — it returns a `Vec` containing `init` followed by each intermediate accumulation. `partition_by<T, P>` wraps `Iterator::partition` with a custom predicate. `group_by<T>` groups consecutive equal elements into sublists. All accept `Fn`-bounded generic parameters so they work with both closures and named functions.

Key patterns:
- `F: Fn(&A, &B) -> C` — combining function over references
- `a.iter().zip(b.iter()).map(|(x, y)| f(x, y)).collect()` — zip then map
- `items.iter().take_while(|x| pred(x)).cloned().collect()` — lazy short-circuit

## OCaml Approach

OCaml's `List` module provides `map`, `filter`, `fold_left`, `fold_right`, and `partition` as stdlib HOFs. Custom combinators like `zip_with` and `scan_left` are straightforward to write and compose. OCaml's structural tail-call optimization makes recursive HOFs efficient without iterators.

```ocaml
let zip_with f xs ys = List.map2 f xs ys
let scan_left f init xs =
  List.fold_left (fun (acc, res) x ->
    let acc' = f acc x in (acc', acc' :: res)
  ) (init, [init]) xs |> snd |> List.rev
```

## Key Differences

1. **Iterator vs recursion**: Rust HOFs operate on `Iterator` chains with lazy evaluation and optional collect; OCaml HOFs typically process `list` values recursively, though `Seq` provides laziness.
2. **Zero-copy iteration**: Rust `&[T]` slices avoid allocation in HOFs that don't need ownership; OCaml lists are always heap-allocated linked lists.
3. **Trait bounds**: Rust requires explicit `Fn`, `FnMut`, or `FnOnce` bounds on HOF parameters; OCaml infers the function type structurally without named bounds.
4. **Return types**: Rust HOFs returning `impl Fn` cannot name the concrete type; OCaml HOFs return plain function types that are fully transparent to the type checker.

## Exercises

1. **`unfold`**: Implement `unfold<T, S, F>(seed: S, f: F) -> Vec<T> where F: Fn(S) -> Option<(T, S)>` that generates a vector by repeatedly applying `f` until it returns `None`.
2. **`chunk_by`**: Write a HOF that groups elements into fixed-size chunks, returning a `Vec<Vec<T>>`, handling the remainder chunk if the length is not divisible.
3. **`find_map`**: Implement `find_map<T, U, F>(items: &[T], f: F) -> Option<U> where F: Fn(&T) -> Option<U>` that returns the first `Some` result from applying `f` to each element.
