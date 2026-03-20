📖 **[View on hightechmind.io →](https://hightechmind.io/rust/106-lifetime-elision)**

---

# 106-lifetime-elision — Lifetime Elision Rules
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Lifetime annotations are often redundant — the compiler can infer them from context. Rust's three lifetime elision rules specify exactly when annotations can be omitted, making the common cases concise while requiring explicit annotations only for ambiguous cases.

Understanding elision rules helps you read Rust code that lacks annotations and know when you need to add them. It also explains why `fn first_word(s: &str) -> &str` compiles without any `'a`.

## Learning Outcomes

- Know the three elision rules by name and understand what each covers
- Recognize when a function's lifetimes are inferred vs must be explicit
- Understand rule 3 (self lifetime) for methods on structs
- Know when two input references require an explicit output lifetime
- Read and write lifetime annotations only when the rules do not apply

## Rust Application

`src/lib.rs` uses CLRS-style inline documentation explaining each rule. `first_word(s: &str) -> &str` uses rule 2 (one input → output gets same lifetime). `pick_first<'a>(a: &'a str, _b: &str) -> &'a str` requires explicit `'a` because rule 2 does not apply with two input references. The `TextBuffer` struct methods use rule 3 (method with `&self` → output gets `self`'s lifetime), requiring no annotations.

The three rules are: (1) each input reference gets its own lifetime; (2) if there is exactly one input lifetime, all outputs get it; (3) if one input is `&self`/`&mut self`, all outputs get the self lifetime.

## OCaml Approach

OCaml has no lifetime annotations and no elision rules — all references are managed by the GC with no lifetime tracking:

```ocaml
let first_word s = String.split_on_char ' ' s |> List.hd

(* Two inputs — OCaml makes no distinction *)
let pick_first a _b = a  (* GC tracks both; no lifetime concern *)
```

All OCaml functions return values that the GC will keep alive as long as needed. The programmer never needs to annotate how long a return value borrows from an argument.

## Key Differences

1. **Elision scope**: Rust elision rules cover the most common patterns; explicit annotations are needed for any case beyond them. OCaml needs no annotations at all.
2. **Ambiguity**: Two input references in Rust create ambiguity that the compiler reports as an error if the output is a reference; OCaml has no such ambiguity.
3. **Rule 3 value**: The `&self` lifetime rule makes virtually all method return values annotation-free, which is why most Rust struct methods look clean.
4. **Learning curve**: Rust programmers need to internalize the three rules; OCaml programmers have no equivalent concept to learn.

## Exercises

1. Write a function with two `&str` inputs and one `&str` output. Verify that an explicit lifetime annotation is required. Then add it.
2. Create a struct `Parser<'a> { input: &'a str, pos: usize }` and implement three methods that all return references — verify which ones need explicit annotations.
3. Write a function `fn split_at_first_space(s: &str) -> (&str, &str)` returning two slices of the input and verify the elision rules apply.
