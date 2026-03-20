📖 **[View on hightechmind.io →](https://hightechmind.io/rust/280-iterator-any-all)**

---

# 280: Existential Checks with any() and all()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Testing whether at least one element satisfies a condition (existential quantification: ∃) or whether all elements satisfy a condition (universal quantification: ∀) are the two most fundamental logical queries on collections. These appear constantly in validation: "does any user have admin rights?", "are all inputs valid?", "does any file exist?". Rust's `any()` and `all()` short-circuit — stopping as soon as the answer is determined — making them efficient even on very large collections.

## Learning Outcomes

- Understand `any(pred)` as ∃ (existential) and `all(pred)` as ∀ (universal) quantification
- Recognize that both short-circuit: `any()` stops at first `true`, `all()` stops at first `false`
- Know the vacuous truth: `all()` returns `true` and `any()` returns `false` for empty iterators
- Combine with `filter()` to test subsets: "all evens satisfy X"

## Rust Application

`any()` and `all()` take a `FnMut(&Item) -> bool` and return `bool`. They short-circuit evaluation:

```rust
assert!([1i32, 2, 3].iter().any(|&x| x == 2));  // true, stops at 2
assert!(![1i32, 2, 3].iter().any(|&x| x == 9)); // false, checks all
assert!([2i32, 4, 6].iter().all(|&x| x % 2 == 0)); // true
assert!(![1i32, 2, 3].iter().all(|&x| x % 2 == 0)); // false, stops at 1

// Vacuous truth/falsity for empty
let empty: Vec<i32> = vec![];
assert!(empty.iter().all(|_| false)); // vacuously true
assert!(!empty.iter().any(|_| true)); // false — no elements
```

## OCaml Approach

OCaml provides `List.exists` (equivalent to `any`) and `List.for_all` (equivalent to `all`), both short-circuiting:

```ocaml
let has_even = List.exists (fun x -> x mod 2 = 0) [1;3;4;5]  (* true *)
let all_pos = List.for_all (fun x -> x > 0) [1;2;3]           (* true *)
```

The naming differs: `exists` vs `any`, `for_all` vs `all` — but the semantics are identical.

## Key Differences

1. **Naming**: Rust uses `any`/`all`; OCaml uses `exists`/`for_all` — same semantics, different names.
2. **Short-circuiting**: Both languages guarantee short-circuit evaluation — they stop early when the answer is known.
3. **Logical laws**: De Morgan's laws hold: `!any(p)` == `all(!p)` and `!all(p)` == `any(!p)`.
4. **Mutation in closures**: Rust's `FnMut` allows side effects in the predicate (e.g., counting calls), while OCaml closures capture refs.

## Exercises

1. Use `all()` to validate that all elements in a `Vec<String>` are non-empty and contain only ASCII characters.
2. Implement a `none(pred)` function using `any()` and negation, then verify it satisfies `none(p) == !any(p)`.
3. Use `any()` to short-circuit an expensive check: given a list of file paths, return true if any file has size > 1MB (simulate with `len()`).
