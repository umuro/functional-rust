# 103: Shared References (&T)

**Difficulty:** 2  **Level:** Intermediate

Multiple readers can borrow a value simultaneously — as long as no writer exists — enforced at compile time with zero runtime cost.

## The Problem This Solves

Iterator invalidation is a classic C++ bug: you iterate over a `std::vector`, and inside the loop something modifies the vector — adding an element triggers a reallocation, your iterator now points to freed memory, and you have undefined behavior. The compiler doesn't catch it. It works in testing, crashes in production.

Python sidesteps this at a cost: modifying a list while iterating over it raises `RuntimeError: dictionary changed size during iteration` — caught at runtime, not compile time. You discover the bug when the code runs, not when you write it.

Rust's shared reference rule — "multiple readers, zero writers" — eliminates this entire class of bugs at compile time. While any shared reference (`&T`) to a value exists, the value cannot be mutated. The compiler tracks this. No runtime check, no overhead.

## The Intuition

A shared reference (`&T`) is like a read lock checked at compile time: any number of readers can hold one simultaneously, but the moment any writer exists, no readers are allowed — and the compiler enforces this at zero runtime cost.

## How It Works in Rust

```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
    // s is borrowed — we can read it, but we don't own it
    // s is automatically released when this function returns
}

fn demo_multiple_borrows() {
    let text = String::from("hello, world");

    // Multiple shared references at once — all fine
    let r1 = &text;
    let r2 = &text;
    let r3 = &text;
    println!("{} {} {}", r1, r2, r3); // all three read simultaneously

    // Can't mutate while shared refs exist
    // text.push_str("!"); // ERROR: cannot borrow `text` as mutable
                           // because it is also borrowed as immutable

    // After r1/r2/r3 go out of scope, mutation is fine again
}

fn sum_slice(numbers: &[i32]) -> i32 {
    // &[i32] borrows a slice — no copying, no ownership transfer
    numbers.iter().sum()
}

fn demo_function_borrow() {
    let data = vec![1, 2, 3, 4, 5];
    let total = sum_slice(&data); // borrow data
    println!("Sum: {}, Vec still valid: {:?}", total, data); // data unchanged
}
```

The compiler error when you violate this:
```
error[E0502]: cannot borrow `text` as mutable because it is also borrowed as immutable
```

## What This Unlocks

- **Zero-cost read access** — pass large structs to functions without copying, no allocation, no overhead.
- **Iterator safety** — any code that mutates a collection while you're iterating over it is caught at compile time, not at runtime.
- **Fearless concurrency (read side)** — multiple threads can hold `&T` simultaneously without locks, because the compiler guarantees no mutation is happening.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default access | Immutable by default (like `&T`) | Must explicitly borrow with `&` |
| Multiple readers | Always allowed (GC manages) | Allowed via `&T`; enforced at compile time |
| Read/write conflict | No concept (GC handles) | Compile error if `&T` exists alongside `&mut T` |
| Runtime cost | GC ref counting | Zero — purely compile-time |
| Iterator invalidation | Possible (mutable state) | Impossible — compiler rejects it |
