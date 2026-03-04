# 581: Result Matching Idioms

**Difficulty:** 2  **Level:** Beginner

Handle success and failure explicitly with `Result<T, E>` — propagate errors with `?`, match when you need both paths.

## The Problem This Solves

Exception-based error handling is invisible. A function throws — or doesn't — and the caller has no way to tell from the signature. You either wrap everything in try/catch (verbose, often forgotten) or let exceptions bubble (dangerous). Forgetting to handle an error is always valid code.

Go's two-return-value convention (`value, err := f()`) makes errors visible but doesn't force you to check them. You can write `value, _ := f()` and silently discard the error.

Rust's `Result<T, E>` is the "you must deal with this" type. A function that can fail returns `Result`. Ignoring it triggers a compiler warning. Accessing the value without checking requires an explicit `unwrap()` — a deliberate choice, not an oversight.

## The Intuition

`Result<T, E>` has two variants: `Ok(T)` (success, carries the value) and `Err(E)` (failure, carries the error). Like `Option`, you match on it or use combinators. Unlike `Option`, the error carries information.

The `?` operator is the ergonomic key: it means "if this is `Err`, return the `Err` from my function immediately; if it's `Ok`, give me the inner value." It's shorthand for the match-and-early-return pattern, but it composes. Chain five operations that might fail — each gets a `?` — and the error from whichever one fails propagates automatically.

OCaml spells `?` as `let*` with `Result.bind`. The logic is identical; the syntax differs.

The real power is when `collect()` works on `Vec<Result<T, E>>`: it either gives you `Ok(Vec<T>)` (all successes) or `Err(E)` (first failure). One call, no loop.

## How It Works in Rust

```rust
#[derive(Debug)]
enum AppErr { Parse(ParseIntError), Range(i32), DivZero }

fn parse(s: &str) -> Result<i32, AppErr> {
    s.parse().map_err(AppErr::Parse)  // convert ParseIntError to AppErr
}

fn validate(n: i32) -> Result<i32, AppErr> {
    if n >= 1 && n <= 100 { Ok(n) } else { Err(AppErr::Range(n)) }
}

// ? chains — each step propagates errors automatically
fn process(s: &str) -> Result<i32, AppErr> {
    let n = parse(s)?;    // Err(ParseError) if not a number
    let v = validate(n)?; // Err(Range) if out of bounds
    Ok(v * v)             // success
}

// match on Result — explicit handling
match process("42") {
    Ok(v)  => println!("got {}", v),
    Err(e) => println!("failed: {:?}", e),
}

// Combinators — like Option
let r: Result<i32, _> = Ok(5);
r.map(|x| x * 2);  // Ok(10)
r.map_err(|e| format!("error: {:?}", e));  // transforms the error

// collect() on Vec<Result<T, E>> — all-or-nothing
let inputs = vec!["1", "2", "3"];
let nums: Result<Vec<i32>, _> = inputs.iter().map(|s| parse(s)).collect();
// Ok([1, 2, 3]) — all succeed

let mixed = vec!["1", "x", "3"];
let result: Result<Vec<i32>, _> = mixed.iter().map(|s| parse(s)).collect();
// Err(Parse(...)) — first failure wins
```

## What This Unlocks

- **Visible error contracts** — the function signature tells you it can fail, and what kind of error to expect.
- **Composable error propagation** — `?` threads errors through call chains without explicit match-and-return at each step.
- **Batch processing** — `collect::<Result<Vec<_>, _>>()` gives you all-or-nothing semantics on collections.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type | `('a, 'e) result` | `Result<T, E>` |
| Variants | `Ok x`, `Error e` | `Ok(x)`, `Err(e)` |
| Chain/bind | `Result.bind r f` or `let*` | `r.and_then(f)` or `?` |
| Early return on error | Via `let*` monad | `?` operator |
| Map error | `Result.map_error f r` | `r.map_err(f)` |
| Collect | Manual fold | `collect::<Result<Vec<_>, _>>()` |
