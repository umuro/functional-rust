# 184: Introduction to Free Monads

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Build programs as pure data structures that you can run multiple different ways — without changing the program.

## The Problem This Solves

Imagine writing a CLI tool that asks the user for input and prints responses. Easy enough. But then you need to test it. Suddenly you're faking stdin, redirecting stdout, or splitting your logic into awkward "pure core / impure shell" layers. The code that _does_ things (print, read) is tangled with the code that _decides_ things (what to print, what to do with input).

Here's what the tangled version looks like:

```rust
fn greet() {
    println!("What is your name?");  // side effect: baked in
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();  // side effect: baked in
    println!("Hello, {}!", name.trim());  // can't test without real IO
}
```

To test this you need real IO, or you need to parameterize every function with input/output streams. The testing tax grows with every function that touches the outside world. Every function with side effects is harder to compose, harder to reason about, harder to run in a different context (async, mock, replay).

The Free Monad pattern solves this by turning your program into a **data structure** — a tree of instructions. You describe what you _want_ to happen (print this, read a line, then do this with the result) without deciding _how_ it actually executes. Then you write separate interpreters: one for real IO, one for testing with fake inputs, one for logging, one for async. Same program description, different execution engines.

This is exactly how SQL works: you write `SELECT * FROM users` — that's data describing what you want. The database engine is the interpreter that decides how to actually retrieve it. The query doesn't care if it runs on PostgreSQL or SQLite. A Free Monad brings that same separation to your Rust programs.

## The Intuition

Think of a recipe. A recipe is a list of instructions: "add flour," "mix for 3 minutes," "bake at 180°C." The recipe itself is just data — it doesn't _do_ anything. You (the cook) are the interpreter. You could follow the recipe in a real kitchen, or you could simulate it in a cooking game, or trace it on paper to check the steps. The recipe stays the same; how it's executed depends on who's interpreting it.

A Free Monad is a recipe for a program. Each step in the recipe is an instruction in your `enum`. The instructions form a tree because each step says what to do _next_. "Print this message, then do _next_." "Read a line, then call _f_ with what you read."

Here's the core data structure from this example:

```rust
enum Console<A> {
    Pure(A),                                        // "I'm done, here's the result"
    Print(String, Box<Console<A>>),                 // "Print this, then do next"
    GetLine(Box<dyn FnOnce(String) -> Console<A>>), // "Read a line, pass it to f"
}
```

- `Pure(A)` — the program is finished. `A` is the final value.
- `Print(msg, next)` — print `msg`, then continue with `next`.
- `GetLine(f)` — get user input, feed it to `f` which decides what to do next.

The function `bind` (also called `flatMap` or `and_then`) is how you chain steps together. It says: "take this program, and when it finishes with a value, feed that value into this next function." It's what lets you write sequential logic — "do A, then do B with A's result, then do C."

## How It Works in Rust

**Step 1: Build the program as data**

```rust
fn greet_program() -> Console<String> {
    // bind takes: a program that produces A, and a function A -> program producing B
    // Result: a program that produces B
    bind(Console::print("What is your name?"), move |()| {
        bind(Console::get_line(), move |name: String| {
            bind(Console::print(&format!("Hello, {}!", name)), move |()| {
                Console::pure(name)  // done — return the name
            })
        })
    })
}
```

This just _builds_ the data structure. No printing happens. No reading happens. You get back a `Console<String>` value — a tree describing: print, read, print, return.

**Step 2: Write an interpreter**

The pure interpreter loops through the structure, feeds fake input, collects output:

```rust
fn interpret_pure(inputs: &[&str], prog: Console<String>) -> (Vec<String>, String) {
    let mut outputs = Vec::new();
    let mut input_idx = 0;
    let mut current = prog;

    loop {
        match current {
            Console::Pure(a) => return (outputs, a),           // done
            Console::Print(msg, next) => {
                outputs.push(msg);      // collect instead of printing
                current = *next;        // move to next step
            }
            Console::GetLine(k) => {
                let input = inputs.get(input_idx).unwrap_or(&"");
                input_idx += 1;
                current = k(input.to_string());  // feed fake input to continuation
            }
        }
    }
}
```

**Step 3: Run the same program different ways**

```rust
// Test it — no real IO needed
let (outputs, result) = interpret_pure(&["Alice"], greet_program());
assert_eq!(outputs, vec!["What is your name?", "Hello, Alice!"]);
assert_eq!(result, "Alice");

// To add real IO: write a different interpreter
// fn interpret_io(prog: Console<String>) -> String { ... }
// Same program, different execution.
```

**What breaks without this pattern:**

If you called `println!` and `stdin().read_line()` directly, your test would block waiting for real input. You'd have to mock the OS, run in a subprocess, or redesign your entire architecture. The Free Monad gives you testability for free.

## What This Unlocks

- **Swappable execution**: Write once, run in tests with fake IO, in production with real IO, with async runtimes, or with logging middleware — no code changes to the program itself.
- **Program introspection**: Because your program is data, you can analyze it before running it — count steps, validate operations, serialize it, or optimize it.
- **Effect isolation**: In real codebases you'll see this in database access layers (build a query tree, run against real or test DB), command processors (record commands as data, replay them), and anywhere you need the decoupling between "what to do" and "how to do it."

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Generic free monad | Works over any functor with HKTs | Must specialize per functor (no HKTs) |
| Closures in data | Natural — GC handles lifetime | Must `Box<dyn FnOnce>` + `'static` bounds |
| Chaining steps | `>>=` operator, reads naturally | Nested `bind(...)` calls — verbose |
| Interpreting | Recursive pattern match | Loop or recursion — same logic |
| Memory | GC reclaims nodes | Heap boxes freed on drop |
