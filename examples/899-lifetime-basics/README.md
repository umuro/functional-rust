📖 **[View on hightechmind.io →](https://hightechmind.io/rust/899-lifetime-basics)**

---

# 899-lifetime-basics — Lifetime Basics

## Problem Statement

A dangling pointer — a reference to memory that has been freed — is one of the most common and dangerous bugs in C/C++. Rust prevents dangling pointers through lifetimes: compile-time annotations that track how long a reference remains valid. When a function returns a reference, the compiler needs to know whether it comes from parameter `a`, parameter `b`, or neither. Explicit lifetime parameters `'a` provide this information. OCaml's GC prevents dangling pointers at runtime; Rust prevents them at compile time with zero runtime cost. Lifetimes are the mechanism behind Rust's memory safety guarantee.

## Learning Outcomes

- Read and write lifetime annotations on function signatures
- Understand that lifetimes express relationships between reference lifetimes, not their duration
- Use lifetime annotations in structs that hold references
- Understand lifetime elision rules for common single-reference patterns
- Compare Rust's compile-time lifetime tracking with OCaml's GC-based approach

## Rust Application

`longest<'a>(a: &'a str, b: &'a str) -> &'a str` tells the compiler the returned reference lives no longer than the shorter of the two inputs. `trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str` returns a reference tied to `s` only — `prefix` is not involved in the output lifetime. The `Excerpt<'a>` struct holds `text: &'a str` — the struct cannot outlive the string it references. `longest_in<'a>(strs: &[&'a str]) -> Option<&'a str>` finds the longest string without copying, borrowing from the slice's elements.

## OCaml Approach

OCaml has no lifetime annotations. All values are heap-allocated and GC-managed — there is no concept of a value "going out of scope" while it has a live reference. Functions returning references to parameters are impossible in the C sense; OCaml functions always return GC-managed values. The equivalent safety guarantee comes from the GC: no value is freed while any reference to it exists. The cost is GC overhead; the benefit is no explicit lifetime management.

## Key Differences

1. **Compile-time vs runtime**: Rust lifetimes enforce safety at compile time (zero overhead); OCaml's GC enforces it at runtime (GC overhead).
2. **Annotation burden**: Rust requires lifetime annotations when the compiler cannot infer them; OCaml requires no annotations — the GC handles it.
3. **Elision rules**: Rust elides lifetimes in common patterns (single reference input, `&self` methods); when elision doesn't apply, explicit `'a` is required.
4. **Struct lifetimes**: Rust structs containing references need lifetime parameters; OCaml records can contain any values freely.

## Exercises

1. Write `first_word<'a>(s: &'a str) -> &'a str` that returns the first whitespace-delimited word as a borrowed slice.
2. Implement a `Cache<'a, T>` struct that holds a reference to a slice and a computed value, where both must have the same lifetime.
3. Write `longer_name<'a, 'b>(first: &'a str, last: &'b str) -> &'a str` and explain why the return lifetime is `'a` and not `'b`.
