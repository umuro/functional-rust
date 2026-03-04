# 471: String vs &str

**Difficulty:** 1  **Level:** Beginner

The #1 source of confusion for Rust beginners — here's the clear mental model that makes it click.

## The Problem This Solves

You write `let name = "Alice";` and Rust says it's `&str`. You call `String::from("Alice")` and Rust says it's `String`. You try to pass one where the other is expected and the compiler yells at you. What's the difference? Why does Rust have two string types at all?

Every other common language has one string type. Python has `str`. Java has `String`. JavaScript has `string`. Rust has two — and understanding why unlocks a core piece of how Rust's memory model works.

The answer is ownership. `String` *owns* its data: it allocates memory on the heap, manages it, and frees it when it goes out of scope. `&str` doesn't own anything — it's just a *view* into string data that already exists somewhere. This distinction lets Rust give you maximum flexibility and zero unnecessary copies. Once you have the mental model, you'll see why Rust's approach is actually simpler than it first appears.

## The Intuition

Think of it like the difference between owning a book and borrowing it from a library:

- **`String`** = you own the book. You can write in it, lend it to others, or throw it away. The book lives in your house (heap memory).
- **`&str`** = you borrowed the book. You can read it. You can't throw it away — you don't own it. You just have a reference to where it is.

In Python, every string is owned and reference-counted. In C, you manage memory yourself and burn your fingers doing it. Rust gives you the efficiency of C — no copies you didn't ask for — with the safety of Python — no dangling pointers, no use-after-free.

The practical rule: **write `&str` in function parameters, return `String` when you need to create new data.**

## How It Works in Rust

```rust
// &str: a borrowed view — no heap allocation, just a pointer + length
let literal: &str = "world";  // lives in the program's read-only memory

// String: heap-allocated, owned, growable
let owned = String::from("Alice");

// The magic of deref coercion: &String automatically becomes &str
// So a function that takes &str accepts BOTH literals AND String references
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

greet("world");   // &str literal — works
greet(&owned);    // &String coerces to &str — works
greet(owned.as_str()); // explicit conversion — also works

// Functions that CREATE strings return String (they own the new data)
fn make_greeting(name: &str) -> String {
    format!("Hello, {}!", name)  // format! always creates an owned String
}

// A &str can be a slice of a String (a view into part of it)
let sentence = String::from("Hello World");
fn first_word(s: &str) -> &str {
    &s[..s.find(' ').unwrap_or(s.len())]  // returns a &str pointing INTO s
}
let word = first_word(&sentence);  // word borrows from sentence
// sentence must stay alive while word is used — the compiler enforces this

// &str is a "fat pointer": 16 bytes total (pointer + length)
// No null terminator, can point to any byte range
let s1: &str = "no alloc";
let s2: &str = "no alloc";
// The compiler might share the same memory for identical literals
println!("same ptr: {}", std::ptr::eq(s1, s2)); // often true
```

When does ownership matter here? If you return a `&str` from a function, it must point to data that outlives the function. That's why you can return `&str` from a slice of an input (it borrows the caller's data) but you can't return a `&str` pointing to a local `String` (that would dangle after the function returns).

## What This Unlocks

- **Zero-copy parsing** — slice text into parts (`&str` views) without allocating new strings for each piece
- **Efficient APIs** — accept `&str` in your functions so callers don't need to allocate just to call you
- **Understanding the borrow checker** — `&str` lifetime rules are the clearest illustration of why Rust's borrow checker exists and what it prevents

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Owned string | `string` (immutable, GC-managed) | `String` — heap, growable, owned |
| Borrowed view | Same type — no distinction | `&str` — slice reference, no allocation |
| Parameter convention | `string` | `&str` (accepts `&String` and literals) |
| Create from literal | `"hello"` | `"hello"` is `&str`; `String::from("hello")` owns |
| Convert between | `String.copy s` | `s.to_string()` / `String::from(s)` |
| Lifetime tracking | GC handles it | Borrow checker enforces it at compile time |
