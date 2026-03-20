📖 **[View on hightechmind.io →](https://hightechmind.io/rust/055-list-filter-from-scratch)**

---

# Example 055: List Filter from Scratch

**Difficulty:** ⭐
**Category:** Higher-Order Functions
**OCaml Source:** CS3110

## Problem Statement

Derive `List.filter` from scratch using a predicate function. Filter must preserve
the relative order of all surviving elements.

## Learning Outcomes

- Higher-order functions: passing predicates as first-class values
- Pattern-match recursion vs iterator adaptor vs fold — three idiomatic styles
- Why a recursive generic function in Rust needs `&dyn Fn` to avoid monomorphization explosion
- Order preservation: `filter` is stable — elements that pass appear in their original order
- Partial application in OCaml (`let evens = filter (fun n -> ...)`) vs Rust closures

## OCaml Approach

A single recursive function `filter p lst` pattern-matches on the list head and tail.
Partial application (`let evens = filter (fun n -> n mod 2 = 0)`) produces a
specialized function from the generic one.

## Rust Approach

1. **Iterator** (`filter`): `list.iter().filter(|x| p(x)).cloned().collect()` — the natural
   idiomatic form; mirrors what `Iterator::filter` does internally.
2. **Recursive** (`filter_rec`): pattern-matches on `[head, tail @ ..]` exactly like OCaml;
   uses an inner `fn go(..., p: &dyn Fn)` helper to avoid infinite monomorphization.
3. **Fold** (`filter_fold`): builds result via `fold`, appending elements that pass `p`.

## Key Differences

1. **Partial application**: OCaml `let evens = filter f` naturally curries; Rust wraps in a closure: `|nums| filter(nums, |n| n % 2 == 0)`.
2. **Recursion and types**: Passing `&p` in a recursive generic function wraps the closure type in references infinitely — Rust requires a `&dyn Fn` break point.
3. **Slice patterns**: `[head, tail @ ..]` in Rust is the exact structural match of OCaml's `h :: t` on lists.
4. **Iterator adaptor**: Rust's `Iterator::filter` IS this function — studying the from-scratch version reveals what the standard library does.
5. **Order preservation**: Both languages guarantee left-to-right traversal; the recursive form builds in reverse and corrects by prepending, then the fold/iterator traverse naturally.

## Exercises

1. Implement `filter_not_none` from scratch: filter a `Vec<Option<T>>` to keep only `Some` values and unwrap them into a `Vec<T>`.
2. Write `filter_with_count` that filters a list and also returns the number of elements that were removed.
3. Implement `stable_partition` from scratch that separates elements into two groups (satisfying and not satisfying a predicate) while preserving original relative order in both groups.
