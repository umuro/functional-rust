# 105: Lifetime Basics

**Difficulty:** 3  **Level:** Advanced

Lifetime annotations tell the compiler how long a reference must remain valid — turning dangling pointer bugs into compile errors.

## The Problem This Solves

A dangling pointer is a reference to memory that has already been freed. In C, this is trivially easy to create: return a pointer to a local variable, and the caller holds a reference to a stack frame that no longer exists. The behavior is undefined — sometimes it works, sometimes it crashes, sometimes it silently returns wrong data. C compilers warn about the simplest cases, but complex patterns slip through.

Python and OCaml avoid this entirely with garbage collection: memory is never freed while any reference to it exists. But that requires a runtime that tracks all live references — overhead that Rust doesn't pay.

Rust proves the absence of dangling pointers at compile time using lifetimes. A lifetime annotation (`'a`) is the compiler's way of naming "how long this reference lives." When a function returns a reference, the compiler needs to know: does this reference point into the first argument, the second, a static allocation, or somewhere else? The annotation makes that relationship explicit, and the compiler verifies the pointed-to data outlives the reference.

## The Intuition

Lifetime annotations are the compiler's way of asking you to label which data a reference points into — so it can prove, before the program runs, that no reference will ever outlive the data it points to.

## How It Works in Rust

```rust
// Without lifetime annotation, the compiler can't reason about this:
// fn longest(s1: &str, s2: &str) -> &str { ... }
// Which input does the return value point into? The compiler doesn't know.

// With lifetime annotation: 'a means "return value lives as long as both inputs"
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() { s1 } else { s2 }
}

fn demo() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", result); // fine — both alive here
    }
    // println!("{}", result); // ERROR: s2 dropped, result might point to it
}

// ERROR: returning reference to local variable
fn dangling() -> &str {  // missing lifetime
    let s = String::from("hello");
    &s  // ERROR: s is dropped when function returns, reference would dangle
}

// FIX: return owned value, or take the data as input
fn not_dangling(s: &str) -> &str {
    s  // returning a reference to the input — always lives long enough
}

// Sometimes only one input, so return must come from it:
fn first_word(s: &str) -> &str {
    // lifetime elided — compiler infers return borrows from s
    s.split_whitespace().next().unwrap_or("")
}
```

The compiler error for a dangling reference:
```
error[E0106]: missing lifetime specifier
error[E0515]: cannot return reference to local variable `s`
```

## What This Unlocks

- **Provably no dangling pointers** — the compiler checks every reference-returning function and rejects any path that could produce a dangling reference.
- **Zero-cost safety** — lifetimes are erased at compile time; at runtime, references are just pointers, with no overhead whatsoever.
- **Self-documenting relationships** — `<'a>` annotations make explicit which data a function borrows and how long its outputs are valid; this information is otherwise implicit and error-prone.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Dangling references | Impossible (GC keeps data alive) | Impossible (compiler rejects via lifetimes) |
| Reference lifetime | GC-managed, always valid | Explicitly tracked, proven at compile time |
| Lifetime annotations | Not needed (no concept) | Required when compiler can't infer |
| Runtime cost | GC overhead | Zero — lifetimes are compile-time only |
| Dangling pointer bug | Can't happen (GC) | Can't happen (borrow checker) |
