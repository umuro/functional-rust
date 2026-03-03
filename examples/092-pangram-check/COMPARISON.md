# Comparison: Pangram Check — OCaml vs Rust

## Core Insight

OCaml models this with its functor system (`Set.Make(Char)`) — you create a specialized set module for characters. Rust's generics mean `HashSet<char>` just works. The deeper lesson is about abstraction cost: OCaml's functors are more explicit and modular, while Rust's trait-based generics are more ergonomic for common cases.

## OCaml

```ocaml
module CS = Set.Make(Char)
let alphabet = List.init 26 (fun i -> Char.chr (i + Char.code 'a')) |> CS.of_list
let is_pangram s =
  let chars = s |> String.lowercase_ascii |> String.to_seq
    |> Seq.filter (fun c -> c >= 'a' && c <= 'z') |> CS.of_seq in
  CS.subset alphabet chars
```

## Rust — HashSet

```rust
pub fn is_pangram_hashset(s: &str) -> bool {
    let chars: HashSet<char> = s.chars()
        .filter_map(|c| { let lc = c.to_ascii_lowercase(); lc.is_ascii_lowercase().then_some(lc) })
        .collect();
    chars.len() == 26
}
```

## Rust — Bitset

```rust
pub fn is_pangram_bitset(s: &str) -> bool {
    let mut bits: u32 = 0;
    for c in s.chars() {
        let lc = c.to_ascii_lowercase();
        if lc.is_ascii_lowercase() { bits |= 1 << (lc as u32 - 'a' as u32); }
    }
    bits == (1 << 26) - 1
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Set creation | `Set.Make(Char)` functor | `HashSet<char>` directly |
| Subset check | `CS.subset alphabet chars` | `chars.len() == 26` |
| Lowercase | `String.lowercase_ascii` | `.to_ascii_lowercase()` |
| Filtering | `Seq.filter` | `.filter_map()` |
| Bitset approach | Manual with `lor`/`lsl` | Same with `\|=`/`<<` |
| Performance | O(n log n) with balanced tree set | O(n) with bitset |

## Learner Notes

- **Functor vs Generics**: OCaml's `Set.Make` creates a module; Rust's `HashSet` uses trait bounds — different abstraction styles
- **Bitset trick**: Both languages support it; Rust's `u32` bit ops are identical to OCaml's `lor`/`lsl`
- **Early exit**: The bitset approach can short-circuit once `bits == ALL` — neither the OCaml nor basic Rust version does this
- **`filter_map`**: Rust's combined filter+map avoids an intermediate allocation that separate filter + map would need
