📖 **[View on hightechmind.io →](https://hightechmind.io/rust/105-lifetime-basics)**

---

# 105-lifetime-basics — Lifetime Basics
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Returning a reference from a function is safe only if the referenced data outlives the reference. In C, returning a pointer to a local variable is undefined behavior — the data is destroyed when the function returns. Rust's lifetime system prevents this at compile time by tracking how long each reference is valid.

Lifetime annotations (`'a`) do not change runtime behavior — they are purely compile-time metadata that the borrow checker uses to verify that no reference outlives its data.

## Learning Outcomes

- Understand lifetime annotations as relationships between input and output reference lifetimes
- Read `fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str` and explain what `'a` means
- Annotate structs that hold references
- Understand the three lifetime elision rules and when annotations are omitted
- Know the `'static` lifetime and when it applies

## Rust Application

`src/lib.rs` demonstrates three cases. `longest<'a>(s1: &'a str, s2: &'a str) -> &'a str` shows that the returned reference lives as long as the shorter of `s1` and `s2` — the lifetime `'a` is the intersection. `Important<'a>` is a struct holding `content: &'a str` — the struct cannot outlive the string it borrows. `first_word` shows a single-input function where the output lifetime is inferred to match the input.

Lifetimes do not appear in the compiled binary — they are erased after type checking.

## OCaml Approach

OCaml has no lifetime system. The GC ensures referenced data lives at least as long as any reference to it:

```ocaml
let longest s1 s2 =
  if String.length s1 >= String.length s2 then s1 else s2
(* Both s1 and s2 are GC-managed; the returned reference is always valid *)
```

A struct holding a reference is just a record with a string field — the GC tracks all references and prevents dangling pointers at runtime.

## Key Differences

1. **Compile time vs runtime**: Rust's lifetimes prevent dangling pointers at compile time with zero runtime overhead; OCaml's GC prevents them at runtime.
2. **Annotation burden**: Rust requires explicit lifetime annotations when the compiler cannot infer them; OCaml has none.
3. **`'static` lifetime**: Rust's `&'static str` lives for the entire program duration (string literals); OCaml has no equivalent concept because the GC handles all lifetimes uniformly.
4. **Relationships**: Rust lifetimes express relationships between references (output borrows from input); OCaml's GC makes these relationships implicit.

## Exercises

1. Write a function `longest_word<'a>(sentence: &'a str) -> &'a str` that returns the longest word from a sentence as a borrowed slice.
2. Create a `Cache<'a> { data: &'a [i32], computed: Vec<i32> }` struct and implement a method that borrows from `data` and populates `computed`.
3. Demonstrate the lifetime error when trying to return a reference to a local variable and explain the exact compiler message.
