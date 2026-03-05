# 573: let-else for Early Return

**Difficulty:** 2  **Level:** Beginner

Unwrap a pattern or bail out — keep the happy path flat without nesting.

## The Problem This Solves

Parsing and validation code suffers from rightward drift. You check if a config line has the right format — that's an `if let`. Inside that, you parse the port — another `if let`. Inside that, you validate the range — another `if let`. By the third check you're two or three levels deep, and the actual logic is buried at the center of a pyramid.

The classic fix is `match` with an explicit `return` or `continue` in the `None`/`Err` arm. But that's verbose: four lines for what should be one. Another option is `.ok_or()` and `?` chaining, which works when you're in a `Result`-returning context — but not in loops, not in `void` functions, not when the fallback is `continue` or `break`.

`let-else` is the dedicated solution for the "unwrap or bail" pattern. One line, flat code, no nesting.

## The Intuition

`let Some(x) = opt else { return; };` reads as: "let `x` be the inner value of `opt`, otherwise (in the else branch) do this and diverge." The `else` block *must* diverge — it has to `return`, `break`, `continue`, or `panic!`. After the `let-else`, `x` is in scope and you're guaranteed it matched.

Compare to `if let`:
```rust
if let Some(x) = opt {
    // x is only in scope here
    use(x);
}
```
With `let-else`, `x` is in scope *after* the statement, not inside a block. That's the key difference. The happy path stays at the top level; failure is handled locally and immediately.

OCaml handles this via `let*` (Option.bind chaining) or explicit match. The `let-else` idiom is Rust-specific and was stabilized in Rust 1.65 (2022).

## How It Works in Rust

```rust
// Parse a "host:port" string — bail on any malformation
fn parse_config(line: &str) -> Option<(String, u16)> {
    let parts: Vec<&str> = line.split(':').collect();

    // Destructure into exactly two parts, or return None
    let [host, port_str] = parts.as_slice() else { return None; };

    // Parse the port, or return None
    let Ok(port) = port_str.parse::<u16>() else { return None; };

    Some((host.to_string(), port))  // flat — no nesting
}

// let-else in a loop — use continue instead of return
fn sum_valid(inputs: &[&str]) -> i32 {
    let mut total = 0;
    for &s in inputs {
        let Ok(n) = s.parse::<i32>() else {
            eprintln!("skip: {}", s);
            continue;  // diverge with continue, not return
        };
        total += n;  // n is in scope here, at the loop's top level
    }
    total
}

// let-else with enum — skip non-admin users
fn admin_name(users: &[User], id: u64) -> Option<&str> {
    let Some(u) = users.iter().find(|u| u.id == id) else { return None; };
    if !u.admin { return None; }
    Some(&u.name)
}
```

## What This Unlocks

- **Flat validation pipelines** — chain multiple `let-else` statements at the same indentation level.
- **Loop filtering** — `let ... else { continue; }` is the cleanest way to skip malformed iterations.
- **Post-binding scope** — the bound variable is available for the rest of the function, not locked in a nested block.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bail-on-none | `let* x = opt in ...` (bind chains) | `let Some(x) = opt else { return; };` |
| Flat happy path | Via monadic `let*` chaining | Via sequential `let-else` statements |
| In loops | Recursive/List.filter | `let ... else { continue; }` |
| Scope of binding | Inside `in` block | After the `let-else` statement |
| Divergence required | N/A | Else branch must diverge (return/break/continue/panic) |
