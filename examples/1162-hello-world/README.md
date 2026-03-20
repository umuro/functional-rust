# Example 1162: Hello World — Functions and Greetings

**Difficulty:** ⭐
**Category:** Functions & Basics
**OCaml Source:** `let greet (name : string) : string = "Hello, " ^ name ^ "!"`

## Problem Statement

Implement a `greet` function that takes a name and returns a formatted greeting string. This is intentionally the simplest possible functional program: a pure function with one input and one output, no mutable state, no I/O side effects, and no complex data structures. Its purpose is to introduce the foundational concepts of function definition, string construction, and type annotations before moving on to higher-order functions and richer data types.

## Learning Outcomes

- How to define a named function in both OCaml and Rust — the structural parallel between `let greet name = ...` and `fn greet(name: &str) -> String`
- How OCaml's `^` string concatenation operator maps to Rust's `format!` macro, and why the two languages use fundamentally different string construction models
- How type annotations are written in each language: OCaml annotates inline with `(name : string) : string`; Rust annotates in the signature with `name: &str) -> String`
- The difference between borrowed string data (`&str`) and owned heap-allocated strings (`String`) in Rust, and why `greet` returns the owned type
- What "pure function" means concretely: same input always produces same output, no observable side effects, no mutation — a property this trivial example makes immediately testable

## OCaml Approach

In OCaml the function is defined with `let greet (name : string) : string = "Hello, " ^ name ^ "!"`. The `^` operator concatenates strings and returns a new string value. Type annotations are optional because the compiler infers them from the literals and operator, but they are written here for clarity. The function body is a single expression — no `return` keyword, no statement terminator. OCaml strings are immutable byte sequences; string operations always produce new values.

## Rust Application

In Rust the function is `pub fn greet(name: &str) -> String { format!("Hello, {name}!") }`. The `format!` macro is the idiomatic way to construct strings from mixed content; it avoids the intermediate allocations that chaining `+` on `String` values would produce. The parameter type `&str` accepts both string literals and references to owned `String` values without requiring the caller to transfer ownership. The return type is `String` (owned, heap-allocated) because the function creates a new string that did not exist before — it must own the allocation it returns.

## Key Differences

1. **String construction:** OCaml uses the `^` infix operator to concatenate string values directly; Rust uses the `format!` macro with inline variable interpolation `{name}`, which is more readable for multi-part strings and avoids intermediate heap allocations.
2. **String types:** OCaml has a single `string` type (mutable byte array since OCaml 4.06, but treated as immutable in practice); Rust distinguishes between `&str` (borrowed slice, no allocation) and `String` (owned heap value), and the distinction affects every function signature that touches strings.
3. **Type annotations:** OCaml places annotations inside the parameter list and after the closing parenthesis — `(name : string) : string`; Rust places them after the parameter name with a colon and uses `->` for the return type — `name: &str) -> String`.
4. **Return syntax:** OCaml function bodies are expressions; the last expression is the return value with no keyword needed. Rust also treats the last expression as the return value when there is no semicolon — `format!(...)` without a trailing `;` is the idiomatic form.

## Exercises

1. Extend `greet` to accept an optional title so that `greet_with_title(Some("Dr."), "Smith")` returns `"Hello, Dr. Smith!"` and `greet_with_title(None, "Smith")` returns `"Hello, Smith!"`. Use `Option<&str>` for the title parameter and handle both cases with `match` or `if let`.
2. Implement `greet_many` that takes a `Vec<&str>` of names and returns a `Vec<String>` of greeting strings. Write it using `.iter().map(|name| greet(name)).collect()` and verify it produces the same results as calling `greet` individually on each name.
3. Add a `farewell(name: &str) -> String` function and combine both into a `Conversation` struct that stores the greeting and farewell as fields. Implement a `Display` trait for `Conversation` so `println!("{conv}")` prints both lines. Write tests covering the full struct.
