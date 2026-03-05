📖 **[View on hightechmind.io →](https://hightechmind.io/rust/599-free-monad-rust)**

---

# 599: Free Monad Interpretation

**Difficulty:** 4  **Level:** Advanced

Describe an interactive program as a pure data structure, then run it with a real IO interpreter or a pure test interpreter — same program, different execution.

## The Problem This Solves

You're writing a program that asks a user their name and age, then tells them something. Simple enough. But there's a hidden problem: the moment you call `println!` or `stdin().read_line()`, your program is coupled to real IO. You can't run it in a test without actual terminal interaction. You can't easily replay a session. You can't swap in a different IO strategy (network, file, event loop) without rewriting the program logic.

Here's what the tangled version looks like:

```rust
fn run_program() {
    println!("What is your name?");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    println!("Hello, {}!", name);

    println!("How old are you?");
    let mut age_str = String::new();
    std::io::stdin().read_line(&mut age_str).unwrap();
    let age: u32 = age_str.trim().parse().unwrap_or(0);

    println!("In 10 years you'll be {}.", age + 10);
}
```

This runs. But test it. You can't without automating terminal I/O, which is painful and fragile. And if you later want to run this same logic over a network socket or from a file of pre-recorded inputs, you're rewriting everything.

The Free Monad pattern turns the program into a **data structure** — a linked list of instructions saying "print this, read a line, then do this with the result." Once you have that data, you can run it with any executor: real stdio, a vector of fake inputs, a network reader, an async runtime. The program describes _what_, the interpreter decides _how_. This is exactly that pain solved.

## The Intuition

Imagine a chatbot script written as a flowchart:

```
→ Print "What is your name?"
→ Read line → store as `name`
→ Print "Hello, [name]!"
→ Read line → store as `age_str`
→ Compute age+10
→ Print "In 10 years you'll be [result]."
→ Done
```

That flowchart is a _description_. You can run it with a human at a terminal. You could run it with an automated test that feeds "Alice" and "30" into the read steps. You could run it in a chat interface. The flowchart doesn't change — only the execution engine does.

In code, the flowchart is the `Prog` enum:

```rust
enum Prog {
    Done,
    Print(String, Box<Prog>),          // "print this, then continue with next"
    Read(Box<dyn Fn(String) -> Prog>), // "read a line, feed it to f, continue"
}
```

- `Done` — the program finished
- `Print(msg, next)` — print `msg`, then do `next`
- `Read(f)` — get a line of text, pass it to `f` which decides what to do next

Smart constructors build these without the noise:

```rust
fn print_prog(s: impl Into<String>, next: Prog) -> Prog {
    Prog::Print(s.into(), Box::new(next))
}

fn read_prog(f: impl Fn(String) -> Prog + 'static) -> Prog {
    Prog::Read(Box::new(f))
}
```

## How It Works in Rust

**Step 1: Build the program (no IO happens here)**

```rust
fn make_program() -> Prog {
    print_prog("What is your name?",
    read_prog(|name|
    print_prog(format!("Hello, {}!", name),
    read_prog(|age_str| {
        let age: u32 = age_str.parse().unwrap_or(0);
        print_prog(format!("In 10 years you'll be {}.", age + 10),
        Prog::Done)
    }))))
}
```

This is a pure function. No `println!`. No `stdin`. It just builds the tree of instructions.

**Step 2: Production interpreter — real IO**

```rust
fn run_io(prog: Prog) {
    match prog {
        Prog::Done => {}
        Prog::Print(s, next) => {
            println!("{}", s);   // actual side effect here
            run_io(*next);
        }
        Prog::Read(f) => {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).ok();  // actual side effect here
            run_io(f(buf.trim().to_string()));
        }
    }
}
```

**Step 3: Test interpreter — pure, no IO**

```rust
fn run_test(prog: Prog, mut inputs: Vec<String>) -> Vec<String> {
    let mut outputs = Vec::new();
    let mut current = prog;

    loop {
        current = match current {
            Prog::Done           => break,
            Prog::Print(s, next) => {
                outputs.push(s);   // collect instead of printing
                *next
            }
            Prog::Read(f) => {
                let input = inputs.remove(0);  // consume from fake input list
                f(input)
            }
        };
    }
    outputs
}
```

**Step 4: Test without any IO**

```rust
let prog = make_program();
let outputs = run_test(prog, vec!["Alice".into(), "30".into()]);

assert!(outputs[1].contains("Alice"));   // "Hello, Alice!"
assert!(outputs[2].contains("40"));      // "In 10 years you'll be 40."
```

Zero terminal interaction. Deterministic. Runs in milliseconds.

**What breaks without this pattern:**

Without the separation, `make_program()` would call `println!` and `stdin()` directly. The test would block waiting for real input. You'd need a subprocess harness or OS-level IO redirection — fragile, platform-specific, slow.

## What This Unlocks

- **Testable interactive programs**: Feed `vec!["Alice", "30"]` and assert on collected output — no terminal, no mocking, no subprocesses.
- **Pluggable execution**: Same `make_program()` runs with `run_io` for production, `run_test` for unit tests, and could run with `run_async` for an async runtime or `run_network` for a socket-based interface.
- **Session replay and recording**: Because the program is data, you can serialize it, record what inputs came in, and replay the interaction exactly — useful for regression testing and debugging.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Free monad | `type 'a free = Pure of 'a \| Free of ...` | `enum Free<A>` / specialized `enum Prog` |
| Flatmap | `bind` with `>>=` | Manual `bind` or struct-based chaining |
| Interpreter | Recursive pattern match | Loop or recursion — same idea |
| Stack safety | Needs trampoline for deep programs | Same — loop in `run_test` avoids stack overflow |
| Closures in data | GC'd naturally | `Box<dyn Fn>` — heap, `'static` required |
