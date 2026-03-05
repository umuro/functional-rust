📖 **[View on hightechmind.io →](https://hightechmind.io/rust/282-double-ended-iterator)**

---

# 282: DoubleEndedIterator

**Difficulty:** 3  **Level:** Advanced

Traverse from both ends simultaneously; implement `next_back()` to unlock `.rev()` on your custom iterators.

## The Problem This Solves

You want to reverse an iterator. `rev()` sounds simple, but it requires knowing *where the back of the iterator is*. A filtered iterator over an arbitrary sequence can't support `rev()` — it doesn't know where it ends without consuming everything first. Only iterators that can efficiently access their back end can be reversed lazily.

`DoubleEndedIterator` is the trait that expresses "I know both my front and my back." Implementing `next_back()` alongside `next()` lets the runtime consume from either end. This is how `rev()` works under the hood: it swaps the roles of `next` and `next_back`, giving you backward traversal with zero allocation.

The advanced use case is consuming from both ends simultaneously — zip the front and back of a sequence to compare outer elements, or fold from the outside in. This is the foundation for algorithms like two-pointer approaches expressed functionally.

## The Intuition

Extend your iterator with a `next_back()` method that consumes from the *end* instead of the front — enabling both `rev()` and simultaneous front/back traversal.

## How It Works in Rust

```rust
// Any range or slice iter already implements DoubleEndedIterator
let reversed: Vec<i32> = (1..=5).rev().collect();  // [5, 4, 3, 2, 1]

// rev() on a string — collects backwards
let reversed: String = "hello".chars().rev().collect();  // "olleh"

// Implement DoubleEndedIterator for a custom type
struct Counter { front: i32, back: i32 }

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.front > self.back { return None; }
        let v = self.front;
        self.front += 1;
        Some(v)
    }
}

impl DoubleEndedIterator for Counter {
    fn next_back(&mut self) -> Option<i32> {
        if self.front > self.back { return None; }  // same termination check
        let v = self.back;
        self.back -= 1;  // consume from the back
        Some(v)
    }
}

// Now .rev() works:
Counter::new(5).rev().collect::<Vec<_>>();  // [5, 4, 3, 2, 1]

// Consume from both ends simultaneously
let mut counter = Counter::new(5);
counter.next();       // Some(1) — front
counter.next_back();  // Some(5) — back
counter.next();       // Some(2) — front
counter.next_back();  // Some(4) — back
counter.next();       // Some(3) — middle (front == back)
```

## What This Unlocks

- **Lazy reversal:** `.rev()` on ranges, slices, and custom iterators with zero allocation — no reversed copy, just swapped traversal direction.
- **Two-pointer patterns:** Process elements from both ends inward, like palindrome checks or meeting-in-the-middle algorithms, without indexing.
- **Adapter composition:** `.filter().rev()` — wait, this requires the *filtered* iterator to know its back, which it doesn't. Knowing *when* rev works guides you toward `collect()` first when needed.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reverse a list | `List.rev lst` — allocates new reversed list | `.rev()` on DEI — zero allocation |
| Mechanism | Builds new list | Swaps `next`/`next_back` roles |
| Custom reverse | Implement manually | Implement `next_back()`, get `.rev()` free |
| Not all iterators | N/A (lists are always reversible) | Only types implementing `DoubleEndedIterator` |
| Both ends | Manual recursion | `next()` + `next_back()` simultaneously |
