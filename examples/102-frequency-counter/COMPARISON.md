# Comparison: Frequency Counter — OCaml vs Rust

## Core Insight

OCaml's `Map` is immutable — `SMap.add` returns a new map, leaving the original unchanged. Rust's `HashMap` is mutable, and the `entry` API is its killer feature: `freq.entry(w).or_insert(0) += 1` does a single lookup for both read and write, whereas OCaml's `find` + `add` does two tree traversals.

## OCaml

```ocaml
module SMap = Map.Make(String)
let word_freq text =
  text |> String.split_on_char ' '
  |> List.fold_left (fun acc w ->
    let count = try SMap.find w acc with Not_found -> 0 in
    SMap.add w (count + 1) acc
  ) SMap.empty
```

## Rust

```rust
pub fn word_freq(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    freq
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Map type | `Map.Make(String)` (tree) | `HashMap` (hash) / `BTreeMap` (tree) |
| Mutability | Immutable (new map each add) | Mutable in-place |
| Lookup+update | `find` + `add` (2 traversals) | `entry().or_insert()` (1 lookup) |
| Missing key | `Not_found` exception | `or_insert(default)` |
| Ordering | Sorted (tree-based) | Unordered (HashMap) / Sorted (BTreeMap) |
| Functor needed | Yes: `Map.Make(String)` | No: generics handle it |

## Learner Notes

- **Entry API**: Rust's most powerful map feature — avoids the check-then-insert anti-pattern
- **`BTreeMap`** ≈ OCaml's `Map`: both are balanced trees with O(log n) ops and sorted iteration
- **`HashMap`** is O(1) average but unordered — use `BTreeMap` when you need sorted keys
- **Exception vs default**: OCaml's `Not_found` requires try/with; Rust's `or_insert` is cleaner
- **Ownership**: `entry()` takes ownership of the key — use `String`, not `&str`
