# Hello World — Functions and Greetings

## Problem Statement
Implement a `greet` function that takes a name and returns a greeting string. The simplest possible functional program: a pure function from input to output.

## Learning Outcomes
- The most basic unit of functional programming: a pure function with no side effects
- How OCaml's string concatenation (`^`) maps to Rust's `format!` macro
- Type annotations in both languages for function signatures

## Rust Application
`greet(name: &str) -> String` uses `format!("Hello, {name}!")` — Rust's `format!` macro inlines the variable directly. The function is pure: same input always produces same output, no mutation, no I/O.

## OCaml Approach
`let greet (name : string) : string = "Hello, " ^ name ^ "!"` uses the `^` string concatenation operator. OCaml type annotations are optional (inferred) but shown here for clarity.

## Key Differences
1. **String building:** OCaml uses `^` for concatenation; Rust uses `format!` with inline variable syntax `{name}`
2. **Type annotations:** OCaml annotates inline `(name : string) : string`; Rust annotates in the signature `name: &str) -> String`
3. **Ownership:** Rust returns an owned `String` (heap-allocated); OCaml returns an immutable string value

## Exercises
1. Extend `greet` to accept a title (e.g., "Dr.", "Prof.") and produce "Hello, Dr. Smith!" — make the title optional using `Option<&str>`
2. Implement `greet_many` that takes a `Vec<&str>` and returns a `Vec<String>` using `.map()` on an iterator
3. Add a `farewell` function and combine both into a `Conversation` struct that records the greeting and farewell
