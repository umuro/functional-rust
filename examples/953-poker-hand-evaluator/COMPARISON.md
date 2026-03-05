# OCaml vs Rust: Poker Hand Evaluator

## Side-by-Side Code

### OCaml
```ocaml
type hand_type = HighCard | Pair | TwoPair | ThreeKind | Straight
  | Flush | FullHouse | FourKind | StraightFlush

let classify (ranks : int list) (is_flush : bool) =
  let sorted = List.sort (fun a b -> compare b a) ranks in
  let counts = List.sort (fun a b -> compare b a)
    (List.map (fun r -> List.length (List.filter ((=) r) sorted))
      (List.sort_uniq compare sorted)) in
  let is_straight = match sorted with
    | [a;_;_;_;e] -> a - e = 4 && List.length (List.sort_uniq compare sorted) = 5
    | _ -> false in
  match is_flush, is_straight, counts with
  | true, true, _       -> StraightFlush
  | _, _, 4 :: _        -> FourKind
  | _, _, [3; 2]        -> FullHouse
  | true, _, _          -> Flush
  | _, true, _          -> Straight
  | _, _, 3 :: _        -> ThreeKind
  | _, _, [2; 2; 1]     -> TwoPair
  | _, _, 2 :: _        -> Pair
  | _                   -> HighCard
```

### Rust (idiomatic)
```rust
pub fn classify(ranks: &[u8], is_flush: bool) -> HandType {
    let counts: Vec<usize> = {
        let mut map = HashMap::new();
        for &r in ranks {
            *map.entry(r).or_insert(0usize) += 1;
        }
        let mut v: Vec<usize> = map.into_values().collect();
        v.sort_unstable_by(|a, b| b.cmp(a));
        v
    };
    let mut sorted = ranks.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let is_straight = sorted.len() == 5
        && counts.iter().all(|&c| c == 1)
        && (sorted[0] as i32 - sorted[4] as i32) == 4;

    match (is_flush, is_straight, counts.as_slice()) {
        (true, true, _)    => HandType::StraightFlush,
        (_, _, [4, ..])    => HandType::FourKind,
        (_, _, [3, 2])     => HandType::FullHouse,
        (true, _, _)       => HandType::Flush,
        (_, true, _)       => HandType::Straight,
        (_, _, [3, ..])    => HandType::ThreeKind,
        (_, _, [2, 2, ..]) => HandType::TwoPair,
        (_, _, [2, ..])    => HandType::Pair,
        _                  => HandType::HighCard,
    }
}
```

### Rust (functional/recursive)
```rust
fn count_occurrences(rank: u8, ranks: &[u8]) -> usize {
    ranks.iter().filter(|&&r| r == rank).count()
}

fn unique_sorted(ranks: &[u8]) -> Vec<u8> {
    let mut seen: Vec<u8> = Vec::new();
    for &r in ranks {
        if !seen.contains(&r) { seen.push(r); }
    }
    seen.sort_unstable();
    seen
}

pub fn classify_functional(ranks: &[u8], is_flush: bool) -> HandType {
    let mut sorted = ranks.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let uniq = unique_sorted(&sorted);
    let mut counts: Vec<usize> = uniq
        .iter()
        .map(|&r| count_occurrences(r, &sorted))
        .collect();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    let is_straight = sorted.len() == 5
        && uniq.len() == 5
        && (sorted[0] as i32 - sorted[4] as i32) == 4;
    // same match arm as above …
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Classify function | `val classify : int list -> bool -> hand_type` | `fn classify(ranks: &[u8], is_flush: bool) -> HandType` |
| Rank sequence | `int list` | `&[u8]` (borrowed slice) |
| Hand category | `hand_type` (variant) | `HandType` (enum) |
| Count sequence | `int list` | `Vec<usize>` / `&[usize]` |
| List pattern | `4 :: _` | `[4, ..]` |
| Exact list pattern | `[3; 2]` | `[3, 2]` |

## Key Insights

1. **Slice patterns replace list patterns 1-to-1.** OCaml's `4 :: _` becomes Rust's `[4, ..]`; OCaml's `[3; 2]` (exact two-element list) becomes Rust's `[3, 2]` (exact two-element slice). The mental model transfers directly.

2. **Tuple patterns enable multi-axis dispatch.** Both languages match a triple `(is_flush, is_straight, counts)` in one expression. Rust's exhaustiveness checker guarantees no case is missed, just like OCaml's.

3. **Enum ordering is declared, not computed.** OCaml needs a separate rank-comparison function to order hand types. In Rust, `#[derive(PartialOrd, Ord)]` gives ordering for free based on variant declaration order — HighCard first, StraightFlush last.

4. **HashMap vs List.filter/map.** OCaml computes frequencies with `List.filter ((=) r)` inside `List.map` — elegant but O(n²). Rust's idiomatic solution uses a `HashMap` for O(n) counting. The functional Rust solution mirrors OCaml's approach explicitly for pedagogical clarity.

5. **Owned vs borrowed counts.** OCaml's garbage collector manages the intermediate lists freely. Rust builds counts into an owned `Vec<usize>`, then borrows it as `&[usize]` for the match — the block-expression pattern (`let v = { … v }`) keeps the borrow lifetime clear.

## When to Use Each Style

**Use idiomatic Rust when:** You want O(n) counting via HashMap and the clearest connection between algorithm and Rust idioms.
**Use functional/recursive Rust when:** Teaching the OCaml→Rust translation — the `count_occurrences` + `unique_sorted` decomposition mirrors OCaml's `List.filter` + `List.map` pipeline step by step.
