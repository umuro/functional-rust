# 568: @ Bindings in Patterns

**Difficulty:** 2  **Level:** Beginner

Match a pattern AND bind the matched value to a name — get both the test and the variable.

## The Problem This Solves

Here's the conflict: you want to match a range of values, but you also need the actual value inside the arm. Without `@`, you pick one or the other. You can write `1..=100 => "small"` and lose the value. You can write `x => if x >= 1 && x <= 100 { ... }` and lose the clean pattern syntax. You can't have both in the same arm.

The same problem appears with enum variants: you want to fire a log message that includes the whole `Event` struct, but also destructure its fields for the actual logic. Without `@`, you'd either bind the whole thing and access fields through `.x`, `.y`, or destructure and lose the original.

`@` bindings solve this: `n @ 1..=100` matches the range AND binds `n` to the actual value. `e @ Event::Click { x, y }` binds the whole event to `e` AND gives you `x` and `y` from the destructure.

## The Intuition

The `@` symbol reads as "at" — "bind the value *at* this position to this name, where the value also matches this pattern." It's glue between a name and a condition.

OCaml spells this `as`: `Click(x, y) as e when x > 0` — the matched value is available as `e`, and `x`, `y` are destructured. Rust uses `@` instead. The semantics are identical.

The practical rule: reach for `@` when you need to log, debug, forward, or compare the whole matched thing, while also branching on its shape or value. `n @ 0 => ("zero", n)` — a zero cost way to have your cake and eat it too.

## How It Works in Rust

```rust
// @ with range — bind the value AND test the range
fn categorize(n: i32) -> (&'static str, i32) {
    match n {
        x @ 0           => ("zero",           x),
        x @ 1..=100     => ("small positive", x),
        x @ 101..=1000  => ("medium",         x),
        x               => ("large",          x),  // catch-all, still bound
    }
}

// @ with enum + guard — bind whole event AND destructure fields
fn handle(ev: &Event) -> String {
    match ev {
        // e is the whole Event, x and y are destructured from it
        e @ Event::Click { x, y } if *x > 0 && *y > 0 =>
            format!("valid click {:?}", e),

        Event::Click { .. } => "invalid click".into(),

        // @ with character range — bind AND test
        Event::Key(c @ 'a'..='z') => format!("lower: {}", c),
        Event::Key(c @ 'A'..='Z') => format!("upper: {}", c),
        Event::Key(c)             => format!("other: {}", c),

        Event::Resize(w, h) => format!("resize {}x{}", w, h),
    }
}
```

## What This Unlocks

- **Log and branch simultaneously** — bind the entire value for debug output while also destructuring for logic.
- **Range dispatch with the actual value** — `n @ 1..=100` gives you `n` in the arm body; no need to test the condition again.
- **Guard-ready complex patterns** — combine `@` with `if` guards: `x @ value if predicate(x)` for rich, readable conditions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Syntax | `pattern as name` | `name @ pattern` |
| With guard | `Click(x,y) as e when x > 0` | `e @ Event::Click{x,y} if *x > 0` |
| Bind + range | No native range patterns | `n @ 1..=100` |
| Bind + destructure | `(Some x) as opt` | `opt @ Some(x)` |
| Position | Name comes after `as` | Name comes before `@` |
