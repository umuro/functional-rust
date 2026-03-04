# OCaml vs Rust: Yacht Dice Scoring

## Side-by-Side Code

### OCaml

```ocaml
type category = Ones | Twos | Threes | Fours | Fives | Sixes
  | FullHouse | FourOfAKind | LittleStraight | BigStraight | Yacht | Choice

let count dice n = List.length (List.filter ((=) n) dice)

let score dice = function
  | Ones   -> count dice 1 | Twos   -> 2 * count dice 2
  | Threes -> 3 * count dice 3 | Fours  -> 4 * count dice 4
  | Fives  -> 5 * count dice 5 | Sixes  -> 6 * count dice 6
  | Choice -> List.fold_left (+) 0 dice
  | Yacht  -> if List.for_all ((=) (List.hd dice)) dice then 50 else 0
  | FullHouse ->
    let sorted = List.sort compare dice in
    (match sorted with
     | [a;b;c;d;e] when a=b && b=c && d=e && c<>d -> List.fold_left (+) 0 dice
     | [a;b;c;d;e] when a=b && c=d && d=e && b<>c -> List.fold_left (+) 0 dice
     | _ -> 0)
  | FourOfAKind ->
    (try
       let v = List.find (fun n -> count dice n >= 4)
                         (List.sort_uniq compare dice) in
       4 * v
     with Not_found -> 0)
  | LittleStraight ->
    if List.sort compare dice = [1;2;3;4;5] then 30 else 0
  | BigStraight ->
    if List.sort compare dice = [2;3;4;5;6] then 30 else 0
```

### Rust (idiomatic)

```rust
pub fn score(dice: &[u8; 5], category: Category) -> u32 {
    match category {
        Category::Ones   => u32::from(count(dice, 1)),
        Category::Twos   => 2 * u32::from(count(dice, 2)),
        Category::Threes => 3 * u32::from(count(dice, 3)),
        Category::Fours  => 4 * u32::from(count(dice, 4)),
        Category::Fives  => 5 * u32::from(count(dice, 5)),
        Category::Sixes  => 6 * u32::from(count(dice, 6)),
        Category::Choice => dice.iter().map(|&d| u32::from(d)).sum(),
        Category::Yacht  => {
            if dice.iter().all(|&d| d == dice[0]) { 50 } else { 0 }
        }
        Category::FullHouse => {
            let mut counts = [0u8; 7];
            for &d in dice { counts[d as usize] += 1; }
            let mut freqs: Vec<u8> = counts.iter().copied()
                .filter(|&c| c > 0).collect();
            freqs.sort_unstable();
            if freqs == [2, 3] { dice.iter().map(|&d| u32::from(d)).sum() }
            else { 0 }
        }
        Category::FourOfAKind => (1u8..=6)
            .find(|&n| count(dice, n) >= 4)
            .map(|n| 4 * u32::from(n))
            .unwrap_or(0),
        Category::LittleStraight => {
            let mut s = *dice; s.sort_unstable();
            if s == [1, 2, 3, 4, 5] { 30 } else { 0 }
        }
        Category::BigStraight => {
            let mut s = *dice; s.sort_unstable();
            if s == [2, 3, 4, 5, 6] { 30 } else { 0 }
        }
    }
}
```

### Rust (functional/recursive — FourOfAKind)

```rust
pub fn score_four_of_a_kind_recursive(dice: &[u8; 5], face: u8) -> u32 {
    if face > 6 { return 0; }
    if count(dice, face) >= 4 {
        4 * u32::from(face)
    } else {
        score_four_of_a_kind_recursive(dice, face + 1)
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Category type | `type category = Ones \| Twos \| ...` | `pub enum Category { Ones, Twos, ... }` |
| Dice parameter | `'a list` (unbounded) | `&[u8; 5]` (exactly 5 dice) |
| Score result | `int` | `u32` |
| Count helper | `dice -> int -> int` | `fn count(dice: &[u8], n: u8) -> u8` |
| Missing value | `Not_found` exception | `Option<T>` → `.unwrap_or(0)` |

## Key Insights

1. **Enums are the same concept:** OCaml variant types and Rust enums are both sum types encoding a closed set of alternatives. The translation is nearly mechanical — rename and adjust syntax.

2. **Fixed-length arrays enforce invariants:** OCaml represents five dice as `'a list`; Rust uses `[u8; 5]`. The array type encodes "exactly five" at compile time, eliminating a class of runtime errors with no overhead.

3. **Options replace exceptions:** OCaml's `List.find` raises `Not_found` when nothing matches, caught with `try/with`. Rust's `Iterator::find` returns `Option<T>`. The `.find().map(f).unwrap_or(default)` chain is safer and composes without stack-unwinding overhead.

4. **Frequency table beats sorted-pattern matching:** OCaml's FullHouse implementation matches on two sorted list patterns with guards — readable but fragile and clippy-hostile when ported to Rust. Building a frequency table (`counts[face] += 1`) and comparing sorted frequency counts to `[2, 3]` is cleaner, handles all orderings, and passes clippy without special cases.

5. **`sort_unstable` vs `List.sort`:** Rust's `sort_unstable` on a `[u8; 5]` stack array is zero-allocation and O(n log n). OCaml's `List.sort` allocates a sorted list. For five elements neither matters practically, but the Rust version has no heap traffic at all.

## When to Use Each Style

**Use idiomatic Rust when:** Scoring real game logic — the frequency-table approach for FullHouse and the iterator chain for FourOfAKind are easier to maintain and extend (e.g., adding validation).

**Use recursive Rust when:** Teaching the OCaml-to-Rust mental model — the recursive `score_four_of_a_kind_recursive` mirrors the OCaml `List.find` tail-recursion pattern explicitly and shows how recursion replaces looping over a range.
