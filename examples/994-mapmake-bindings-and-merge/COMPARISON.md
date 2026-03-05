# OCaml vs Rust: Map.Make — Bindings and Merge

## Side-by-Side Code

### OCaml
```ocaml
module SMap = Map.Make(String)

let m1 = SMap.of_list [("a", 1); ("b", 2); ("c", 3)]
let m2 = SMap.of_list [("b", 20); ("c", 30); ("d", 40)]

let merged = SMap.union (fun _k v1 v2 -> Some (v1 + v2)) m1 m2

let pairs = SMap.bindings merged
let () = List.iter (fun (k,v) -> Printf.printf "%s: %d\n" k v) pairs
```

### Rust (idiomatic — entry API)
```rust
use std::collections::BTreeMap;

pub fn map_union_sum(
    m1: &BTreeMap<String, i64>,
    m2: &BTreeMap<String, i64>,
) -> BTreeMap<String, i64> {
    let mut result = m1.clone();
    for (k, v) in m2 {
        result
            .entry(k.clone())
            .and_modify(|existing| *existing += v)
            .or_insert(*v);
    }
    result
}
```

### Rust (functional — iterator fold)
```rust
pub fn map_union_sum_fold(
    m1: &BTreeMap<String, i64>,
    m2: &BTreeMap<String, i64>,
) -> BTreeMap<String, i64> {
    m2.iter().fold(m1.clone(), |mut acc, (k, v)| {
        acc.entry(k.clone())
            .and_modify(|existing| *existing += v)
            .or_insert(*v);
        acc
    })
}
```

### Rust (generic — mirrors OCaml's Map.union callback)
```rust
pub fn map_union_with<F>(
    m1: &BTreeMap<String, i64>,
    m2: &BTreeMap<String, i64>,
    resolve: F,
) -> BTreeMap<String, i64>
where
    F: Fn(&str, i64, i64) -> Option<i64>,
{
    let mut result = m1.clone();
    for (k, v2) in m2 {
        match result.get(k) {
            Some(&v1) => match resolve(k, v1, *v2) {
                Some(merged) => { result.insert(k.clone(), merged); }
                None         => { result.remove(k); }
            },
            None => { result.insert(k.clone(), *v2); }
        }
    }
    result
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Map type | `SMap.t` (`Map.Make(String).t`) | `BTreeMap<String, i64>` |
| Bindings | `val bindings : 'a t -> (key * 'a) list` | `.iter()` → sorted `(&String, &i64)` |
| Union with callback | `val union : (key -> 'a -> 'a -> 'a option) -> 'a t -> 'a t -> 'a t` | `fn map_union_with<F: Fn(&str, i64, i64) -> Option<i64>>` |
| Entry upsert | `Map.update k (function \| Some v -> Some (f v) \| None -> Some d)` | `.entry(k).and_modify(f).or_insert(d)` |

## Key Insights

1. **`BTreeMap` = `Map.Make(String)`**: Both maintain keys in sorted order and produce sorted iteration — `BTreeMap::iter()` is a direct analog to `SMap.bindings`.
2. **Persistence vs mutation**: OCaml maps are structurally shared and immutable; Rust's `BTreeMap` is mutable. We clone to simulate non-destructive merge. The clone is explicit and justified, unlike hidden allocation in GC languages.
3. **Entry API eliminates double-lookup**: OCaml's `find_opt` + `add` requires two tree traversals. Rust's `entry` API navigates the tree once and either modifies in place or inserts — a zero-waste ergonomic pattern.
4. **Callback returns `Option` → key removal**: OCaml's `Map.union` callback returning `None` removes the key. Rust's `map_union_with` replicates this: `resolve` returning `None` calls `result.remove(k)` — same semantics, same expressiveness.
5. **Fold as functional accumulation**: The `fold` version makes the "start from m1, consume m2 entries one by one" structure explicit — closer to how a functional programmer thinks about the merge.

## When to Use Each Style

**Use `map_union_sum` (entry loop) when:** you need clarity and performance — single traversal, minimal allocations, imperative but readable.

**Use `map_union_sum_fold` (fold) when:** composing merge with other iterator operations or passing the merge step as a value — the fold-based form is more composable.

**Use `map_union_with` (generic callback) when:** the conflict-resolution policy varies by call site — mirrors OCaml's `Map.union` exactly and lets callers inject sum, keep-left, keep-max, or removal logic without rewriting the merge loop.
