# 261: Lookahead with Peekable

**Difficulty:** 2  **Level:** Intermediate

Inspect the next element of an iterator without consuming it — essential for parsers and grouping algorithms.

## The Problem This Solves

Sometimes the decision of what to do with the *current* element depends on what comes *next*. A tokenizer needs to know if the next character is a digit before deciding to start a number. A run-length encoder needs to know if the next element equals the current one before deciding to close a group. A merge algorithm needs to compare heads of two sorted streams.

Without lookahead, you're forced to consume an element just to inspect it — then you need to "put it back", which iterators don't support. The workaround is consuming into a buffer, peeking at index 0, and managing the buffer manually. That's exactly what `Peekable` does for you, but as a proper iterator wrapper.

OCaml doesn't have peekable iterators in the standard library — you typically carry a `ref` to the lookahead value, or restructure the algorithm. In Rust, `.peekable()` wraps any iterator and adds `peek()` in one call.

## The Intuition

`peekable()` wraps an iterator. Calling `peek()` on it returns a reference to the *next* element without advancing the iterator. The next call to `next()` still returns that same element.

```rust
let mut iter = [1, 2, 3].iter().peekable();
iter.peek();   // → Some(&&1)  — iterator still at position 0
iter.next();   // → Some(&1)   — now consumed
iter.next();   // → Some(&2)
```

## How It Works in Rust

```rust
// Group consecutive equal elements
let data = [1i32, 1, 2, 2, 2, 3, 1, 1];
let mut iter = data.iter().peekable();
let mut groups: Vec<Vec<i32>> = Vec::new();

while let Some(&val) = iter.peek() {
    let mut group = Vec::new();
    // consume elements while they equal the peeked value
    while iter.peek() == Some(&val) {
        group.push(*iter.next().unwrap());
    }
    groups.push(group);
}
// → [[1,1], [2,2,2], [3], [1,1]]

// Tokenizer: decide based on what's next
let input: Vec<char> = "123abc".chars().collect();
let mut it = input.iter().peekable();
let mut tokens: Vec<String> = Vec::new();

while it.peek().is_some() {
    if it.peek().unwrap().is_ascii_digit() {
        // collect a full run of digits without re-inserting
        let digits: String = std::iter::from_fn(|| {
            if it.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                it.next().copied()
            } else { None }
        }).collect();
        tokens.push(format!("NUM:{}", digits));
    } else {
        tokens.push(format!("CHAR:{}", it.next().unwrap()));
    }
}
```

`peek()` returns `Option<&Self::Item>` — a reference to the buffered element. For `iter()` over `&T`, that's `Option<&&T>`, so double-deref or use `copied()` when needed.

## What This Unlocks

- **Parsers and tokenizers** — decide how to process a character/token based on what follows.
- **Run-length encoding / grouping** — accumulate elements into a group while the next element matches.
- **Conditional lookahead in merge algorithms** — compare heads of sorted iterators without consuming prematurely.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lookahead | Manual `ref` cell or restructure | `.peekable()` + `.peek()` |
| Non-consuming inspect | Not built-in for sequences | `peek()` — zero-cost buffering |
| Advance after peek | Manually manage | Next `next()` call consumes the peeked element |
| `peek_mut()` | N/A | Mutable reference to next element |
