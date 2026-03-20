📖 **[View on hightechmind.io →](https://hightechmind.io/rust/567-pattern-binding-modes)**

---

# Pattern Binding Modes
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

When matching on a reference (e.g., `&Option<String>` instead of `Option<String>`), Rust historically required explicit `ref` keywords to borrow rather than move matched values. Match ergonomics (RFC 2005, Rust 2018) made this automatic in most cases: matching on `&T` adjusts the binding mode so `Some(s)` binds `s: &String` rather than requiring `Some(ref s)`. Understanding binding modes explains many "borrowed vs moved" questions that arise when matching references, and helps write code that works correctly with both owned and borrowed match targets.

## Learning Outcomes

- How `ref` explicitly creates a reference binding in a pattern
- How match ergonomics automatically applies `ref` when matching on `&T`
- How `ref mut` creates a mutable reference binding
- When explicit `ref` is still necessary vs when ergonomics handles it
- The difference between matching `Option<String>` (moves) vs `&Option<String>` (borrows)

## Rust Application

`move_binding(opt: Option<String>)` matches on an owned value — `Some(s)` moves `s`. `ref_binding(opt: &Option<String>)` uses explicit `Some(ref s)` — `s` is `&String`. `ergonomic_binding(opt: &Option<String>)` uses just `Some(s)` — match ergonomics automatically makes `s: &String` when matching on a reference. `ref_mut_binding(opt: &mut Option<String>)` uses `Some(ref mut s)` to mutably borrow the string inside the option.

Key patterns:
- `match &val { Some(s) => ... }` — ergonomics binds `s` as `&T`
- `Some(ref s)` — explicit reference binding
- `Some(ref mut s)` — explicit mutable reference binding
- `match val { Some(s) => ... }` — `s` is moved (owned match target)

## OCaml Approach

OCaml patterns always bind by reference to the GC heap — there is no move vs copy distinction:

```ocaml
let ref_binding opt = match opt with
  | Some s -> String.length s
  | None -> 0
(* s is always a reference to the string — no ref keyword needed *)
```

## Key Differences

1. **Explicit vs implicit**: Rust requires `ref` without ergonomics or when ergonomics do not apply; OCaml always binds by reference implicitly.
2. **Mutability**: Rust's `ref mut` creates a mutable reference binding — a precise distinction; OCaml uses `ref` cells for mutable state.
3. **Move vs borrow**: Rust's binding mode determines whether the matched value is moved or borrowed — critical for ownership; OCaml has no moves.
4. **Ergonomics scope**: Match ergonomics only apply at the outer level of the pattern when matching on a reference — inner patterns may still need explicit `ref`.

## Exercises

1. **Explicit vs ergonomic**: Write the same function twice — once using explicit `ref` and once relying on match ergonomics — verify they compile to the same behavior.
2. **Ref mut mutation**: Write `fn append_exclaim(v: &mut Vec<String>)` using `for s in v.iter_mut() { ... }` with `ref mut` inside the loop to mutate each string.
3. **Mixed modes**: Match on `&(String, &str)` — verify that `(ref a, b)` gives `a: &String, b: &&str` and that ergonomics alone gives `a: &String, b: &&str` without explicit `ref`.
