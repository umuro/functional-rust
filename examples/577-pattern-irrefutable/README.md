# 577: Irrefutable vs Refutable Patterns

**Difficulty:** 2  **Level:** Beginner

Some patterns always match; others might not — and Rust's syntax reflects the difference.

## The Problem This Solves

Rust distinguishes between patterns that *always* succeed (irrefutable) and patterns that *might* fail (refutable). This distinction determines where each kind of pattern is allowed. `let` bindings require irrefutable patterns. `if let`, `while let`, and `match` arms accept refutable ones. Mixing them up is a compile error with a helpful message, but understanding the distinction from the start prevents the confusion entirely.

Irrefutable patterns are everywhere: every `let` statement, every function parameter, every `for` loop. They're so common you may not think of them as "patterns" at all. Refutable patterns — `Some(v)`, `Ok(x)`, variant names — can fail to match, so they need a context that handles the failure.

## The Intuition

An irrefutable pattern is a guarantee: no matter what value you give it, it will match. `let x = expr` always works — `x` can hold any value. `let (a, b, c) = (1, 2, 3)` always works — a 3-tuple always has three elements. A struct destructure in a `let` always works — the struct always has those fields.

A refutable pattern might say "no": `Some(v)` doesn't match `None`. That's why you need `if let` — it provides an else branch for when the pattern doesn't match. Using a refutable pattern in a plain `let` is a compile error because there's no way to handle the "doesn't match" case.

## How It Works in Rust

1. **Irrefutable `let`** — `let x = 42; let (a, b) = (1, 2); let Point { x, y } = p;` — always match, no `if` needed.
2. **Irrefutable function params** — `fn add((a, b): (i32, i32)) -> i32 { a + b }` — tuple destructuring in the parameter position.
3. **Irrefutable `for`** — `for (n, ch) in &pairs` — destructures each element; works because every pair is a 2-tuple.
4. **Refutable `if let`** — `if let Some(v) = opt { ... }` — handles the case where it doesn't match.
5. **Refutable `while let`** — `while let Some(t) = stack.pop() { ... }` — loops until the pattern fails to match.

## What This Unlocks

- Destructure structs, tuples, and pairs directly in `let` and `for` without wrapping in `match`.
- Know immediately which syntax to reach for: `let` for things that always match, `if let`/`match` for things that might not.
- Write idiomatic destructuring in function signatures instead of extracting fields manually inside the body.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Irrefutable let | `let (a, b) = pair` — always works | Same: `let (a, b) = pair` |
| Refutable patterns | `match opt with Some v -> ... \| None -> ...` | `if let Some(v) = opt { ... }` or `match` |
| For loop destructuring | `List.iter (fun (n, ch) -> ...)` | `for (n, ch) in &pairs` — irrefutable destructuring |
| Refutable in let | Compiler warns of non-exhaustive; may generate exception | Compile error: "refutable pattern in local binding" |
