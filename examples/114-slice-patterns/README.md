📖 **[View on hightechmind.io →](https://hightechmind.io/rust/114-slice-patterns)**

---

# 114-slice-patterns — Slice Patterns
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Pattern matching on sequences is the heart of functional programming. OCaml's list patterns `x :: rest` enable recursive algorithms that read naturally. Rust provides analogous slice patterns — `[first, rest @ ..]`, `[a, b]`, `[head, .., tail]` — for contiguous memory. Unlike OCaml's linked lists, Rust's slice patterns operate on contiguous arrays with O(1) indexed access.

Slice patterns enable writing recursive algorithms in a functional style that the compiler verifies for exhaustiveness, bringing OCaml-style list processing to Rust's arrays and slices.

## Learning Outcomes

- Use `[head, rest @ ..]` to deconstruct a slice like OCaml's `x :: rest`
- Match fixed-length slices with `[a, b]`, `[a, b, c]`
- Use `[first, .., last]` to bind first and last without naming the middle
- Write recursive functions using slice patterns
- Understand exhaustiveness checking for slice patterns

## Rust Application

`src/lib.rs` demonstrates `sum` using `[x, rest @ ..]` recursion (mirroring OCaml), `take` using `(n, slice)` tuple matching, `describe` for shape classification (`[]`, `[_]`, `[_, _]`, `_`), and `rotate` using fixed patterns. The `rest @ ..` binding captures the remaining slice as a reference, not a copy — an important efficiency compared to OCaml's list tail sharing.

Slice patterns are used in parser combinator implementations, serialization codecs, and anywhere that structural decomposition of arrays is cleaner than index arithmetic.

## OCaml Approach

OCaml's list patterns are the original inspiration:

```ocaml
let rec sum = function
  | [] -> 0
  | x :: rest -> x + sum rest

let describe = function
  | [] -> "empty"
  | [_] -> "singleton"
  | [_; _] -> "pair"
  | _ -> "many"
```

OCaml's list patterns operate on linked lists; Rust's slice patterns operate on contiguous memory. OCaml 4.12+ also supports array patterns with `[| a; b |]` syntax.

## Key Differences

1. **Contiguous vs linked**: Rust's `[head, rest @ ..]` binds a contiguous subslice (a reference, zero cost); OCaml's `x :: rest` binds the next node in a linked list.
2. **Rest binding**: Rust's `rest @ ..` is a slice reference — no copying; OCaml's `rest` in `x :: rest` is the tail list (shared immutable structure).
3. **Exhaustiveness**: Both compilers check exhaustiveness for slice/list patterns; Rust reports missing cases for fixed-length patterns.
4. **OCaml array patterns**: `[| a; b; c |]` matches OCaml arrays; Rust's `[a, b, c]` matches Rust slices of length 3 — semantically equivalent.

## Exercises

1. Write `merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32>` using slice patterns to implement the merge step of merge sort.
2. Implement `run_length_encode(s: &[i32]) -> Vec<(i32, usize)>` using slice patterns to detect runs of equal elements.
3. Write a simple expression parser using slice patterns to match token sequences like `[Num(n), Plus, Num(m)]`.
