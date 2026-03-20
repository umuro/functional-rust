📖 **[View on hightechmind.io →](https://hightechmind.io/rust/572-pattern-ref-patterns)**

---

# Ref Patterns
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Before match ergonomics (Rust 2018), matching on a reference required explicit `ref` keywords in patterns to borrow rather than move the matched value. This was verbose and confusing, particularly for newcomers. Match ergonomics automated most cases, but explicit `ref` and `ref mut` still appear in older code, in code that must be explicit for clarity, and in specific contexts where ergonomics do not apply. Understanding both the old explicit style and the modern ergonomic style is essential for reading existing Rust codebases.

## Learning Outcomes

- How `Some(ref s)` explicitly borrows `s` from a match on `&Option<String>`
- How match ergonomics (`Some(s)`) automatically infers `s: &String` when matching on `&Option<T>`
- How `Some(ref mut s)` creates a mutable reference binding
- How `ref` in struct patterns works: `Point { ref x, ref y }`
- Why understanding explicit `ref` is needed for reading pre-2018 Rust code

## Rust Application

`inspect(opt: &Option<String>)` uses `Some(ref s) => s.len()` — explicit `ref`, `s: &String`. `inspect_modern(opt: &Option<String>)` uses `Some(s) => s.len()` — ergonomics, same behavior. `append_exclaim(opt: &mut Option<String>)` uses `Some(ref mut s) => s.push('!')` — explicit `ref mut` needed for mutable modification. Both styles produce identical compiled code — `ref` is purely a syntactic annotation.

Key patterns:
- `Some(ref s)` — explicit borrow in pattern
- `Some(s)` on `&Option<T>` — automatic borrow via ergonomics
- `Some(ref mut s)` — explicit mutable borrow in pattern
- `let ref x = val;` — ref binding in `let`

## OCaml Approach

OCaml always binds pattern variables by reference to the GC heap — there is no `ref`/`ref mut` distinction in patterns. Mutation requires `ref` cells in the value, not in the pattern:

```ocaml
let inspect opt = match opt with
  | Some s -> String.length s  (* s is always a reference *)
  | None -> 0
```

## Key Differences

1. **Explicit vs implicit**: Rust requires explicit `ref` (or relies on ergonomics) to borrow in patterns; OCaml always borrows implicitly.
2. **`ref mut`**: Rust's `ref mut` creates a mutable reference — enables modifying the matched value in-place; OCaml uses mutable record fields or `ref` cells for the same effect.
3. **Historical context**: Pre-2018 Rust code uses `ref` extensively; modern code relies on ergonomics; OCaml code never used `ref` in patterns.
4. **Mental model**: Rust patterns explicitly model ownership and borrowing; OCaml patterns model structural decomposition without ownership concerns.

## Exercises

1. **Pre-ergonomics rewrite**: Take `inspect_modern` and rewrite it using explicit `ref` keywords — verify both versions compile and produce the same output.
2. **Ref mut tree**: Write a function `fn negate_first(v: &mut Vec<i32>)` using `if let Some(ref mut first) = v.first_mut() { *first = -*first; }`.
3. **Struct ref pattern**: Match on `&Point { x, y }` using both explicit `ref x, ref y` and ergonomic `x, y` — verify both bind `x: &i32, y: &i32`.
