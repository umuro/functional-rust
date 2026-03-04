# 189: Effect Handlers — Introduction

**Difficulty:** ⭐⭐⭐⭐⭐  **Level:** Expert

Model algebraic effects as typed requests that a program emits and a handler fulfills — cleanly separating the description of "what to do" from the implementation of "how to do it."

## The Problem This Solves

Side effects are normally baked into the function that produces them. `println!` reaches into stdout. `read_line` touches stdin. Testing a function that does both requires capturing stdout or injecting stdin — infrastructure that fights against the language's defaults. Mocking and dependency injection exist precisely to paper over this coupling.

Algebraic effects (from languages like OCaml 5, Koka, Unison) propose a cleaner model: a computation *performs* an effect as a typed request — "I need a line of input" — without knowing how that request will be fulfilled. A nearby *handler* intercepts the request and decides: read from stdin, read from a test buffer, return a hardcoded value, log the request, whatever. The computation and the handler are completely decoupled.

In Rust we don't have native effect syntax, but we can simulate the same structure: the program emits `Effect::Readline` or `Effect::Print(msg)` as data; a `Handler` trait intercepts each one. Swapping the handler changes the behavior without touching the program. The pure simulation handler and the real I/O handler implement the same trait.

## The Intuition

Think of a waiter and a kitchen. The waiter takes orders (effects) from diners (the program). The waiter doesn't know how to cook — that's the kitchen's job (the handler). On a normal day, orders go to the real kitchen. During a health inspection, orders go to a simulation kitchen that records what was ordered without cooking anything. The waiter's behavior is identical in both cases; only the handler changes.

In code: `interactive_program()` calls `readline_effect(...)` to say "I need a line of input here." It doesn't know whether that input comes from a real user or a test vector. The `PureHandler` provides fake inputs and captures outputs to a buffer. The `IoHandler` calls real `stdin.read_line`. The program is identical.

## How It Works in Rust

```rust
// Step 1: Define the effect requests your program can make
enum Effect {
    Print(String),
    Readline,
}

// Step 2: What each effect "returns" to the program
enum EffectResult {
    Unit,
    Line(String),
}

// Step 3: A program step is either done or an effect + continuation
enum Step<A> {
    Done(A),
    Effect(Effect, Box<dyn FnOnce(EffectResult) -> Step<A>>),
    //             ^^ "after you handle this effect, give me the result, I'll continue"
}

// Step 4: Smart constructors build the description
fn readline_effect<A: 'static, F: FnOnce(String) -> Step<A> + 'static>(f: F) -> Step<A> {
    Step::Effect(
        Effect::Readline,
        Box::new(move |r| match r {
            EffectResult::Line(s) => f(s),  // receive the line, pass to continuation
            _ => panic!("unexpected"),
        }),
    )
}

fn print_effect<A: 'static>(msg: impl Into<String>, next: Step<A>) -> Step<A> {
    Step::Effect(Effect::Print(msg.into()), Box::new(move |_| next))
}

// Step 5: Define the Handler trait — one method per effect
trait Handler {
    fn handle_print(&mut self, msg: &str) -> EffectResult;
    fn handle_readline(&mut self) -> EffectResult;
}

// Step 6: Run the program by feeding effect results back into continuations
fn run_with<A, H: Handler>(mut handler: H, mut step: Step<A>) -> (A, H) {
    loop {
        match step {
            Step::Done(x) => return (x, handler),
            Step::Effect(Effect::Print(msg), cont) => {
                let r = handler.handle_print(&msg);
                step = cont(r);  // handler provides result, program continues
            }
            Step::Effect(Effect::Readline, cont) => {
                let r = handler.handle_readline();
                step = cont(r);
            }
        }
    }
}

// The SAME program, two different handlers:
fn interactive_program() -> Step<String> {
    print_effect("What is your name?",
        readline_effect(|name| {
            let greeting = format!("Hello, {}!", name);
            print_effect(greeting, done(name))
        }))
}

// Handler 1: pure test simulation
struct PureHandler { inputs: VecDeque<String>, output: String }
impl Handler for PureHandler {
    fn handle_print(&mut self, msg: &str) -> EffectResult {
        self.output.push_str(msg); self.output.push('\n');
        EffectResult::Unit
    }
    fn handle_readline(&mut self) -> EffectResult {
        EffectResult::Line(self.inputs.pop_front().unwrap_or_default())
    }
}

// Handler 2: real I/O — same interface
struct IoHandler;
impl Handler for IoHandler {
    fn handle_print(&mut self, msg: &str) -> EffectResult { println!("{}", msg); EffectResult::Unit }
    fn handle_readline(&mut self) -> EffectResult {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        EffectResult::Line(line.trim_end().to_string())
    }
}
```

## What This Unlocks

- **Effect polymorphism** — write a single program that works with any handler; swap handlers for testing, logging, replay, or sandboxing without modifying the program.
- **Composable effects** — combine handlers (e.g., a logging handler that wraps a real I/O handler) the same way you compose iterators; no global state involved.
- **Capability-based security** — a program can only use effects that are handled; restrict what a sandboxed computation can do by providing a limited handler.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Performing an effect | `perform (Print "msg")` — built-in syntax, suspends execution | `Step::Effect(Effect::Print(...), Box::new(continuation))` — manual, data-based |
| Resuming after effect | `continue k value` — native continuation resumption | Interpreter calls `cont(result)` — continuation is a boxed closure |
| Handler syntax | `match f () with \| effect (Print msg) k -> ... \| effect Readline k -> ...` | `impl Handler for Foo` — trait with one method per effect |
| Type safety | Effects declared with `effect Print : string -> unit`; type-checked | Effect enum is untyped; wrong EffectResult variant panics at runtime |
| Multiple effects | Native — same `match` handles multiple effects | Multiple `Effect` variants in one enum; single `Handler` trait with one method per variant |
