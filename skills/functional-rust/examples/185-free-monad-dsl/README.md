# 185: Free Monad DSL

**Difficulty:** Expert  **Level:** 4

Build a full domain-specific language (DSL) for console interactions using a Free Monad — with a menu, branching logic, and an Exit operation — all testable without touching real IO.

## The Problem This Solves

You're building an interactive CLI. It has a menu, branching paths, and can exit cleanly. But your interactive logic has a problem: it's wired directly to stdin and stdout. Testing the "choose option 1, enter a name, get greeted" path means automating terminal input or writing fragile subprocess tests. Testing the "choose option 2, exit" path is even harder — the program exits and you have to inspect the exit code from outside.

The deeper problem: the _decisions_ (what the menu shows, what each option does) are stuck inside functions that also _execute_ (read from stdin, print to stdout, call `std::process::exit`). You can't test decisions without triggering execution.

And when requirements change — "add logging," "make it async," "replay user sessions for QA" — you have to touch the same tangled code that handles both concerns.

The Free Monad DSL pattern cuts this knot. You define your program as a data structure: `Print this`, `ReadLine and continue`, `Exit with code`. The program is built purely. Then you write one interpreter for production (real IO, real exit) and another for tests (fake input, captures output, returns `Exited(code)` instead of actually exiting). Same program. Different execution. This is exactly the pain this exists to solve.

## The Intuition

Imagine you're scripting a stage play. The script says: "Character A speaks line 1. Then waits for Character B's response. Then speaks line 2 based on what B said." The script is just _text_ — it doesn't perform itself. On opening night, real actors perform it. At a table read, everyone reads lines aloud. In a simulation, you could run through all possible dialogue trees automatically.

The `Console<A>` enum is your script:

```rust
enum Console<A> {
    Pure(A),                                       // The scene is over, here's the outcome
    Print(String, Box<dyn FnOnce() -> Console<A>>),  // "Say this line, then..."
    ReadLine(Box<dyn FnOnce(String) -> Console<A>>), // "Wait for response, then..."
    Exit(i32),                                     // "The play ends with this code"
}
```

Each constructor says: "do this, then do _that_." The `Exit` case is special — it's a dead end with an exit code, no continuation. The `A` in `Console<A>` is the type of value the whole program eventually produces (if it doesn't exit first).

`bind` is how you chain scenes. "After this scene finishes producing an `A`, feed that `A` into the next scene."

## How It Works in Rust

**Step 1: Smart constructors — build DSL instructions cleanly**

```rust
fn print_line(msg: &str) -> Console<()> {
    let msg = msg.to_string();
    // Print the message, then immediately continue with ()
    Console::Print(msg, Box::new(|| Console::Pure(())))
}

fn read_line_dsl() -> Console<String> {
    // Wait for input, pass it to the next step as a String
    Console::ReadLine(Box::new(|s| Console::Pure(s)))
}

fn exit_prog<A>(code: i32) -> Console<A> {
    Console::Exit(code)  // no continuation — program ends here
}
```

**Step 2: Build the program with `bind`**

```rust
fn menu_program() -> Console<String> {
    bind(print_line("=== Menu ==="), move |()| {
    bind(print_line("1. Greet"),     move |()| {
    bind(print_line("2. Exit"),      move |()| {
    bind(print_line("Choose: "),     move |()| {
    bind(read_line_dsl(), move |choice: String| {
        match choice.as_str() {
            "1" => bind(print_line("Enter name: "), move |()| {
                       bind(read_line_dsl(), move |name: String| {
                           bind(print_line(&format!("Hello, {}!", name)), move |()| {
                               pure(format!("greeted {}", name))
                           })
                       })
                   }),
            "2" => exit_prog(0),  // <-- no IO, just data
            _   => bind(print_line("Invalid choice"), |()| pure("error".to_string())),
        }
    })})})})})
}
```

Notice: `exit_prog(0)` doesn't call `std::process::exit`. It just puts `Console::Exit(0)` in the data structure. The interpreter decides what that means.

**Step 3: Two interpreters — production vs test**

The test interpreter captures everything and returns results instead of side-effecting:

```rust
fn interpret_pure(inputs: &[&str], prog: Console<String>) -> (Vec<String>, ProgramResult<String>) {
    let mut outputs = Vec::new();
    let mut idx = 0;
    let mut current = prog;

    loop {
        match current {
            Console::Pure(a)       => return (outputs, ProgramResult::Ok(a)),
            Console::Exit(code)    => return (outputs, ProgramResult::Exited(code)), // no actual exit!
            Console::Print(msg, k) => { outputs.push(msg); current = k(); }
            Console::ReadLine(k)   => {
                let input = inputs.get(idx).unwrap_or(&"").to_string();
                idx += 1;
                current = k(input);
            }
        }
    }
}
```

**Step 4: Test all paths trivially**

```rust
// Path: choose 1, enter name
let (out, result) = interpret_pure(&["1", "Alice"], menu_program());
assert_eq!(result, ProgramResult::Ok("greeted Alice".to_string()));

// Path: choose 2 (exit)
let (_, result) = interpret_pure(&["2"], menu_program());
assert_eq!(result, ProgramResult::Exited(0));  // program "exits" without actually exiting

// Path: invalid input
let (out, _) = interpret_pure(&["x"], menu_program());
assert!(out.contains(&"Invalid choice".to_string()));
```

All three paths tested in milliseconds, no subprocess, no terminal emulation.

## What This Unlocks

- **Full branch coverage without IO**: Every menu path, every error case — tested with a list of strings. No mocking frameworks needed.
- **Pluggable execution models**: The production interpreter calls real `println!` and `stdin`. An async interpreter could use `tokio::io`. A replay interpreter could read from a recorded session file. Zero changes to the program.
- **Auditable programs**: Because the program is data, you can traverse it before running — count how many prompts a user will see, validate that all branches terminate or exit, log the full interaction plan.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Program type | `type 'a console` with HKT | Specialized `enum Console<A>` |
| Continuations in data | Closures, GC'd naturally | `Box<dyn FnOnce>` — heap allocated |
| Exit handling | Algebraic — just a case | `Console::Exit(i32)` — same idea |
| Chaining | `>>=` operator | Nested `bind(...)` — syntactically heavier |
| Testing | Same pattern | Same pattern — equally clean at runtime |
