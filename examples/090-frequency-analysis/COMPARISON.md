# OCaml vs Rust: Frequency Analysis — Letter Distribution

## Side-by-Side Code

### OCaml
```ocaml
module CMap = Map.Make(Char)

let frequency s =
  String.fold_left (fun m c ->
    let c = Char.lowercase_ascii c in
    if c >= 'a' && c <= 'z' then
      CMap.update c (function None -> Some 1 | Some n -> Some (n+1)) m
    else m
  ) CMap.empty s

let sorted_freq s =
  frequency s |> CMap.bindings
  |> List.sort (fun (_, a) (_, b) -> compare b a)
```

### Rust (idiomatic — HashMap)
```rust
use std::collections::HashMap;

pub fn frequency(s: &str) -> HashMap<char, usize> {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}

pub fn sorted_freq(s: &str) -> Vec<(char, usize)> {
    let mut pairs: Vec<(char, usize)> = frequency(s).into_iter().collect();
    pairs.sort_by(|(c1, n1), (c2, n2)| n2.cmp(n1).then(c1.cmp(c2)));
    pairs
}
```

### Rust (BTreeMap — mirrors OCaml's Map.Make, keys sorted)
```rust
use std::collections::BTreeMap;

pub fn frequency_btree(s: &str) -> BTreeMap<char, usize> {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold(BTreeMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Frequency map type | `CMap.t` (≈ `Char -> int`) | `HashMap<char, usize>` or `BTreeMap<char, usize>` |
| Function signature | `val frequency : string -> CMap.t` | `fn frequency(s: &str) -> HashMap<char, usize>` |
| Sorted result | `(char * int) list` | `Vec<(char, usize)>` |
| Map entry update | `CMap.update c f m` | `*map.entry(c).or_insert(0) += 1` |
| String fold | `String.fold_left f init s` | `s.chars().fold(init, f)` |
| Bindings extraction | `CMap.bindings m` | `map.into_iter().collect()` |

## Key Insights

1. **`BTreeMap` is the faithful OCaml equivalent.** OCaml's `Map.Make(Char)` is a balanced BST with `O(log n)` operations and keys always in sorted order. Rust's `BTreeMap<char, usize>` is structurally identical. `HashMap` is faster on average but unordered — a deliberate trade-off.

2. **`entry().or_insert()` beats OCaml's `update`.** OCaml requires a function `option -> option` to express "insert 1 or increment". Rust's entry API returns a `&mut usize`, enabling `+= 1` directly — no pattern match needed, zero allocation, and no intermediate closure.

3. **Iterator chains compose like OCaml pipelines.** OCaml's `String.fold_left (fun m c -> ...) CMap.empty s` threads state explicitly. Rust's `.chars().filter().map().fold()` is the same pipeline with typed stages. Both are lazy in spirit; Rust's iterators are actually zero-cost lazy.

4. **Case normalization is symmetric.** OCaml uses `Char.lowercase_ascii`; Rust uses `.to_ascii_lowercase()`. Both operate on ASCII only — correct for letter frequency on ASCII text and cheaper than Unicode case folding.

5. **Sorting requires a tiebreaker for determinism.** OCaml's `List.sort (fun (_, a) (_, b) -> compare b a)` only compares counts, leaving ties non-deterministic (sort is not stable in all implementations). Rust's `.sort_by(|(c1,n1),(c2,n2)| n2.cmp(n1).then(c1.cmp(c2)))` adds alphabetical tiebreaking, producing a fully deterministic result — important for testing and reproducibility.

## When to Use Each Style

**Use `HashMap` (idiomatic Rust) when:** you need maximum throughput and don't care about key order — e.g., building frequency tables for large corpora where you'll sort the results anyway.

**Use `BTreeMap` (OCaml-equivalent) when:** you need keys in sorted order without a separate sort step, or when you want structural parity with OCaml code for comparison/porting purposes.
