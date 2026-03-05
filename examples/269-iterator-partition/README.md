📖 **[View on hightechmind.io →](https://hightechmind.io/rust/269-iterator-partition)**

---

# 269: Splitting by Predicate with partition()

**Difficulty:** 1  **Level:** Beginner

Split an iterator into two collections — matching and non-matching — in a single pass.

## The Problem This Solves

You want to separate a list into two groups: the elements that satisfy a condition and those that don't. Evens and odds. Valid and invalid inputs. `Ok` results and `Err` results. The naive approach runs `filter` twice — once for each half — iterating the source twice and making the intent less clear. Or you write a loop with two `push` calls and a conditional.

`partition()` does both halves in one pass, returning two collections simultaneously. It's both more efficient (single iteration) and more expressive (the split intent is explicit).

In Python, you'd use two comprehensions or `itertools.partition`. In OCaml, `List.partition`. In Rust, `partition()` is available on any iterator.

## The Intuition

`partition(pred)` returns `(matching, not_matching)` — two collections where every element from the iterator ends up in exactly one of them.

```rust
let (evens, odds): (Vec<i32>, Vec<i32>) =
    (1..=10).partition(|&x| x % 2 == 0);
// evens → [2, 4, 6, 8, 10]
// odds  → [1, 3, 5, 7, 9]
```

## How It Works in Rust

```rust
// Separate evens and odds
let nums: Vec<i32> = (1..=10).collect();
let (evens, odds): (Vec<i32>, Vec<i32>) =
    nums.iter().copied().partition(|&x| x % 2 == 0);

// Separate by string length
let words = ["hi", "hello", "yo", "world", "hey", "programming"];
let (short, long): (Vec<_>, Vec<_>) = words.iter().partition(|w| w.len() <= 3);

// Separate Ok and Err results (very common pattern)
let results: Vec<Result<i32, &str>> = vec![
    Ok(1), Err("bad"), Ok(3), Err("fail"), Ok(5)
];
let (oks, errs): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
// Then extract values:
let ok_vals: Vec<i32> = oks.into_iter().flatten().collect();
let err_msgs: Vec<&str> = errs.into_iter().map(|r| r.unwrap_err()).collect();

// Numeric sign splitting
let data = [-3i32, 1, -1, 4, -1, 5, 9, -2, 6];
let (pos, neg): (Vec<i32>, Vec<i32>) =
    data.iter().copied().partition(|&x| x >= 0);
println!("Sum pos: {}, Sum neg: {}", pos.iter().sum::<i32>(), neg.iter().sum::<i32>());
```

Unlike `take_while`/`skip_while`, `partition` scans the *entire* iterator — it does not stop early. Every element is evaluated and placed.

## What This Unlocks

- **Result/Option separation** — split a `Vec<Result<T, E>>` into success and failure vecs in one pass.
- **Single-pass dual filter** — replaces two `filter` calls on the same data with one `partition`.
- **Data classification** — route records into two processing pipelines (valid/invalid, above/below threshold).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Split by predicate | `List.partition pred lst` | `iter.partition(pred)` |
| Scans entire iterator | Yes | Yes — no early termination |
| vs. `take_while`/`filter` | `partition` keeps both halves | `take_while` stops; `filter` drops one half |
| Collection type | Always `list * list` | Generic — type annotation required |
