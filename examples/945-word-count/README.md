**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[word-count on hightechmind.io](https://hightechmind.io/posts/functional-rust/word-count)

---

## Problem Statement

Count word frequencies in a string, normalizing to lowercase and extracting only alphanumeric tokens. Implement three variants: imperative mutation with `HashMap`, a functional fold over owned tokens, and an ordered `BTreeMap` version. Compare the `fold`-based approach with OCaml's pattern of building frequency maps from lists.

## Learning Outcomes

- Tokenize text: lowercase conversion, character-by-character scanning, buffer-based word extraction
- Build a word frequency map using `HashMap` with `entry().or_insert(0)` and `*count += 1`
- Express the same logic as a functional `fold` over an `Iterator` of owned `String` values
- Use `BTreeMap` for alphabetically ordered output — the Rust analog of OCaml's `Map.Make(String)`
- Understand the difference between owned-key insertion and reference-based lookups

## Rust Application

```rust
pub fn tokenize(s: &str) -> Vec<String> {
    let s = s.to_lowercase();
    let mut words = Vec::new();
    let mut buf = String::new();
    for c in s.chars() {
        if c.is_alphanumeric() { buf.push(c); }
        else if !buf.is_empty() { words.push(buf.clone()); buf.clear(); }
    }
    if !buf.is_empty() { words.push(buf); }
    words
}

// Imperative accumulation
pub fn word_count(sentence: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in tokenize(sentence) {
        *map.entry(word).or_insert(0) += 1;
    }
    map
}

// Functional fold — same result, no mutation visible at call site
pub fn word_count_fold(sentence: &str) -> HashMap<String, usize> {
    tokenize(sentence)
        .into_iter()
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        })
}

// Ordered output
pub fn word_count_ordered(sentence: &str) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for word in tokenize(sentence) {
        *map.entry(word).or_insert(0) += 1;
    }
    map
}
```

`entry().or_insert(0)` is the idiomatic Rust pattern for "insert 0 if absent, then increment." It avoids a double lookup that `get` followed by `insert` would require.

`into_iter()` moves the `Vec<String>` into the iterator, so `word` in the fold closure is an owned `String` — suitable as a `HashMap` key without cloning.

`BTreeMap` preserves insertion-order sort (alphabetical for strings). It is the Rust equivalent of OCaml's balanced binary tree map.

## OCaml Approach

```ocaml
let tokenize s =
  let s = String.lowercase_ascii s in
  let buf = Buffer.create 16 in
  let words = ref [] in
  String.iter (fun c ->
    if Char.code c land 127 |> Char.chr |> (fun c -> Char.code c >= 48)
       (* simplified: use is_alnum predicate *)
    then Buffer.add_char buf c
    else if Buffer.length buf > 0 then begin
      words := Buffer.contents buf :: !words;
      Buffer.clear buf
    end
  ) s;
  if Buffer.length buf > 0 then words := Buffer.contents buf :: !words;
  List.rev !words

module StringMap = Map.Make(String)

let word_count sentence =
  List.fold_left (fun acc word ->
    let n = try StringMap.find word acc with Not_found -> 0 in
    StringMap.add word (n + 1) acc
  ) StringMap.empty (tokenize sentence)
```

OCaml's `Map.Make(String)` creates a balanced BST map keyed by strings — always ordered, like Rust's `BTreeMap`. OCaml has no `HashMap` in the standard library (the `Hashtbl` module provides mutable hash tables).

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Default map | `HashMap` — O(1) average, unordered | `Map.Make(M)` — O(log n), always ordered |
| Hash table | `HashMap` | `Hashtbl` (mutable) |
| Entry API | `entry().or_insert(0)` — single lookup | `try find ... with Not_found -> 0` + `add` |
| Ordered map | `BTreeMap` | `Map.Make(M)` |
| Tokenization | Char-level `is_alphanumeric()` | `String.iter` with manual predicate |

`HashMap` is the default Rust choice for frequency counting due to O(1) average-case insertion and lookup. Use `BTreeMap` when sorted output is needed, accepting O(log n) overhead.

## Exercises

1. Add a `top_n(map, n)` function that returns the `n` most frequent words using `sort_by`.
2. Implement `word_count_parallel` that splits the text into chunks, counts each chunk in a thread, then merges the maps.
3. Add stop-word filtering: skip common words like "the", "a", "is" before counting.
4. Write `bigram_count` that counts frequency of consecutive word pairs.
5. Compare `HashMap` vs `BTreeMap` performance on a 10,000-word corpus with a benchmark.
