# 112: Cow\<T\> — Clone on Write

**Difficulty:** 2  **Level:** Intermediate

`Cow<'a, T>` holds either a borrowed reference or an owned value — deferring heap allocation until the moment you actually need to modify the data.

## The Problem This Solves

A common pattern: a function receives a string or slice, and in most cases returns it unchanged — but occasionally needs to modify it. The naive approach allocates a new `String` every time, even when 90% of calls don't modify anything. Wasteful.

The alternative — returning a reference — doesn't work when the function sometimes needs to return new data it created. You'd need two different return types, a messy `Option`, or caller-side allocation.

In C, you'd use `const char*` for the borrow case and `char*` for the owned case, with a flag or convention to tell the caller which it is. Leaks and double-frees follow. In Python, strings are immutable, so every "modification" already allocates a new string — the problem doesn't arise, but neither does the optimization.

`Cow<'a, T>` (Clone on Write) is a smart enum: `Borrowed(&'a T)` or `Owned(T)`. You manipulate it uniformly. If you never mutate it, no allocation ever happens. The first time you call `to_mut()` on a `Borrowed` variant, it clones into an `Owned`. The allocation is deferred until — and only if — it's actually needed.

## The Intuition

`Cow<T>` is "maybe I'll need to modify this, maybe I won't" — it holds a borrowed reference until the first mutation, then clones into an owned value exactly once, so you only pay for allocation when you actually need it.

## How It Works in Rust

```rust
use std::borrow::Cow;

// Returns borrowed if no change needed, owned only if sanitization happens
fn sanitize(input: &str) -> Cow<str> {
    if input.chars().all(|c| c.is_alphanumeric() || c == ' ') {
        Cow::Borrowed(input)  // no allocation — return a view of the input
    } else {
        let cleaned: String = input
            .chars()
            .filter(|c| c.is_alphanumeric() || c == ' ')
            .collect();
        Cow::Owned(cleaned)   // allocate only when we actually changed something
    }
}

fn demo() {
    let clean = "hello world";
    let dirty = "hello! world@";
    
    let r1 = sanitize(clean); // Borrowed — no allocation
    let r2 = sanitize(dirty); // Owned — allocation happened
    
    println!("{}", r1); // works uniformly
    println!("{}", r2); // works uniformly
}

// to_mut() clones lazily — only on first mutation
fn demo_lazy_clone() {
    let data: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3]);
    
    // No allocation yet
    println!("Length: {}", data.len()); // reads through borrow
    
    // First mutation triggers the clone
    let mut data = data;
    data.to_mut().push(4); // NOW it allocates and clones
    println!("{:?}", data); // [1, 2, 3, 4] — owned
}

// Useful in APIs that accept both &str and String
fn process(input: Cow<str>) {
    // Caller can pass &str or String — no allocation either way
    println!("Processing: {}", input);
}

fn caller() {
    process(Cow::Borrowed("literal"));          // no allocation
    process(Cow::Owned(String::from("owned"))); // already owned
}
```

## What This Unlocks

- **Zero allocation in the common case** — functions that rarely modify their input can return borrowed data for the common path and owned data only for the exceptional path.
- **Flexible APIs** — a function parameter of `Cow<str>` accepts both `&str` and `String` without forcing the caller to allocate or the function to always copy.
- **Deferred cost** — `to_mut()` means you can write code that *might* modify data, and the allocation only happens if you actually call it.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Borrowed vs owned string | `string` (always owned/GC) | `&str` (borrowed) vs `String` (owned) vs `Cow<str>` (either) |
| Lazy copy | Not needed (GC shares) | `Cow` — clone deferred to first mutation |
| Zero-copy read path | GC provides structural sharing | `Cow::Borrowed` — zero allocation |
| Explicit allocation moment | Invisible (GC) | Visible — `to_mut()` or `Cow::Owned` |
| Common use case | N/A | Sanitizers, normalizers, functions that conditionally transform |
