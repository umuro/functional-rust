# OCaml vs Rust: Set.Make — Set Operations for Data Processing

## Side-by-Side Code

### OCaml

```ocaml
module StringSet = Set.Make(String)

let words = ["the"; "cat"; "sat"; "on"; "the"; "mat"; "the"; "cat"]
let unique = StringSet.of_list words
let () = Printf.printf "Unique words: %d\n" (StringSet.cardinal unique)

let stopwords = StringSet.of_list ["the"; "on"; "a"; "an"]
let content_words = StringSet.diff unique stopwords
let () = StringSet.iter (fun w -> Printf.printf "%s " w) content_words
```

### Rust (idiomatic)

```rust
use std::collections::HashSet;

fn unique_words<'a>(words: &[&'a str]) -> HashSet<&'a str> {
    words.iter().copied().collect()
}

fn remove_stopwords<'a>(
    words: &HashSet<&'a str>,
    stopwords: &HashSet<&'a str>,
) -> HashSet<&'a str> {
    words.difference(stopwords).copied().collect()
}
```

### Rust (BTreeSet — sorted output, closer to OCaml semantics)

```rust
use std::collections::BTreeSet;

fn unique_words_sorted<'a>(words: &[&'a str]) -> BTreeSet<&'a str> {
    words.iter().copied().collect()
}

fn remove_stopwords_sorted<'a>(
    words: &BTreeSet<&'a str>,
    stopwords: &BTreeSet<&'a str>,
) -> BTreeSet<&'a str> {
    words.difference(stopwords).copied().collect()
}
```

## Type Signatures

| Concept | OCaml | Rust (HashSet) |
|---------|-------|----------------|
| Set type | `StringSet.t` (abstract, ordered) | `HashSet<&str>` (unordered) |
| Sorted set | `StringSet.t` (always sorted) | `BTreeSet<&str>` |
| Construction | `StringSet.of_list : string list -> t` | `words.iter().copied().collect::<HashSet<_>>()` |
| Cardinality | `StringSet.cardinal : t -> int` | `.len()` |
| Membership | `StringSet.mem : string -> t -> bool` | `.contains(&word)` |
| Difference | `StringSet.diff : t -> t -> t` | `.difference(other).copied().collect()` |
| Union | `StringSet.union : t -> t -> t` | `.union(other).copied().collect()` |
| Intersection | `StringSet.inter : t -> t -> t` | `.intersection(other).copied().collect()` |
| Iteration | `StringSet.iter : (string -> unit) -> t -> unit` | `.iter()` |

## Key Insights

1. **No functor needed in Rust:** OCaml's `Set.Make` is a functor that takes a module with a comparison function and produces a fully-typed set module. Rust's `HashSet<T>` is generic from day one — the type parameter carries all the information needed.

2. **Ordered vs unordered:** OCaml's `Set` guarantees lexicographic order for iteration (it's a balanced BST). Rust's `HashSet` makes no ordering guarantee. Use `BTreeSet` when sorted output matters — it has the same API but is backed by a B-tree.

3. **Lazy iterators for set algebra:** OCaml's `diff`, `union`, `inter` return new set values immediately. Rust's `.difference()`, `.union()`, `.intersection()` return lazy iterators — you must `.collect()` to materialise the result. This lets you compose operations without allocating intermediate sets.

4. **Deduplication idiom:** OCaml uses `of_list` explicitly. In Rust, collecting any iterator (including one with duplicates) into a `HashSet` is the idiomatic deduplication pattern: `words.iter().copied().collect::<HashSet<_>>()`.

5. **Lifetime propagation:** When storing `&str` references inside a `HashSet`, Rust requires explicit lifetime annotations on functions that return the set, ensuring the referenced strings outlive the set. OCaml owns its strings and has no equivalent concern.

## When to Use Each Style

**Use `HashSet` when:** order of results does not matter and you need O(1) average-case membership testing and set operations — the common case for deduplication and filtering pipelines.

**Use `BTreeSet` when:** you need deterministic, sorted iteration — analogous to OCaml's `Set.Make` which always iterates in ascending order. Useful for producing reproducible output or sorted reports.
