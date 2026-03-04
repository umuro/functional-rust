# 114: Slice Patterns

**Difficulty:** 2  **Level:** Intermediate

Pattern match on the shape of slices — `[first, rest @ ..]`, `[a, b]`, `[head, .., tail]` — the same structural thinking as OCaml list patterns, but on contiguous memory.

## The Problem This Solves

In C, processing arrays means index-based loops with manual bounds checks. Want to handle "empty array", "one element", and "two or more elements" differently? Three `if` branches, checking `len == 0`, `len == 1`, `len >= 2`. Miss a case, or check the wrong index, and you have an out-of-bounds access.

Most languages with pattern matching focus on ADTs (algebraic data types) and enums. But sequences — arrays, slices, lists — are ubiquitous. Having to write structural code with explicit indexing when you could just match on shape is a missed opportunity.

OCaml's list patterns (`| [] -> ... | x :: rest -> ...`) are elegant for linked lists but don't apply to arrays. Rust's slice patterns fill this gap: you can match on the structure of any contiguous sequence with the same clarity. The compiler guarantees exhaustiveness — every case you forgot is a warning or error, not a runtime crash.

## The Intuition

Slice patterns let you describe the *shape* of a slice — "empty", "exactly one element", "first element and the rest" — and the compiler matches structurally, with exhaustiveness checking to ensure no case is missed.

## How It Works in Rust

```rust
fn describe_slice(nums: &[i32]) -> &str {
    match nums {
        []        => "empty",
        [_]       => "exactly one element",
        [_, _]    => "exactly two elements",
        [first, .., last] => {
            println!("first={}, last={}", first, last);
            "three or more elements"
        }
    }
}

// Recursive-style processing with slice patterns
fn sum_recursive(nums: &[i32]) -> i32 {
    match nums {
        []              => 0,
        [head, tail @ ..] => head + sum_recursive(tail),
        // tail @ .. binds the rest of the slice to `tail`
    }
}

// Destructure specific positions
fn first_two(nums: &[i32]) -> Option<(i32, i32)> {
    match nums {
        [a, b, ..] => Some((*a, *b)),
        _          => None,
    }
}

// Nested patterns
fn starts_and_ends(data: &[&str]) -> Option<(&str, &str)> {
    match data {
        [first, .., last] => Some((first, last)),
        [only]            => Some((only, only)),
        []                => None,
    }
}

// Matching with guards
fn categorize(nums: &[i32]) -> &str {
    match nums {
        [] => "empty",
        [n] if *n > 0 => "single positive",
        [n] if *n < 0 => "single negative",
        [n] => "single zero",       // n == 0
        [a, b] if a == b => "two equal elements",
        _ => "general case",
    }
}

fn demo() {
    println!("{}", describe_slice(&[]));       // "empty"
    println!("{}", describe_slice(&[42]));     // "exactly one element"
    println!("{}", describe_slice(&[1,2,3]));  // "three or more elements"
    
    println!("{}", sum_recursive(&[1, 2, 3, 4, 5])); // 15
    println!("{:?}", first_two(&[10, 20, 30]));       // Some((10, 20))
}
```

## What This Unlocks

- **Exhaustive sequence matching** — the compiler warns if you miss a case; no silent bugs from forgetting to handle an empty slice.
- **OCaml-style structural thinking on arrays** — write recursive algorithms on slices with the same elegance as OCaml's list patterns, but on contiguous memory (no linked-list overhead).
- **Readable over-indexing** — `[first, second, ..]` is more readable than `nums[0]` and `nums[1]` with manual length checks.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sequence pattern matching | `[]` and `x :: rest` on lists | `[]`, `[x]`, `[h, t @ ..]` on slices |
| Underlying data structure | Linked list (cons cells) | Contiguous memory (slice `&[T]`) |
| Rest binding | `match lst with \| x :: rest ->` | `[head, rest @ ..]` |
| Random access cost | O(n) for lists | O(1) for slices |
| Exhaustiveness | Compiler-checked | Compiler-checked |
