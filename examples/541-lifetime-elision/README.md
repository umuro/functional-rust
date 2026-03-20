📖 **[View on hightechmind.io →](https://hightechmind.io/rust/541-lifetime-elision)**

---

# Lifetime Elision Rules

## Problem Statement

Writing explicit lifetime annotations on every function would make Rust code extremely verbose. Lifetime elision rules were introduced to allow the compiler to infer annotations in the most common cases, making everyday code read cleanly. Three rules cover the vast majority of functions: (1) each input reference gets its own lifetime, (2) if there is exactly one input lifetime, it propagates to all output references, (3) if one of the inputs is `&self` or `&mut self`, its lifetime propagates to all output references. Understanding these rules explains when annotations are required and why.

## Learning Outcomes

- The three elision rules that allow annotations to be omitted in common cases
- How `fn strlen(s: &str) -> usize` expands to `fn strlen<'a>(s: &'a str) -> usize`
- How `fn first_word(s: &str) -> &str` expands by Rule 2 (one input → output gets its lifetime)
- How `fn remaining(&self) -> &str` on a struct expands by Rule 3 (&self lifetime)
- When elision fails: multiple reference inputs with a reference output require explicit annotation

## Rust Application

`strlen(s: &str) -> usize` — no output reference, so only Rule 1 applies (input gets its own lifetime). `first_word(s: &str) -> &str` — Rule 2: one input `&str` → output `&str` gets the same lifetime. `Parser<'a>` with method `remaining(&self) -> &str` — Rule 3: `&self` has `'a`, so the output `&str` gets `'a`. `longer<'a>(x: &'a str, y: &'a str) -> &'a str` — two inputs, elision cannot determine which, so annotation is required. The source file includes comments showing both the elided and expanded forms.

Key patterns:
- Elided form: `fn f(s: &str) -> &str` (readable)
- Expanded form: `fn f<'a>(s: &'a str) -> &'a str` (explicit, same semantics)
- Failure case: two input `&str` with one output `&str` — must annotate

## OCaml Approach

OCaml has no lifetime elision because there are no lifetime annotations to elide. All functions operate on GC-managed values, and no annotation is ever required:

```ocaml
let strlen s = String.length s
let first_word s = match String.split_on_char ' ' s with w :: _ -> w | [] -> ""
let longer x y = if String.length x >= String.length y then x else y
```

## Key Differences

1. **Annotation reduction**: Rust's elision rules eliminate annotations in ~90% of practical cases; OCaml eliminates them in 100% of cases because the GC removes the need.
2. **Rule transparency**: Rust programmers must understand elision rules to read and write idiomatic code; OCaml programmers never encounter lifetime annotations.
3. **Elision failure**: When Rust elision fails (multiple input references, multiple output references), explicit annotations are required — a learning hurdle that OCaml completely avoids.
4. **Correctness guarantee**: Rust's elision rules are specified precisely — they are not guesses; the expanded form is provably equivalent; OCaml's implicit safety comes from GC correctness.

## Exercises

1. **Manual expansion**: Take the five functions in the source file and write out their fully-expanded forms with all lifetime annotations explicit — verify they produce identical behavior.
2. **Elision limit**: Write a function `fn pick(cond: bool, a: &str, b: &str) -> &str` — observe that elision fails here and add the correct annotation.
3. **Method elision**: Add a method `fn peek(&self) -> char` to `Parser` that returns the first character of `self.input` — verify elision applies Rule 3 correctly.
