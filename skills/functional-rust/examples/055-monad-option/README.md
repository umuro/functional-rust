# 055: Option Monad

**Difficulty:** ⭐⭐  **Level:** Intermediate

Chain fallible operations on `Option` without nesting — using `and_then`, the monadic bind you already know.

## The Problem This Solves

You're writing code where each step might produce nothing. Maybe you're looking up a key in a map, then using that value to look up another key, then checking a condition on the result. Each step returns `Option<T>`. The natural instinct is to nest:

```rust
// Without and_then: the pyramid of doom
fn find_user_docs(env: &HashMap<&str, &str>, paths: &HashMap<&str, Vec<&str>>) -> Option<String> {
    match env.get("HOME") {
        None => None,
        Some(home) => match paths.get(home.to_owned()) {
            None => None,
            Some(dirs) => {
                if dirs.contains(&"documents") {
                    Some("documents found".to_string())
                } else {
                    None
                }
            }
        }
    }
}
```

Three levels of nesting for three fallible steps. Add two more steps and you're off the right edge of your screen. The logic is buried inside the matching. It's hard to read, hard to refactor, and easy to miss a case.

The real pain: `None` propagates identically through every step — if any step fails, the whole chain fails. You're writing the same boilerplate over and over to express a simple idea: "do this, then this, then this, and if anything is missing, bail out."

The Option monad exists to solve exactly that pain.

## The Intuition

Think of a production line. Each station takes a part, does something with it, and passes it to the next station — or sends a "rejected" signal that skips all remaining stations automatically.

`Option` is that production line. `None` is the rejection signal. Once you get `None`, every subsequent step is skipped and `None` comes out the end. You don't have to check at every station; the line handles it.

`and_then` is the conveyor belt connecting stations. It says: "if I have a value, pass it to the next function; if I have `None`, skip everything and propagate `None`."

```rust
// The idea in code:
Some(value)
    .and_then(|v| do_something(v))   // runs if Some
    .and_then(|v| do_more(v))        // runs if still Some
    .and_then(|v| final_step(v))     // runs if still Some
// Result: Some(final) or None (wherever the line broke)
```

This is a **monad**: a pattern for chaining operations that might fail (or have effects), without nesting. The word sounds academic but the behavior is familiar. You already use it every time you write `?` in a function that returns `Option`.

`?` is literally `and_then` with early return. They're the same thing written differently.

## How It Works in Rust

**Start simple: one fallible step**

```rust
fn safe_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 { None } else { Some(x / y) }
}

let result = safe_div(10, 2); // Some(5)
let result = safe_div(10, 0); // None
```

**Chain two steps with `and_then`**

```rust
fn safe_sqrt(x: i32) -> Option<f64> {
    if x < 0 { None } else { Some((x as f64).sqrt()) }
}

// and_then: "if I have a value, run this function on it"
// The function must return Option — it decides whether to continue or bail
let result = safe_div(100, 4)          // Some(25)
    .and_then(|q| safe_sqrt(q))        // Some(5.0)
    .map(|r| r as i32);                // Some(5)
                                       // (map is for infallible transforms)

let result = safe_div(100, 0)          // None (divide by zero)
    .and_then(|q| safe_sqrt(q))        // skipped — None propagates
    .map(|r| r as i32);                // skipped — still None
```

**The same logic with `?` operator**

```rust
fn compute(a: i32, b: i32) -> Option<i32> {
    let q = safe_div(a, b)?;   // ? means: unwrap or return None immediately
    let r = safe_sqrt(q)?;     // same
    Some(r as i32)
}
```

`?` and `.and_then()` chains are identical in what they do. `?` is just easier to read when you have many steps. Both are the Option monad in action.

**A real-world lookup chain**

```rust
fn find_user_docs(env: &HashMap<&str, &str>, paths: &HashMap<&str, Vec<&str>>) -> Option<String> {
    env.get("HOME")                          // Option<&&str>
        .and_then(|home| paths.get(home.to_owned())) // Option<&Vec<&str>>
        .and_then(|dirs| {
            if dirs.contains(&"documents") {
                Some("documents found".to_string())
            } else {
                None
            }
        })
}
// If HOME is missing → None. If path not found → None. If no documents → None.
// No nested matches. Three steps, three lines.
```

## What This Unlocks

- **Flat fallible pipelines.** Chain 5 steps that each might fail without any nesting — your logic reads top-to-bottom, not inside-out.
- **Interchangeability with `?`.** Write a chain with `.and_then()` while designing the logic, switch to `?` syntax for the final code. They compile to the same thing.
- **Pattern recognition across codebases.** Once you see `and_then` as "monadic bind for Option," you'll immediately understand `Result::and_then`, `Iterator::flat_map`, and `Future::and_then` — they're the same pattern applied to different types.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Monadic bind | `Option.bind` / `>>=` operator | `Option::and_then` |
| Do-notation sugar | Not built in (use `let*` in newer OCaml) | `?` operator |
| `None` propagation | Automatic in bind chain | Automatic in `and_then` / `?` |
| Value ownership | Shared (immutable GC'd values) | `and_then` consumes the `Option` |
| Naming | `bind`, `return` (theory names) | `and_then`, `Some` (practical names) |
