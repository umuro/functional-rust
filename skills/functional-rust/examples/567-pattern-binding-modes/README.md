# 567: Binding Modes: ref, ref mut

**Difficulty:** 2  **Level:** Beginner

Control how pattern bindings take ownership — borrow instead of move where needed.

## The Problem This Solves

Rust's ownership rules make destructuring a minefield if you're not careful. You destructure a `Vec<String>` inside a struct, and suddenly the struct is partially moved — unusable afterward. You match on a `String` field to inspect it, and the value is consumed. You wanted to look, not own.

The other half of the problem is mutation: you want to modify the first element of a slice in-place without removing it. Without `ref mut`, you'd have to reach into the slice by index and hope you got it right.

Both `ref` and `ref mut` are the escape hatch: they tell the pattern "bind this as a reference, don't move it." In modern Rust, *match ergonomics* (matching through `&T`) handles the common cases automatically, but `ref` remains essential when you're in a `let` binding outside a reference context, or when you need explicit control.

## The Intuition

Default pattern bindings move values. `let v = s` moves `s` into `v`. In patterns, `let Wrapper(v) = w` would move the inner value out, consuming `w`. Adding `ref` flips the semantic: `let Wrapper(ref v) = w` borrows the inner value — `v` is a `&InnerType`, and `w` is still intact.

Think of `ref` as writing `&` on the right side of a binding. `let ref r = s` is equivalent to `let r = &s`. It's the same borrow, just spelled differently to fit inside a pattern.

`ref mut` is the mutable version: borrow and allow mutation through the reference. The `*first *= 2` in a slice pattern does the actual mutation through the mutable reference.

## How It Works in Rust

```rust
// ref in let binding — explicit borrow
let s = String::from("hello");
let ref r = s;      // r: &String; s is still owned
println!("{} {}", r, s);  // both usable

// ref in slice pattern — borrow head and tail separately
fn first_and_rest(v: &[String]) -> Option<(&str, &[String])> {
    match v {
        [ref head, ref rest @ ..] => Some((head, rest)),
        [] => None,
    }
}

// ref mut — borrow first element for mutation
fn double_first(v: &mut [i32]) {
    if let [ref mut first, ..] = v {
        *first *= 2;  // mutate through the mutable reference
    }
}

// Modern ergonomics: matching &T auto-borrows in many cases
let opt = Some(String::from("hello"));
if let Some(s) = &opt {  // s: &String, no explicit ref needed
    println!("borrowed: {}", s);
}
println!("opt still alive: {:?}", opt);

// Tree traversal — auto-deref through Box works without ref
fn sum(t: &Tree) -> i32 {
    match t {
        Tree::Leaf           => 0,
        Tree::Node(v, l, r) => v + sum(l) + sum(r),  // l and r auto-deref
    }
}
```

## What This Unlocks

- **Inspect-without-consuming** — borrow fields in a match without moving the original value out of scope.
- **In-place mutation via pattern** — `ref mut` lets you modify elements in place through slice and tuple patterns.
- **Composable borrows** — return references into a container (`&[String]`) without copying.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Binding mode | Always by value (GC handles memory) | By value (move) by default; `ref` to borrow |
| Mutable binding | `let x = ref v` (explicit mutable cell) | `ref mut x` in pattern — borrows mutably |
| Auto-deref in match | N/A (no ownership) | Match ergonomics: matching `&T` borrows automatically |
| Modify in place | Mutable record field update | `ref mut` + dereference |
| Slice head/tail | `x :: xs` (list cons) | `[ref head, ref rest @ ..]` |
