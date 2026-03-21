**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[poker-hand-evaluator on hightechmind.io](https://hightechmind.io/posts/functional-rust/poker-hand-evaluator)

---

## Problem Statement

Classify a five-card poker hand (straight flush, four of a kind, full house, etc.) using functional techniques. Build a frequency map of rank counts, detect flush and straight conditions, then use a pattern match on the sorted count vector to classify the hand. Implement both a `HashMap`-based idiomatic version and a purely functional recursive alternative.

## Learning Outcomes

- Build a rank-frequency map with `HashMap` and extract sorted count values
- Detect a straight using `counts.iter().all(|&c| c == 1)` and a consecutive rank spread of 4
- Detect a flush by checking that all cards share the same suit
- Classify hand types by pattern-matching on `(is_flush, is_straight, counts.as_slice())`
- Implement the same classification recursively using `filter` + `count` without a `HashMap`

## Rust Application

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    HighCard, Pair, TwoPair, ThreeKind, Straight, Flush,
    FullHouse, FourKind, StraightFlush,
}

pub fn classify(ranks: &[u8], is_flush: bool) -> HandType {
    let counts: Vec<usize> = {
        let mut map = HashMap::new();
        for &r in ranks { *map.entry(r).or_insert(0usize) += 1; }
        let mut v: Vec<usize> = map.into_values().collect();
        v.sort_unstable_by(|a, b| b.cmp(a));  // descending
        v
    };

    let mut sorted = ranks.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));

    let is_straight = sorted.len() == 5
        && counts.iter().all(|&c| c == 1)
        && (sorted[0] as i32 - sorted[4] as i32) == 4;

    match (is_flush, is_straight, counts.as_slice()) {
        (true, true, _)  => HandType::StraightFlush,
        (_, _, [4, ..])  => HandType::FourKind,
        (_, _, [3, 2])   => HandType::FullHouse,
        (true, _, _)     => HandType::Flush,
        (_, true, _)     => HandType::Straight,
        (_, _, [3, ..])  => HandType::ThreeKind,
        (_, _, [2, 2, ..]) => HandType::TwoPair,
        (_, _, [2, ..])  => HandType::Pair,
        _               => HandType::HighCard,
    }
}
```

Sorting counts in descending order makes the patterns readable: `[4, ..]` matches four-of-a-kind (one rank appears 4 times), `[3, 2]` matches full house (3 + 2). Rust's slice pattern matching with `..` for "rest" is clean and exhaustive.

`HandType` derives `Ord` in declaration order — `StraightFlush` > `FourKind` > ... > `HighCard` — enabling direct comparison of hand strengths.

## OCaml Approach

```ocaml
type hand_type =
  | HighCard | Pair | TwoPair | ThreeKind | Straight
  | Flush | FullHouse | FourKind | StraightFlush

let classify ranks is_flush =
  let counts =
    List.sort_uniq compare ranks
    |> List.map (fun r -> List.length (List.filter ((=) r) ranks))
    |> List.sort (fun a b -> compare b a)  (* descending *)
  in
  let sorted = List.sort (fun a b -> compare b a) ranks in
  let is_straight =
    List.for_all (fun c -> c = 1) counts &&
    (List.hd sorted - List.nth sorted 4) = 4
  in
  match is_flush, is_straight, counts with
  | true, true, _ -> StraightFlush
  | _, _, [4; _]  -> FourKind
  | _, _, [3; 2]  -> FullHouse
  | true, _, _    -> Flush
  | _, true, _    -> Straight
  | _, _, [3; _]  -> ThreeKind
  | _, _, [2; 2; _] -> TwoPair
  | _, _, [2; _]  -> Pair
  | _             -> HighCard
```

OCaml's pattern match on `(is_flush, is_straight, counts)` is structurally identical to the Rust version. The main difference is that OCaml's ADT constructors are ordered by declaration for comparison, just like Rust's `#[derive(Ord)]`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Frequency map | `HashMap` with `entry().or_insert(0)` | `List.filter` + `List.length` per rank |
| Slice patterns | `[4, ..]`, `[3, 2]`, etc. | `[4; _]`, `[3; 2]`, etc. |
| Enum ordering | `#[derive(PartialOrd, Ord)]` by declaration order | ADT comparison by declaration order |
| Descending sort | `sort_unstable_by(\|a, b\| b.cmp(a))` | `List.sort (fun a b -> compare b a)` |

The pattern-match-on-counts approach is elegant and easily extended. Adding new hand variants (e.g., five-of-a-kind for wild cards) requires only a new enum variant and a new match arm.

## Exercises

1. Add ace-low straight detection (A-2-3-4-5) where ace can count as 1.
2. Implement hand comparison: given two hands, determine which beats the other (handle ties with high-card comparison).
3. Extend to parse hand strings like `"AS KH QD JC 10S"` into `(ranks, suits)` before calling `classify`.
4. Implement a `best_hand(hands: &[&str])` function that returns the index of the winning hand.
5. Add joker support: one wild card can substitute for any rank to maximize hand type.
