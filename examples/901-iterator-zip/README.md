📖 **[View on hightechmind.io →](https://hightechmind.io/rust/901-iterator-zip)**

---

# 901-iterator-zip — Iterator Zip
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Pairing elements from two sequences is fundamental to many algorithms: matching names with scores, computing dot products, running pairwise comparisons, building dictionaries from key and value slices. OCaml's `List.combine` (zip) and `List.split` (unzip) are the eager equivalents. Rust's `.zip()` adapter is lazy and does not panic on length mismatch — it stops at the shorter iterator. This laziness makes zip safe for use with infinite iterators. The `.enumerate()` method is a special case of zip with an index sequence, and `.unzip()` is the inverse consumer.

## Learning Outcomes

- Use `.zip()` to pair elements from two iterators lazily
- Build a `HashMap` by zipping keys and values
- Use `.enumerate()` as zip-with-index (equivalent to OCaml's `List.mapi`)
- Use `.unzip()` to split an iterator of pairs into two collections
- Understand that `.zip()` stops at the shorter iterator — no panic on length mismatch

## Rust Application

`zip_slices` uses `a.iter().zip(b.iter()).map(|(&x, &y)| (x, y)).collect()`. `names_to_scores` zips two slices directly into a `HashMap` via `.map(...).collect()`. `indexed` wraps `.enumerate()` as a reusable function. `unzip_pairs` uses `.into_iter().unzip()` to split a `Vec<(A, B)>` into `(Vec<A>, Vec<B>)`. The tests verify that zip stops silently at the shorter input — `zip_slices(&[1,2,3], &[10,20])` yields `[(1,10), (2,20)]`.

## OCaml Approach

`List.combine: 'a list -> 'b list -> ('a * 'b) list` panics (`Invalid_argument`) on unequal lengths — it expects exact match. `List.split: ('a * 'b) list -> 'a list * 'b list` is the inverse. `List.mapi: (int -> 'a -> 'b) -> 'a list -> 'b list` provides index + element. For lazy zip: `Seq.zip: 'a Seq.t -> 'b Seq.t -> ('a * 'b) Seq.t` (since OCaml 4.14), which truncates like Rust's `.zip()`. Building a `Hashtbl` from two lists: `List.combine keys values |> List.iter (fun (k, v) -> Hashtbl.add tbl k v)`.

## Key Differences

1. **Length mismatch**: Rust `.zip()` silently truncates; OCaml `List.combine` raises `Invalid_argument` — `Seq.zip` truncates like Rust.
2. **Laziness**: Rust `.zip()` is lazy; OCaml `List.combine` is eager.
3. **unzip**: Rust `.unzip()` works on any `Iterator<Item=(A,B)>`; OCaml `List.split` only works on lists.
4. **enumerate**: Both provide `enumerate` / `mapi`; OCaml's `mapi` maps immediately, Rust's `enumerate()` is lazy.

## Exercises

1. Write `dot_product(a: &[f64], b: &[f64]) -> f64` using `.zip().map(|(x,y)| x*y).sum()`.
2. Implement `zip_with_default<T: Clone>(a: &[T], b: &[T], default: T) -> Vec<(T, T)>` that pads the shorter slice.
3. Use `zip` and `scan` together to compute the running difference between two parallel time series.
