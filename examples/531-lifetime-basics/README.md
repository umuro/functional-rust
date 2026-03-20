📖 **[View on hightechmind.io →](https://hightechmind.io/rust/531-lifetime-basics)**

---

# Lifetime Annotations: 'a Basics

## Problem Statement

Memory safety without a garbage collector requires the compiler to track how long references are valid. C and C++ leave this to the programmer, leading to use-after-free bugs — one of the most common sources of security vulnerabilities (CVEs). Rust solves this with lifetimes: explicit annotations that encode reference validity constraints in the type system. When a function returns a reference, the compiler needs to know which input the output borrows from, so it can reject code that would create a dangling pointer. Lifetime annotations are the mechanism for expressing this relationship.

## Learning Outcomes

- What lifetime annotations express: constraints on how long references remain valid
- How `'a` in `fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str` constrains the output
- The lifetime elision rules that let most functions omit explicit annotations
- How structs holding references require lifetime parameters: `struct Excerpt<'a> { part: &'a str }`
- Why lifetime annotations prevent use-after-free at compile time

## Rust Application

`longer<'a>(s1: &'a str, s2: &'a str) -> &'a str` returns a reference valid for the intersection of both inputs — the caller cannot use the result after either input goes out of scope. `first_word(s: &str) -> &str` uses elision (one input, one output — same lifetime inferred). `pick_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str` has two independent lifetimes; the output is tied only to `'a`. `Excerpt<'a>` demonstrates a struct that borrows from an external string — it cannot outlive that string.

Key patterns:
- `<'a>` lifetime parameter declared on function or struct
- `&'a str` — reference with named lifetime
- Elision: `fn first_word(s: &str) -> &str` — compiler infers the single lifetime

## OCaml Approach

OCaml has no lifetime annotations — the garbage collector ensures referenced values remain alive as long as any reference exists. The equivalent of `longer` is:

```ocaml
let longer s1 s2 = if String.length s1 >= String.length s2 then s1 else s2
```

There is no annotation needed, and the GC prevents dangling references. However, this means OCaml programs cannot express "this reference is borrowed, not owned" at the type level.

## Key Differences

1. **Compile-time vs runtime safety**: Rust lifetime annotations catch dangling references at compile time with zero runtime overhead; OCaml's GC catches use-after-free at... never, because the GC keeps everything alive.
2. **Annotation burden**: Rust requires explicit lifetime annotations when elision rules do not apply; OCaml requires no annotations because ownership is not tracked.
3. **Struct references**: Rust `struct Excerpt<'a>` must annotate the lifetime of borrowed fields; OCaml records holding references need no annotation — the GC manages all values.
4. **Output lifetime ambiguity**: When a function has multiple reference inputs and one reference output, Rust requires the programmer to specify which input the output borrows from; OCaml has no such requirement.

## Exercises

1. **Longest word**: Write `fn longest_word<'a>(sentence: &'a str) -> &'a str` that returns a slice pointing into the original string for the longest whitespace-separated word.
2. **Pair of borrows**: Implement `struct Pair<'a, 'b> { first: &'a str, second: &'b str }` and a method `fn shorter(&self) -> &str` — figure out what lifetime the return type needs.
3. **Nested lifetime**: Write a function `fn inner_word<'a>(outer: &'a str, _separator: &str) -> &'a str` that returns the substring before the first comma, tied only to `outer`'s lifetime.
