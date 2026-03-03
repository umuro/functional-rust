# Comparison: Isogram Check — OCaml vs Rust

## Core Insight

OCaml's `List.sort_uniq` elegantly combines sorting and deduplication. Rust separates these operations but offers a more powerful alternative: `HashSet::insert` returns a boolean indicating whether the element was new, allowing early termination on the first duplicate — something OCaml's approach cannot do.

## OCaml

```ocaml
let is_isogram s =
  let chars = s |> String.lowercase_ascii |> String.to_seq
    |> Seq.filter (fun c -> c >= 'a' && c <= 'z') |> List.of_seq in
  let unique = List.sort_uniq Char.compare chars in
  List.length chars = List.length unique
```

## Rust — HashSet with early exit

```rust
pub fn is_isogram_hashset(s: &str) -> bool {
    let mut seen = HashSet::new();
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| seen.insert(c.to_ascii_lowercase()))
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Dedup | `List.sort_uniq` | `sort_unstable()` + `dedup()` |
| Early exit | No (processes all) | `HashSet::insert` + `all()` |
| Set approach | Would need `Set.Make(Char)` | `HashSet<char>` built-in |
| Bitset | `lor`/`lsl` | `\|=`/`<<` |
| Complexity | O(n log n) | O(n) with HashSet/bitset |

## Learner Notes

- **`HashSet::insert` idiom**: Returning bool on insert is uniquely useful — OCaml sets don't offer this
- **`all()` short-circuits**: Stops at first `false`, making this O(k) where k is first duplicate position
- **Bitset is fastest**: For ASCII-only, 26 bits in a `u32` beats any collection
- **`sort_unstable`**: Rust's unstable sort doesn't preserve order of equal elements but is faster — fine here since we just dedup
