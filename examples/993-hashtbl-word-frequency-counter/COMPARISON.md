# OCaml vs Rust: Hashtbl — Word Frequency Counter

## Side-by-Side Code

### OCaml

```ocaml
let count_words text =
  let tbl = Hashtbl.create 32 in
  let words = String.split_on_char ' ' text in
  List.iter (fun w ->
    let w = String.lowercase_ascii w in
    let n = try Hashtbl.find tbl w with Not_found -> 0 in
    Hashtbl.replace tbl w (n + 1)
  ) words;
  tbl
```

### Rust (idiomatic — entry API)

```rust
pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let w = word.to_lowercase();
        *map.entry(w).or_insert(0) += 1;
    }
    map
}
```

### Rust (functional — fold accumulator)

```rust
pub fn count_words_fold(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), |mut acc, w| {
            *acc.entry(w).or_insert(0) += 1;
            acc
        })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Hash table type | `('a, 'b) Hashtbl.t` | `HashMap<K, V>` |
| Function signature | `val count_words : string -> (string, int) Hashtbl.t` | `fn count_words(text: &str) -> HashMap<String, usize>` |
| Key type | `string` | `String` (owned) |
| Value type | `int` | `usize` |
| Lookup with default | `try Hashtbl.find tbl k with Not_found -> 0` | `map.entry(k).or_insert(0)` |

## Key Insights

1. **Entry API eliminates the try/catch pattern.** In OCaml you need
   `try Hashtbl.find tbl k with Not_found -> default` for every lookup that
   might miss. Rust's `HashMap::entry` collapses this into a single method
   chain — `entry(key).or_insert(default)` — that is both safer (no exception
   control flow) and faster (one hash computation instead of two).

2. **Ownership means the key is consumed on insert.** `map.entry(w)` takes
   ownership of `w: String`. If the key already exists, the `String` is
   dropped; if it does not, it becomes the stored key. OCaml's GC makes
   deallocation implicit; Rust makes it explicit through drop semantics.

3. **`split_whitespace` vs `split_on_char ' '`.** OCaml's
   `String.split_on_char ' '` splits on exactly one space character and
   produces empty strings at the edges or for consecutive spaces.
   Rust's `str::split_whitespace` splits on any Unicode whitespace sequence
   and never yields empty tokens — a safer default for natural language text.

4. **`usize` vs `int` for counts.** Rust uses `usize` (unsigned, pointer-
   sized) for counts that can never be negative, which reflects the semantic
   intent and avoids an entire class of sign-related bugs present when using
   OCaml's signed `int`.

5. **Fold vs imperative loop are equivalent.** The `fold` variant makes the
   data-flow explicit: the accumulator flows through each iteration and is
   returned at the end. The imperative variant with `mut map` is identical in
   performance — Rust's optimizer sees through both patterns.

## When to Use Each Style

**Use the entry-API loop when:** you are building up a map in a small, focused
function and clarity of intent matters more than composability. Most production
Rust code uses this style.

**Use `fold` when:** you are composing this operation inside a longer iterator
pipeline, or when you want to signal to the reader that the map is built
functionally with no side effects escaping the closure.
