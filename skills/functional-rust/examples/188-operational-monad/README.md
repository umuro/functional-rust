# 188: Operational Monad

**Difficulty:** ⭐⭐⭐⭐⭐  **Level:** Expert

Describe a program as a sequence of typed instructions, then interpret it separately — like the free monad but with an explicit instruction set that's easier to inspect and extend.

## The Problem This Solves

The free monad (example 187) is powerful but abstract — the instruction type is tangled with the continuation structure. When you have a DSL with multiple operation types, the free monad requires wrapping and unwrapping multiple layers of functors. Adding a new operation means updating the functor type, the `bind` implementation, and every interpreter.

The operational monad separates concerns more cleanly. The **instruction set** is just an enum — `Read`, `Write(String)`. A **program** sequences these instructions: `Instr(Read, continuation) | Return(value)`. The instruction says *what* to do; the continuation says *what to do with the result*. Interpreters handle each instruction case independently, making them easy to write, test, and swap.

This pattern is how you build testable I/O in functional style. Your `hello_program()` function never touches stdin or stdout — it builds a data structure describing the interaction. Tests provide a list of fake inputs; production uses real I/O. The program itself is completely pure; all effects live in the interpreter.

## The Intuition

A theatre script. The script says `ACTOR: "What is your name?"` then `PAUSE for audience response` then `ACTOR: "Hello, [response]!"`. The script doesn't make any sounds — it's just text. A director *interprets* the script: assigns actors, specifies the setting, decides whether to use real microphones or audio playback. Two different directors, same script, completely different productions.

In code: `prog_read(|name| prog_write(format!("Hello, {}!", name), prog_return(name)))` is the script. `run_pure(&["Alice"], prog)` is one director (uses a list for input, collects output to a buffer). `run_io(prog)` would be another director (uses real stdin/stdout). The script — the program — is pure data.

## How It Works in Rust

```rust
// The instruction set — what operations exist
enum Instr {
    Read,           // "give me a line of input"
    Write(String),  // "output this string"
}

// A program: either done, or one instruction + what to do with its result
enum Prog<A> {
    Return(A),
    Instr(Instr, Box<dyn FnOnce(InstrResult) -> Prog<A>>),
    //     ^^^^                  ^^^^^^^^^^
    //     what to do            what the instruction "returns" to us
}

// Each instruction has a return type — this is where OCaml's GADT shines
enum InstrResult {
    ReadResult(String),   // Read returns a String
    WriteResult,          // Write returns ()
}

// Smart constructors — the API for building programs
fn prog_read<A: 'static, F: FnOnce(String) -> Prog<A> + 'static>(f: F) -> Prog<A> {
    Prog::Instr(
        Instr::Read,
        Box::new(move |r| match r {
            InstrResult::ReadResult(s) => f(s),   // unwrap the result, pass to continuation
            _ => panic!("type mismatch"),          // in OCaml GADTs, this case doesn't exist
        }),
    )
}

fn prog_write<A: 'static>(s: impl Into<String>, next: Prog<A>) -> Prog<A> {
    let s = s.into();
    Prog::Instr(Instr::Write(s), Box::new(move |_| next))
}

// A program description — pure data, no I/O:
fn hello_program() -> Prog<String> {
    bind(prog_write("Enter name:", prog_return(())), |_| {
        prog_read(|name| {
            let msg = format!("Hello, {}!", name);
            bind(prog_write(msg, prog_return(())), move |_| prog_return(name))
        })
    })
}

// Pure interpreter — no real I/O, fully testable:
fn run_pure<A>(inputs: &[&str], prog: Prog<A>) -> (A, String) {
    let mut buf = String::new();
    let mut inputs: VecDeque<String> = inputs.iter().map(|s| s.to_string()).collect();

    fn go<A>(prog: Prog<A>, inputs: &mut VecDeque<String>, buf: &mut String) -> A {
        match prog {
            Prog::Return(x) => x,
            Prog::Instr(Instr::Read, cont) => {
                let line = inputs.pop_front().unwrap_or_default();
                go(cont(InstrResult::ReadResult(line)), inputs, buf)  // feed fake input
            }
            Prog::Instr(Instr::Write(s), cont) => {
                buf.push_str(&s); buf.push('\n');
                go(cont(InstrResult::WriteResult), inputs, buf)  // collect output
            }
        }
    }
    (go(prog, &mut inputs, &mut buf), buf)
}

// Test — no stdin/stdout involved:
let (name, output) = run_pure(&["Alice"], hello_program());
assert_eq!(name, "Alice");
assert!(output.contains("Hello, Alice!"));
```

## What This Unlocks

- **Testable I/O** — programs that interact with users, files, or networks can be fully unit-tested by running them against fake interpreters; no mocking frameworks needed.
- **Logging/auditing interpreters** — wrap the interpreter to record every instruction executed; replay bugs by re-running the exact instruction sequence.
- **Game engine command queues** — a frame's worth of game actions as an operational monad program; the interpreter applies them to the game state, or serializes them for multiplayer sync.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Instruction typing | `type _ instr = Read : string instr \| Write : string -> unit instr` — GADT gives each instruction its own return type | `enum Instr` + `enum InstrResult` — two enums with manual matching; no compile-time per-instruction return type |
| Type safety of results | GADT: `Read`'s continuation receives `string`, guaranteed statically | Rust: `ReadResult(String)` unwrapped at runtime with `panic!` if wrong variant |
| Bind | Clean recursion with locally abstract types | `Box<dyn FnOnce>` with `'static` bounds — heavier but equivalent |
| Extensibility | Add a new `type _ instr` constructor; all match arms catch it | Add new `Instr` variant + `InstrResult` variant; update interpreter `match` |
| Real-world analogy | Exactly the pattern used in Haskell's `operational` library | Matches the "free algebra" pattern used in Elm's `Cmd` type |
