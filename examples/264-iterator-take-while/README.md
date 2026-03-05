📖 **[View on hightechmind.io →](https://hightechmind.io/rust/264-iterator-take-while)**

---

# 264: Conditional Stopping with take_while()

**Difficulty:** 1  **Level:** Beginner

Consume elements from the front of an iterator until a condition fails — lazy, works on infinite iterators.

## The Problem This Solves

You have a sorted sequence or a stream and you want everything up to a boundary. Reading log entries until you hit a timestamp past midnight. Taking words until you hit an empty string. Consuming sorted numbers up to a threshold. With a for loop, you'd set a flag or `break` — which works, but you lose the composability of the iterator chain.

The critical distinction from `filter()` is ordering: `take_while` assumes elements are ordered with respect to the predicate. It stops at the *first* failure — it doesn't scan the whole iterator. That makes it the only option for infinite iterators, where scanning everything is impossible.

OCaml has `List.filteri` but not a built-in `take_while` for lists (though `Seq.take_while` exists). In Rust, `take_while(pred)` is available on any iterator and is lazy.

## The Intuition

`take_while(pred)` yields elements as long as the predicate returns `true`. The moment it returns `false`, the iterator stops — even if later elements would match.

```rust
let nums = [1, 2, 3, 4, 5, 4, 3];
let result: Vec<_> = nums.iter().take_while(|&&x| x < 4).collect();
// → [1, 2, 3]   stops at 4, never sees the trailing 3
```

## How It Works in Rust

```rust
// Basic: take leading elements less than 5
let nums = [1i32, 2, 3, 4, 5, 6, 7, 8, 9];
let small: Vec<i32> = nums.iter().copied().take_while(|&x| x < 5).collect();
// → [1, 2, 3, 4]

// Stops at first failure — NOT a filter
let data = [3i32, 1, 4, 1, -5, 9];
let positives: Vec<i32> = data.iter().copied().take_while(|&x| x > 0).collect();
// → [3, 1, 4, 1]    stops at -5; the 9 after it is never seen

// Works on infinite iterators — essential use case
let triangulars: Vec<u64> = (1u64..)
    .take_while(|&n| n * (n + 1) / 2 < 30)
    .collect();
// → [1, 2, 3, 4, 5, 6, 7]   n where triangle number < 30

// Parse a leading alphabetic word from mixed input
let word: String = "hello123world".chars()
    .take_while(|c| c.is_alphabetic())
    .collect();
// → "hello"

// Sorted list: short words at the front
let sorted_words = ["ant", "bee", "cat", "dog", "elephant"];
let short: Vec<_> = sorted_words.iter()
    .take_while(|w| w.len() <= 3)
    .collect();
// → ["ant", "bee", "cat", "dog"]
```

Combine with `skip_while` to extract a middle range: skip a prefix, then take until the end condition.

## What This Unlocks

- **Infinite iterator termination** — the only safe way to consume from `0..` or `successors()` when you don't know the count upfront.
- **Sorted boundary extraction** — take all elements from a sorted stream up to a threshold without scanning past it.
- **Text parsing** — collect a leading run of digits, letters, or whitespace from a character stream.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Take prefix by predicate | `Seq.take_while` (lazy) / manual for lists | `iter.take_while(pred)` |
| Stops at first failure | Yes (same semantics) | Yes |
| Works on infinite sequences | `Seq` only | Any `Iterator` |
| Combined with skip | Manual composition | `.skip_while(p1).take_while(p2)` |
