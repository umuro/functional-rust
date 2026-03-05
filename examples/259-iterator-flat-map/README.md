📖 **[View on hightechmind.io →](https://hightechmind.io/rust/259-iterator-flat-map)**

---

# 259: Flattening with flat_map()

**Difficulty:** 2  **Level:** Intermediate

Map each element to zero-or-more outputs, collecting them all into a single flat sequence.

## The Problem This Solves

Sometimes a transformation produces *multiple* values per input — or none at all. Splitting sentences into words, expanding a range per number, parsing optional results, extracting key-value pairs. A plain `map()` leaves you with nested collections: `Vec<Vec<&str>>`, `Vec<Option<i32>>`. You then need a second pass to flatten them.

In Python you'd write a list comprehension with two `for` clauses: `[word for sentence in sentences for word in sentence.split()]`. In OCaml, `List.concat_map` does the same. In Rust, `flat_map()` is exactly that — map then flatten, in a single lazy pass.

The zero-output case is especially powerful: `flat_map(|s| s.parse::<i32>())` silently drops parse failures because `Result` implements `IntoIterator` (yielding one element on `Ok`, zero on `Err`). This replaces a `filter` followed by an `unwrap`.

## The Intuition

`flat_map(f)` applies `f` to each element (where `f` returns something iterable), then concatenates all the results into one sequence. It's `map` + `flatten` in a single adapter — and it's the iterator monad's `bind` operation.

```rust
let sentences = ["hello world", "foo bar"];
let words: Vec<&str> = sentences.iter()
    .flat_map(|s| s.split_whitespace())
    .collect();
// → ["hello", "world", "foo", "bar"]
```

## How It Works in Rust

```rust
// Expand each number into a range
let nums = [1i32, 2, 3, 4];
let expanded: Vec<i32> = nums.iter().flat_map(|&n| 0..n).collect();
// → [0, 0,1, 0,1,2, 0,1,2,3]

// Silently discard parse failures (Result yields 1 or 0 elements)
let strings = ["1", "two", "3", "four", "5"];
let valid: Vec<i32> = strings.iter()
    .flat_map(|s| s.parse::<i32>())  // Err variants produce 0 items
    .collect();
// → [1, 3, 5]

// CSV: outer iterator = lines, inner iterator = fields
let csv = "1,2,3\n4,5,6";
let values: Vec<&str> = csv.lines()
    .flat_map(|line| line.split(','))
    .collect();
// → ["1","2","3","4","5","6"]
```

`flat_map(f)` is exactly equivalent to `.map(f).flatten()` — use whichever reads more clearly.

## What This Unlocks

- **One-to-many transformations** — split strings, expand ranges, generate permutations per element.
- **Lossy parsing in pipelines** — use `flat_map(|x| x.parse().ok())` to skip invalid inputs without separate filter/unwrap steps.
- **Nested collection flattening** — turn `Vec<Vec<T>>`, lines of CSV, or trees of tokens into a flat stream.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map + flatten | `List.concat_map f lst` | `iter.flat_map(f)` |
| Skip `None`/errors | Manual `filter_map` pattern | `flat_map(\|x\| option_or_result)` |
| Lazy evaluation | No (strict lists) | Yes — produces one element at a time |
| Nesting depth | Flattens all levels with `List.flatten` | Flattens exactly one level |
