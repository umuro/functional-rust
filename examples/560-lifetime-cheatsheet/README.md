📖 **[View on hightechmind.io →](https://hightechmind.io/rust/560-lifetime-cheatsheet)**

---

# Lifetime Cheatsheet
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Lifetime annotations in Rust are a powerful but syntactically noisy feature. After learning each individual rule in isolation, programmers often struggle to apply them quickly when reading or writing real code. A cheatsheet consolidates the most common patterns — elision, structs with lifetimes, impl blocks, static lifetimes, HRTB, and subtyping — into a single reference with expanded forms and their practical interpretations.

## Learning Outcomes

- Quick recall of all major lifetime annotation patterns in one place
- How to read elided lifetimes and mentally expand them to explicit forms
- Common struct, impl, and trait patterns with lifetimes side-by-side
- When each pattern is appropriate and what it communicates about the API
- Lifetime-related terminology: region, covariance, subtyping, HRTB, static

## Rust Application

The source is structured as a reference: `trim(s: &str) -> &str` (elided), `longer<'a>(a, b) -> &'a str` (two inputs), `struct View<'a>` (struct), `impl<'a> View<'a>` (impl), `T: 'static` (outlives bound), `for<'a> Fn(&'a str)` (HRTB). Each pattern is annotated with its expanded form and when to use it. This example is reference material rather than a single algorithm — read alongside other lifetime examples for context.

Key patterns summarized:
- `&'a T` — reference valid for lifetime `'a`
- `T: 'a` — T outlives lifetime `'a` (contains no shorter-lived borrows)
- `'a: 'b` — `'a` outlives `'b` (subtyping)
- `for<'a>` — universal quantification (HRTB)
- `'static` — valid for entire program duration

## OCaml Approach

OCaml has no lifetime syntax — the cheatsheet concept does not apply. The equivalent reference for OCaml would cover GC semantics, weak references, and the `Gc` module rather than lifetime annotations.

```ocaml
(* OCaml "lifetime cheatsheet": everything is GC-managed
   - ref t: mutable reference, always valid while reachable
   - Weak.t: non-retaining reference, becomes None after GC
   - module Gc: control GC behavior
*)
```

## Key Differences

1. **Annotation presence**: Rust requires lifetime annotations in many signatures; OCaml requires none — the entire domain of knowledge captured here is Rust-specific.
2. **Learning curve**: Lifetimes are the most commonly cited Rust learning challenge; OCaml's GC model is simpler to learn but less powerful for systems programming.
3. **Cheatsheet necessity**: Rust programmers frequently consult lifetime rules; OCaml programmers rarely need to look up memory management syntax.
4. **Runtime vs compile-time**: Rust's lifetime system moves memory safety guarantees to compile time; OCaml's GC provides them at runtime — both are correct, with different performance profiles.

## Exercises

1. **Pattern classification**: Take five functions from previous examples and classify each lifetime parameter by its role: input constraint, output source, subtyping, or HRTB.
2. **Expand elided forms**: Write the fully-explicit lifetime-annotated forms of `fn process(s: &str) -> &str`, `fn combine(a: &str, b: &str) -> String`, and a struct method returning `&str`.
3. **Custom cheatsheet**: Add two more patterns not covered in the source: (a) a function with a `where T: 'a` bound, and (b) a function with an `impl Trait + 'a` return type — explain each in a comment.
