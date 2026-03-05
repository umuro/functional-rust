# OCaml vs Rust: Anadromes

## Side-by-Side Code

### OCaml

```ocaml
module StrSet = Set.Make(String)

let string_rev s =
  let last = pred (String.length s) in
  String.init (succ last) (fun i -> s.[last - i])

let get_anadromes set =
  let aux s =
    let r = string_rev s in
    if s < r && StrSet.mem r set
    then Some (s, r)
    else None
  in
  Seq.filter_map aux (StrSet.to_seq set)

(* Pipeline:
   read stdin
   |> filter (length > 6)
   |> map lowercase
   |> StrSet.of_seq
   |> get_anadromes
   |> iter print *)
```

### Rust (idiomatic)

```rust
use std::collections::BTreeSet;

pub fn string_rev(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn get_anadromes(words: &BTreeSet<String>) -> Vec<(String, String)> {
    words
        .iter()
        .filter_map(|s| {
            let r = string_rev(s);
            if s.as_str() < r.as_str() && words.contains(&r) {
                Some((s.clone(), r))
            } else {
                None
            }
        })
        .collect()
}
```

### Rust (functional pipeline — mirrors OCaml's `main`)

```rust
pub fn build_word_set<I>(words: I, min_len: usize) -> BTreeSet<String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    words
        .into_iter()
        .map(|w| w.as_ref().to_lowercase())
        .filter(|w| w.len() > min_len)
        .collect()
}

// Usage — same pipeline as the OCaml main:
let set = build_word_set(lines, 6);
let pairs = get_anadromes(&set);
for (a, b) in pairs {
    println!("{:>9} | {}", a, b);
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| String reversal | `val string_rev : string -> string` | `fn string_rev(s: &str) -> String` |
| Word set | `StrSet.t` (balanced BST) | `BTreeSet<String>` |
| Anadrome finder | `val get_anadromes : StrSet.t -> (string * string) Seq.t` | `fn get_anadromes(words: &BTreeSet<String>) -> Vec<(String, String)>` |
| Optional pair | `'a option` returned from `aux` | `Option<(String, String)>` passed to `filter_map` |
| Lowercase | `String.lowercase_ascii` | `str::to_lowercase()` (full Unicode) |

## Key Insights

1. **String reversal — char vs byte safety:** OCaml's `s.[i]` is a byte index (safe for ASCII); Rust's `.chars().rev()` iterates Unicode scalar values, making it correct for any UTF-8 input without special handling.

2. **Ordered sets give free deduplication:** Both `Set.Make` and `BTreeSet` iterate in sorted order. The `s < rev(s)` guard means each anadrome pair is encountered exactly once — the smaller string first. No extra bookkeeping needed.

3. **`filter_map` = filter + transform fused:** OCaml's `Seq.filter_map` and Rust's `.filter_map()` both express "maybe transform this element" in one combinator, eliminating a two-step filter + map. This mirrors the `aux` function in OCaml directly.

4. **Lazy vs eager:** OCaml's `Seq` is lazy (streams on demand); Rust's iterator chain is also lazy but `.collect()` eagerly materialises the result into a `Vec`. The algorithmic structure is identical — only the evaluation strategy differs.

5. **Membership test cost:** Both `StrSet.mem` and `BTreeSet::contains` are `O(log n)`. A `HashSet` would give `O(1)` average but lose the sorted-iteration property that enables the duplicate-avoidance trick.

## When to Use Each Style

**Use idiomatic Rust (`filter_map` chain) when:** you want to stay in iterator-land and compose transforms lazily before a single final `.collect()` — this is the standard Rust idiom for data-processing pipelines.

**Use the functional/recursive style when:** you need to make the structural recursion explicit for educational purposes or when the algorithm's correctness depends on the recursive decomposition being visible (e.g., tree traversals).
