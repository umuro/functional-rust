# 572: ref Patterns and &

**Difficulty:** 2  **Level:** Beginner

Understand how `ref`, `&`, and match ergonomics interact — borrow inside patterns without fighting the borrow checker.

## The Problem This Solves

You have a `Vec<i32>` and you want to sum it. `values.iter()` gives you `&i32` references, and your closure gets `&&i32` — a reference to a reference. Adding `&&i32` values doesn't compile directly. You need to strip the outer reference.

More commonly: you match on a `Vec<String>`, destructure the first element to a `String`, and suddenly the whole vector is partially moved. You only wanted to *look* at it.

The confusion deepens because there are three overlapping tools for this: the `ref` keyword in patterns, the `&` pattern that dereferences, and *match ergonomics* (automatic `ref` insertion when matching a reference). They all exist for good historical and ergonomic reasons, and they all interact.

## The Intuition

When you match `&value` against `&T`, you're asking Rust to "open the reference" and give you what's inside. `let &x = &5` gives you `x: i32` — the dereference happens in the pattern.

When you use `ref x` in a pattern, you're telling Rust: "bind `x` as a *reference to* the matched value, don't move it." `let ref x = s` is equivalent to `let x = &s`.

Modern Rust has *match ergonomics*: when you match a `&T` value against a non-reference pattern, Rust inserts the `ref` automatically. `if let Some(s) = &opt` gives you `s: &String` — no explicit `ref` needed. This covers 90% of cases.

The cases where you still reach for `ref` explicitly: `let` bindings in non-reference contexts, or when you want clarity about what's happening.

## How It Works in Rust

```rust
// & in closure pattern — strip one reference from iter()
let values = vec![1, 2, 3, 4, 5];
let sum: i32 = values.iter().map(|&x| x).sum();
// values.iter() yields &i32; |&x| destructures it to i32

// ref in let binding — explicit borrow without moving
let s = String::from("hello");
let ref r = s;   // r: &String; s still owned
println!("r={} s={}", r, s);

// Match ergonomics — matching &Option<String> auto-borrows
let opt = Some(String::from("hello"));
if let Some(s) = &opt {  // s: &String — ref inserted automatically
    println!("borrowed: {}", s);
}
println!("opt still alive: {:?}", opt);  // not moved

// ref in slice pattern — explicit when needed
fn first_two_borrowed(v: &[String]) -> Option<(&str, &str)> {
    match v {
        [ref a, ref b, ..] => Some((a, b)),  // borrow elements, not move
        _                  => None,
    }
}

// ref mut — borrow first element mutably
fn increment_first(v: &mut [i32]) {
    if let [ref mut first, ..] = v {
        *first += 1;
    }
}
```

## What This Unlocks

- **Inspect collections without consuming them** — borrow fields and elements in patterns, leave the original intact.
- **Closure-friendly iteration** — `|&x|` in closures over `.iter()` output gives you the value directly.
- **Mutable targeted edits** — `ref mut` in a slice pattern lets you modify exactly one element without indexing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Borrows in patterns | N/A (GC, no ownership) | `ref x` borrows; `&x` dereferences |
| Mutable reference cell | `let x = ref v` (explicit cell) | `ref mut x` in pattern |
| Automatic borrowing | N/A | Match ergonomics: `&T` against non-ref pattern inserts `ref` |
| Iteration references | N/A | `iter()` yields `&T`; use `|&x|` or `.copied()` |
| `ref` in let | N/A | `let ref x = value` ≡ `let x = &value` |
