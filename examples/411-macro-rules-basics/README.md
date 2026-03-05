# 411: macro_rules! Declarative Macros

**Difficulty:** 3  **Level:** Advanced

Write code that writes code — match on syntax patterns and expand them at compile time, with zero runtime cost.

## The Problem This Solves

Some code patterns can't be abstracted with functions. `assert_eq!(a, b)` prints the *source text* of both expressions on failure — that's not possible with a function, because a function only sees values, not the code that produced them. `println!("{}", x)` accepts variable argument counts — that's not possible with Rust's type system without variadics (which Rust doesn't have). `vec![1, 2, 3]` constructs a `Vec` with literal syntax — a function call can't achieve this ergonomics.

Macros operate on the **syntax tree** before the compiler processes types and values. They transform one piece of syntax into another. The input is tokens; the output is tokens; the result is compiled normally. This is why `assert_eq!` can display source text, why `println!` can take any number of arguments, and why `vec![]` feels like a language feature.

`macro_rules!` is Rust's declarative macro system. You write pattern-matching rules: if the invocation matches this pattern, expand to that code. It's hygienic (identifiers in the macro don't leak into caller scope), it's zero-cost (all work happens at compile time), and it composes with the rest of the type system.

## The Intuition

A `macro_rules!` macro is a set of match arms, like a `match` expression but for *syntax* rather than values. Each arm has a pattern (what to match in the invocation) and a template (what to expand to). The compiler tries each arm in order and expands the first match.

Fragment specifiers (`$name:expr`, `$name:ident`, `$name:ty`) capture different kinds of syntax. The captured fragments are substituted into the template. Multiple arms let one macro name handle different call signatures — like overloaded functions, but for syntax.

## How It Works in Rust

```rust
// Multiple arms: handles with or without format args
macro_rules! log_info {
    ($msg:expr) => {
        println!("[INFO] {}", $msg)
    };
    ($fmt:expr, $($arg:expr),*) => {
        println!(concat!("[INFO] ", $fmt), $($arg),*)
    };
}

// Creates a HashMap with literal syntax
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            let mut m = std::collections::HashMap::new();
            $(m.insert($k, $v);)*  // repeat for each k => v pair
            m
        }
    };
}

// min of any number of values
macro_rules! min_of {
    ($a:expr) => { $a };  // base case
    ($a:expr, $($rest:expr),+) => {  // recursive case
        {
            let rest_min = min_of!($($rest),+);
            if $a < rest_min { $a } else { rest_min }
        }
    };
}

fn main() {
    log_info!("Application started");
    log_info!("User {} logged in at port {}", "Alice", 8080);

    let m = map! {
        "one" => 1,
        "two" => 2,
        "three" => 3,  // trailing comma: $(,)? handles it
    };
    println!("{:?}", m);

    println!("min(3,7,1,9) = {}", min_of!(3, 7, 1, 9));
}
```

**Fragment specifiers quick reference:**

| Specifier | Matches |
|-----------|---------|
| `expr` | Any expression: `1 + 2`, `"hello"`, `foo()` |
| `ident` | An identifier: `my_var`, `println` |
| `ty` | A type: `i32`, `Vec<String>`, `Option<T>` |
| `pat` | A pattern: `Some(x)`, `Ok(v)`, `1..=5` |
| `literal` | A literal: `42`, `"hello"`, `3.14` |
| `block` | A block: `{ stmt; expr }` |
| `stmt` | A statement |
| `tt` | A single token tree (any single token or `()[]{}` group) |

## What This Unlocks

- **Variadic APIs** — `vec![]`, `println!`, `assert_eq!`, `format!` — all use macros to achieve variable argument count, impossible with regular functions.
- **Compile-time code generation** — `map!{}`, `impl_from_int!{}` — generate repetitive boilerplate at compile time without runtime cost.
- **Domain-specific assertion utilities** — `check_eq!`, custom `debug!` macros with context — richer diagnostics than a function could provide.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Metaprogramming | PPX (preprocessing extensions) — complex, separate tool | `macro_rules!` — built-in, hygienic, integrated with compiler |
| Variadic | Lists: `log "INFO" "msg"` — no variadic syntax | Macros: `log_info!("msg", arg1, arg2)` — variadic via repetition patterns |
| Hygiene | PPX is not hygienic by default | `macro_rules!` is hygienic — introduced identifiers don't leak |
| Compile-time | PPX runs before compilation | `macro_rules!` expands during compilation, same pipeline |
