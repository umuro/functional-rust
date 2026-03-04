# 058: FlatMap/Bind Chains

**Difficulty:** 2  **Level:** Intermediate

Build long chains of fallible operations where the first `None` or `Err` short-circuits everything.

## The Problem This Solves

Some operations are inherently sequential and dependent: to find a user's manager, you first look up the user, then their department, then the manager in that department. Each step can fail (not found). You can't do step 3 without the result of step 2.

In imperative code, this becomes nested conditionals: `if user: if dept: if manager:`. Three levels deep for three lookups, five levels for five. The actual logic — what you're *computing* — gets lost in the nesting. It's the classic "pyramid of doom."

`and_then` (also known as `bind` or `flatMap`) is the functional solution: each step in the chain receives the previous step's value and decides whether to continue. The first `None` or `Err` short-circuits the entire chain. No nesting, no intermediate `if` checks, no explicit early returns.

## The Intuition

You've seen this pattern with JavaScript Promises: `fetch(url).then(parse).then(validate).catch(handle)`. Each `.then()` receives the previous resolved value; if any step rejects, it skips to `.catch()`. `and_then` is exactly this, but for `Option`/`Result`, synchronously.

The mental model: **`and_then` threads a value through a pipeline of functions that can each say "I give up" by returning `None` or `Err`.** If any step gives up, the entire computation gives up. You only need to handle failure once, at the end.

`and_then` vs `map`: if the next function can fail, use `and_then`. If it can't fail, use `map`. The difference is in the return type of the function you pass.

## How It Works in Rust

```rust
// Database-like lookup chain with ? operator
fn find_manager_dept_name(user_id: u32, users: &[User], depts: &[Dept]) -> Option<String> {
    let user = users.iter().find(|u| u.id == user_id)?;   // None if not found
    let dept = depts.iter().find(|d| d.id == user.dept_id)?; // None if not found
    let manager = users.iter().find(|u| u.id == dept.mgr_id)?; // None if not found
    Some(format!("{}'s manager is {} in {}", user.name, manager.name, dept.name))
}

// Equivalent with and_then chain (no ? operator)
fn find_manager_chain(user_id: u32, users: &[User], depts: &[Dept]) -> Option<String> {
    users.iter().find(|u| u.id == user_id)
        .and_then(|user| depts.iter().find(|d| d.id == user.dept_id)
            .map(|dept| (user, dept)))
        .and_then(|(user, dept)| users.iter().find(|u| u.id == dept.mgr_id)
            .map(|mgr| format!("{}'s manager is {} in {}", user.name, mgr.name, dept.name)))
}

// Computation with bounds checking — any step exceeding 100 stops the chain
fn compute() -> Option<i32> {
    Some(0)
        .and_then(|a| step_add(10, a))   // 0 + 10 = 10 ✓
        .and_then(|a| step_mul(3, a))    // 10 * 3 = 30 ✓
        .and_then(|a| step_add(20, a))   // 30 + 20 = 50 ✓
        .and_then(|a| step_add(40, a))   // 50 + 40 = 90 ✓ → Some(90)
}

// One step exceeds limit → whole computation returns None
let r = Some(50).and_then(|a| step_mul(3, a));  // 150 > 100 → None
```

## What This Unlocks

- **Lookup chains:** Multi-table database lookups, config file traversal, deeply nested JSON access — all expressed as linear, readable chains.
- **Data processing pipelines:** parse → extract field → validate → transform, each step explicit and each failure handled once at the end.
- **Bounded accumulation:** Sequences of steps with global invariants (e.g., "total must not exceed X") that short-circuit cleanly without global state.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bind operator | `>>=` (custom infix: `let (>>=) m f = ...`) | `.and_then(f)` (method on `Option`/`Result`) |
| Style | `some_val >>= fun x -> next x >>= fun y -> ...` | `some_val.and_then(\|x\| next(x)).and_then(\|y\| ...)` |
| ? equivalent | `let* x = expr in ...` (OCaml 4.08+ binding operators) | `let x = expr?;` |
| Short-circuit | None propagates through `>>=` automatically | None/Err propagates through `and_then` / `?` |
| Readability | Infix operator reads naturally left-to-right | Method chain also reads left-to-right |
| Multiple return types | Polymorphic, any monad | `Option` and `Result` have separate `and_then` |
