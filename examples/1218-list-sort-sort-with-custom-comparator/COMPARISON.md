# OCaml vs Rust: List Sort (with custom comparator)

## Side-by-Side Code

### OCaml
```ocaml
let words = ["banana"; "apple"; "cherry"; "date"]

(* Alphabetical — String.compare is already a three-way comparator *)
let by_alphabet = List.sort String.compare words

(* Custom comparator: compare by string length *)
let by_length =
  List.sort (fun a b -> compare (String.length a) (String.length b)) words

(* Descending — reverse the arguments to the comparator *)
let by_alpha_desc = List.sort (fun a b -> String.compare b a) words
```

### Rust (idiomatic)
```rust
use std::cmp::Ordering;

pub fn sort_by_comparator<T, F>(items: &[T], cmp: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    let mut out = items.to_vec();
    out.sort_by(cmp);
    out
}

pub fn sort_alphabetical(words: &[String]) -> Vec<String> {
    sort_by_comparator(words, |a, b| a.cmp(b))
}
```

### Rust (functional / key-based)
```rust
pub fn sort_by_key_fn<T, K, F>(items: &[T], key: F) -> Vec<T>
where
    T: Clone,
    K: Ord,
    F: FnMut(&T) -> K,
{
    let mut out = items.to_vec();
    out.sort_by_key(key);
    out
}

pub fn sort_by_length(words: &[String]) -> Vec<String> {
    sort_by_key_fn(words, |s| s.len())
}
```

## Type Signatures

| Concept                  | OCaml                                              | Rust                                                               |
|--------------------------|----------------------------------------------------|--------------------------------------------------------------------|
| `List.sort` signature    | `val sort : ('a -> 'a -> int) -> 'a list -> 'a list` | `fn sort_by<F: FnMut(&T, &T) -> Ordering>(&mut [T], F)`            |
| Comparator result        | `int` (sign: `<0`, `0`, `>0`)                      | `std::cmp::Ordering` (`Less` / `Equal` / `Greater`)                |
| Key-based shortcut       | — (inline in comparator)                           | `fn sort_by_key<K: Ord, F: FnMut(&T) -> K>(&mut [T], F)`           |
| Immutable flavour        | built-in — returns a new list                      | `items.to_vec()` then `.sort_by(...)` — one clone per call         |
| Descending order         | `fun a b -> cmp b a`                               | `|a, b| cmp(a, b).reverse()`                                       |

## Key Insights

1. **`List.sort` ↔ `slice::sort_by`** — same `O(n log n)` stable merge-sort (OCaml stdlib) / Timsort-style pattern-defeating sort (Rust stdlib). The shapes are identical apart from the comparator return type.
2. **`int` comparator vs `Ordering` enum** — OCaml uses `compare` returning an `int` whose sign matters. Rust's `Ordering` is a three-variant enum that forbids the classic bug of `a - b` overflowing for large `i32` values. `a.cmp(&b)` is the safe equivalent.
3. **In-place by default** — Rust's slice methods mutate, so "returning a new list" is opt-in: `let mut v = items.to_vec(); v.sort_by(...); v`. OCaml's `List.sort` is inherently pure because the list cells are immutable.
4. **`sort_by_key` has no OCaml analogue** — in OCaml you re-extract the key inside every comparator call. Rust exposes `sort_by_key` (simple) and `sort_by_cached_key` (caches expensive keys), saving redundant work.
5. **Stability matters, and is named** — both stdlibs default to stable. Rust is explicit: `sort_by_unstable_by` is a separate method you pick for speed. OCaml only ships the stable variant.

## When to Use Each Style

**Use idiomatic Rust (`sort_by_key`) when:** the comparator is "compare some extracted value" — by length, by a struct field, by `key.to_lowercase()`. It is both shorter and faster because each key is computed once.

**Use `sort_by` when:** the comparator genuinely needs both elements — multi-level sorts (`primary.then_with(|| secondary)`), or domain-specific orderings that cannot be reduced to a single `Ord` key.
