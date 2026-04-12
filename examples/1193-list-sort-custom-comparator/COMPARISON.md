# OCaml vs Rust: List.sort — Sort with Custom Comparator

## Side-by-Side Code

### OCaml

```ocaml
let words = ["banana"; "apple"; "cherry"; "date"]

(* Lexicographic: pass String.compare as first-class function *)
let sorted = List.sort String.compare words
(* ["apple"; "banana"; "cherry"; "date"] *)

(* By length, ties broken alphabetically *)
let by_length = List.sort
  (fun a b -> compare (String.length a) (String.length b)) words
(* ["date"; "apple"; "banana"; "cherry"] *)

(* Descending: swap the arguments to reverse the order *)
let descending = List.sort (fun a b -> String.compare b a) words
(* ["date"; "cherry"; "banana"; "apple"] *)
```

### Rust (idiomatic)

```rust
pub fn sort_strings<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort();
    result
}

pub fn sort_by_length<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    // .then() chains a secondary comparator when the primary returns Equal
    result.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    result
}

pub fn sort_descending<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort_by(|a, b| b.cmp(a));
    result
}
```

### Rust (functional/recursive)

```rust
pub fn merge_sort<T: Clone + Ord>(items: &[T]) -> Vec<T> {
    if items.len() <= 1 {
        return items.to_vec();
    }
    let mid = items.len() / 2;
    let left = merge_sort(&items[..mid]);
    let right = merge_sort(&items[mid..]);
    merge(left, right)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| sort with comparator | `val sort : ('a -> 'a -> int) -> 'a list -> 'a list` | `fn sort_with<T: Clone, F: Fn(&T, &T) -> Ordering>(items: &[T], cmp: F) -> Vec<T>` |
| comparator return | `int` (negative / zero / positive) | `std::cmp::Ordering` (Less / Equal / Greater) |
| lexicographic sort | `String.compare` passed directly | `result.sort()` or `sort_by(\|a, b\| a.cmp(b))` |
| chained comparator | `if c = 0 then compare ... else c` | `key1.cmp(&key2).then(key3.cmp(&key4))` |

## Key Insights

1. **Comparator return type:** OCaml comparators return `int` (negative/zero/positive) — a convention borrowed from C's `qsort`. Rust uses `std::cmp::Ordering`, an enum with three variants. Both encode the same three-way comparison; `Ordering` is safer because the compiler prevents misinterpreting any non-zero integer as "not equal".
2. **In-place vs. persistent:** OCaml's `List.sort` always returns a new list — immutability makes this the only option. Rust's `sort_by` mutates the slice in place; to get a new sorted copy, you must clone first with `.to_vec()`. Forgetting to clone is a common mistake for OCaml developers learning Rust.
3. **Chaining comparators:** OCaml chains comparators with explicit branching: `let c = compare_key1 a b in if c <> 0 then c else compare_key2 a b`. Rust's `Ordering::then` method makes the same pattern a single expression: `key1.cmp(&key2).then(key2.cmp(&key2))`. Both are equivalent; the Rust version composes without branching.
4. **Stability guarantee:** Both `List.sort` and Rust's `sort_by` are documented as stable sorts — elements that compare equal appear in their original relative order. This is important for multi-key sorting: sort by secondary key first, then by primary key, and stability preserves the secondary ordering within groups.
5. **Performance:** OCaml's `List.sort` uses a merge sort variant. Rust's `sort_by` uses a hybrid algorithm (timsort-like) that achieves O(n log n) in all cases. Both are efficient; Rust has the additional advantage of in-place operation when you don't need a copy.

## When to Use Each Style

**Use idiomatic Rust when:** Sorting owned data in a `Vec` or borrowed slice — call `.sort_by(...)` directly for maximum efficiency with no extra allocation.
**Use `sort_with` wrapper when:** You want OCaml-style value semantics (returns a new sorted copy) and the input must remain unchanged, or when the function is part of a functional pipeline.
**Use merge sort when:** Teaching the OCaml parallel, or when you need a guaranteed-stable sort with predictable worst-case behavior and can accept the extra allocation.
