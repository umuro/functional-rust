# OCaml vs Rust: String Anagram Check

## Side-by-Side Code

### OCaml
```ocaml
let to_sorted_chars s =
  s |> String.lowercase_ascii
    |> String.to_seq |> List.of_seq
    |> List.sort Char.compare

let is_anagram s1 s2 =
  let s1' = String.lowercase_ascii s1 in
  let s2' = String.lowercase_ascii s2 in
  s1' <> s2' && to_sorted_chars s1 = to_sorted_chars s2

let find_anagrams word candidates =
  List.filter (is_anagram word) candidates
```

### Rust (idiomatic — sort-based)
```rust
pub fn is_anagram_sort(s1: &str, s2: &str) -> bool {
    let normalize = |s: &str| -> String { s.to_lowercase() };
    let sorted_chars = |s: &str| -> Vec<char> {
        let mut chars: Vec<char> = s.to_lowercase().chars().collect();
        chars.sort_unstable();
        chars
    };
    normalize(s1) != normalize(s2) && sorted_chars(s1) == sorted_chars(s2)
}

pub fn find_anagrams<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    candidates.iter().copied().filter(|c| is_anagram_sort(word, c)).collect()
}
```

### Rust (functional — frequency counting)
```rust
pub fn is_anagram_freq(s1: &str, s2: &str) -> bool {
    use std::collections::HashMap;
    let lower1 = s1.to_lowercase();
    let lower2 = s2.to_lowercase();
    if lower1 == lower2 { return false; }
    let freq = |s: &str| -> HashMap<char, i32> {
        let mut map = HashMap::new();
        for c in s.chars() { *map.entry(c).or_insert(0) += 1; }
        map
    };
    freq(&lower1) == freq(&lower2)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Anagram check | `val is_anagram : string -> string -> bool` | `fn is_anagram_sort(s1: &str, s2: &str) -> bool` |
| Find anagrams | `val find_anagrams : string -> string list -> string list` | `fn find_anagrams<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str>` |
| Sorted chars | `val to_sorted_chars : string -> char list` | closure: `|s: &str| -> Vec<char>` |
| String type | `string` (immutable) | `&str` (borrowed slice) |

## Key Insights

1. **Pipeline vs method chains:** OCaml's `|>` operator chains `String.to_seq |> List.of_seq |> List.sort`; Rust chains `.chars().collect()` then `.sort_unstable()` — same idea, different syntax
2. **Closures as helpers:** Both languages use local functions/closures for `sorted_chars`; Rust closures capture nothing here (pure transformations)
3. **Lifetime annotations:** Rust's `find_anagrams` returns `Vec<&'a str>` — the compiler needs to know the returned references live as long as the input candidates. OCaml's GC handles this implicitly
4. **HashMap for O(n):** Rust's standard library makes frequency counting natural with `HashMap::entry`; OCaml's stdlib doesn't have a convenient hash map for this pattern
5. **In-place sorting:** Rust sorts `Vec<char>` in-place (no allocation); OCaml creates a new sorted list

## When to Use Each Style

**Use sort-based when:** strings are short and code clarity matters more than performance  
**Use frequency-counting when:** strings are long or you're doing many comparisons — O(n) beats O(n log n)
