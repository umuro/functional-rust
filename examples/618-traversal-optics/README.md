📖 **[View on hightechmind.io →](https://hightechmind.io/rust/618-traversal-optics)**

---

# 618: Traversal for Collection Optics

**Difficulty:** 5  **Level:** Master

Traverse a collection with an effectful function — short-circuit on `None`/`Err`, collect all results into `Option<Vec<T>>` or `Result<Vec<T>, E>`.

## The Problem This Solves

You have a `Vec<String>` that represents user-provided input. You want to parse every element into `i32`. If *any* element fails to parse, the whole operation should fail. You want a `Vec<i32>` on success, or `None` / an error on failure.

The naive approach: iterate, collect errors separately, handle them after:

```rust
let mut results = Vec::new();
let mut failed = false;
for s in &strs {
    match s.parse::<i32>() {
        Ok(n)  => results.push(n),
        Err(_) => { failed = true; break; }
    }
}
if failed { return None; }
// results is Vec<i32> if we made it here
```

Or with nested iterators:

```rust
let parsed: Vec<Option<i32>> = strs.iter().map(|s| s.parse().ok()).collect();
// Now you have Vec<Option<i32>> — you need to flip it to Option<Vec<i32>>
// How? Another pass, more boilerplate
let result: Option<Vec<i32>> = if parsed.iter().all(|x| x.is_some()) {
    Some(parsed.into_iter().map(|x| x.unwrap()).collect())
} else {
    None
};
```

This pattern — apply a fallible function to every element, short-circuit on first failure, collect results — is exactly what **traversal** is. It appears constantly in data validation, parsing pipelines, and batch processing. This exists to solve exactly that pain.

## The Intuition

A **Traversal** is a Lens that focuses on *multiple* targets at once. Where a Lens reaches one field, a Traversal reaches all elements of a `Vec`, or all `Some` values in a `Vec<Option<T>>`, or all leaves of a tree.

The key operation is **`traverse`**: apply an *effectful* function to each target and collect results. "Effectful" means the function returns a wrapper type — `Option<B>`, `Result<B, E>`, `Vec<B>`.

The magic is in how results are collected:

- **`traverse` with `Option`**: `Vec<A>` + `fn(A) → Option<B>` → `Option<Vec<B>>`
  - All succeed → `Some(Vec<B>)` with all results
  - Any fail → `None` immediately (short-circuit)
  
- **`traverse` with `Result`**: `Vec<A>` + `fn(A) → Result<B, E>` → `Result<Vec<B>, E>`
  - All succeed → `Ok(Vec<B>)`
  - First error → `Err(e)` (short-circuit)

This is the "flip" operation: `Vec<Option<T>>` → `Option<Vec<T>>`. You're converting "a collection of possibly-failed results" into "possibly a collection of results."

**Analogy:** Think of traversal like a checklist. You're checking off items one by one (`A → Option<B>`). If any item fails the check, the whole checklist is invalid (`None`). If all pass, you get the filled-in checklist (`Some(Vec<B>)`). Traverse does this automatically — no manual short-circuit logic needed.

```
Vec<A>  +  fn(A) -> Option<B>
       ↓ traverse
Option<Vec<B>>    ← Some([b1, b2, b3]) or None on first failure
```

In Rust, this is powered by a hidden trick: `Iterator::collect::<Option<Vec<B>>>()` already does this! Collecting an iterator of `Option<B>` into `Option<Vec<B>>` short-circuits on `None`. Traversal just makes this pattern explicit and composable.

## How It Works in Rust

```rust
// Step 1: The fundamental traverse operation — Option effect
fn traverse_opt<A, B>(xs: Vec<A>, f: impl Fn(A) -> Option<B>) -> Option<Vec<B>> {
    xs.into_iter()
        .map(f)              // Iterator<Item = Option<B>>
        .collect()           // collect() on Iterator<Item=Option<B>> into Option<Vec<B>>
                             // ← this is the key: collect() knows to short-circuit!
}

// Works because Rust's FromIterator for Option short-circuits on None
traverse_opt(vec!["1", "2", "3"], |s| s.parse::<i32>().ok());  // Some([1, 2, 3])
traverse_opt(vec!["1", "x", "3"], |s| s.parse::<i32>().ok());  // None — "x" fails

// Step 2: Same pattern for Result — get an error message instead of None
fn traverse_result<A, B, E>(xs: Vec<A>, f: impl Fn(A) -> Result<B, E>) -> Result<Vec<B>, E> {
    xs.into_iter().map(f).collect()  // Same trick! collect() short-circuits on Err
}

// Returns first error encountered:
traverse_result(["1.5", "abc"].to_vec(), |s| {
    s.parse::<f64>().map_err(|_| format!("bad float: {}", s))
});  // Err("bad float: abc")

// Step 3: Traverse a nested structure (matrix)
fn traverse_matrix<A: Clone, B>(
    m: Vec<Vec<A>>,
    f: impl Fn(A) -> Option<B> + Clone,
) -> Option<Vec<Vec<B>>> {
    // outer traverse: for each row, inner traverse: for each element
    traverse_opt(m, |row| traverse_opt(row, f.clone()))
    // If any element in any row fails → None for the whole matrix
}

let matrix = vec![vec!["1", "2"], vec!["3", "4"]];
traverse_matrix(matrix, |s| s.parse::<i32>().ok());
// Some([[1, 2], [3, 4]])

// Step 4: Traversal as filter_map — "Prism traversal"
// Collect only the values that match a pattern (like a Prism over a collection)
fn collect_values<A: Clone, B>(xs: &[A], prism: impl Fn(&A) -> Option<B>) -> Vec<B> {
    xs.iter().filter_map(prism).collect()
    // Different from traverse_opt: here None means "skip this element", not "fail"
}

enum Json { Num(f64), Str(String), Null }
let jsons = vec![Json::Num(1.0), Json::Str("hi".into()), Json::Num(2.0), Json::Null];
let nums = collect_values(&jsons, |j| match j { Json::Num(n) => Some(*n), _ => None });
// [1.0, 2.0] — strings and nulls filtered out

// Step 5: Traverse nested Option<Vec<A>>
fn traverse_nested<A, B>(opt: Option<Vec<A>>, f: impl Fn(A) -> Option<B>) -> Option<Vec<B>> {
    opt.and_then(|xs| traverse_opt(xs, f))
    // None input → None output
    // Some(xs) → traverse xs → Option<Vec<B>>
}
```

## What This Unlocks

- **Data validation pipelines** — parse and validate every field in a batch of records: `traverse_result(records, validate)` gives you either all-valid data or the first error, without manual error-accumulation code.
- **API response handling** — deserialising a `Vec<RawJson>` into `Vec<ParsedItem>` is exactly traversal: `traverse_opt(raw_items, parse_item)`. One `None` means "malformed response."
- **The `Option<Vec<T>>` ↔ `Vec<Option<T>>` flip** — knowing that `collect::<Option<Vec<T>>>()` does the flip explains a class of iterator patterns that are otherwise confusing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| `traverse` | `List.map f xs \|> sequence` or direct `List.filter_map` | `iter().map(f).collect::<Option<Vec<_>>>()` |
| Short-circuit on `None` | Applicative `sequence` over `option` monad | Built into `FromIterator` implementation for `Option<Vec<B>>` |
| `traverse` with `Result` | Same: `sequence` over `result` applicative | `iter().map(f).collect::<Result<Vec<_>, E>>()` |
| Filter-map (Prism traversal) | `List.filter_map` | `iter().filter_map(prism).collect()` |
| Applicative requirement | Explicit: `traverse` requires `Applicative` constraint | Implicit: `FromIterator` trait handles the collection semantics |
