# 277: Iterator count()

**Difficulty:** 1  **Level:** Beginner

Consume an iterator and return the total number of elements as `usize`.

## The Problem This Solves

You need to know how many elements satisfy a condition — how many words in a text, how many even numbers in a range, how many log lines match an error pattern. Without `count()`, you'd write a fold accumulating an integer counter, or collect into a Vec and call `.len()` — wasteful if you only want the count.

`count()` composes naturally with `filter()`: `iter.filter(pred).count()` is the idiomatic "count matching elements" pattern. It's one expression, one traversal, no intermediate allocation. For ranges and slices (which implement `ExactSizeIterator`), `count()` runs in O(1) — it just reads the stored length.

The catch: `count()` consumes the iterator. You can't use it again afterward. For slices, if you only want the length, prefer `.len()` directly — it's O(1) and doesn't move anything.

## The Intuition

Walk the entire iterator and tally the elements, returning the total as `usize`.

## How It Works in Rust

```rust
let nums: Vec<i32> = (1..=10).collect();

// Basic count
nums.iter().count();  // 10

// Count matching elements — one expression, no intermediate Vec
nums.iter().filter(|&&x| x % 2 == 0).count();  // 5

// Count characters matching a predicate
"hello world".chars().filter(|c| "aeiou".contains(*c)).count();  // 3

// Count words
"the quick brown fox".split_whitespace().count();  // 4

// Range count is O(1) — ExactSizeIterator knows its length
(0usize..1_000_000).count();  // 1_000_000, instant

// Combine with take_while for "how many elements before X"
let sorted = [1, 3, 5, 7, 9, 11];
sorted.iter().take_while(|&&x| x < 10).count();  // 5
```

`count()` returns `usize` — unsigned, platform-sized. Can't be negative, can't be used in signed arithmetic without casting.

## What This Unlocks

- **Predicate counting:** `filter(pred).count()` replaces manual loop counters for any "how many match?" query.
- **Text analysis:** Count words, vowels, sentences, or any character class in a string with a single expression.
- **Validation:** Count errors in a result stream; `errors.count() == 0` as a clean "is everything OK?" check.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Count list | `List.length lst` | `iter.count()` |
| Count matching | `List.length (List.filter p lst)` | `iter.filter(p).count()` |
| Complexity | O(n) always | O(n); O(1) for `ExactSizeIterator` |
| Return type | `int` (signed) | `usize` (unsigned, platform-sized) |
| After counting | List still available | Iterator consumed — can't reuse |
