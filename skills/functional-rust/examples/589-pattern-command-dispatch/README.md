# 589: Command Dispatch with Enums

**Difficulty:** 3  **Level:** Intermediate

Represent commands as enum variants and dispatch them with match — type-safe, extensible, and testable.

## The Problem This Solves

String-based command dispatch is the quick way out: `match cmd.name { "set" => ..., "remove" => ..., _ => unknown }`. It compiles regardless of what strings you have. Add a new command? Hope you don't typo. Rename a command? Grep-and-pray. The `_` arm silently swallows anything unrecognized.

HashMap-based dispatch (function pointers or boxed closures indexed by string) is slightly better for extensibility, but the types get messy and the commands themselves can't carry typed payloads. A `"set"` command needs a key and a value; how do you pass those safely through a `HashMap<String, fn(...)>`?

Enum command dispatch solves both. Each command is a variant that carries its own payload — strongly typed, named, checked by the compiler. Adding a new command means adding a variant and an arm; the compiler points you to every match that needs updating. No magic strings, no type erasure.

## The Intuition

Think of the command enum as a type-safe message protocol. `Cmd::Set(key, value)`, `Cmd::Increment(key, delta)`, `Cmd::Remove(key)`, `Cmd::Clear`. Each variant encodes exactly what the command needs, no more, no less.

The executor is a match: "what shape is this command? do the right thing." The match is also the audit log — you can see every command the system handles at a glance. No dynamic dispatch, no virtual functions, no trait objects needed.

An important bonus: pure replay. Store the commands as a `Vec<Cmd>`. Replay them with `fold` over an empty state. You get an event-sourcing architecture for free — the history *is* the state. OCaml's functional approach naturally supports this with immutable stores.

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
enum Cmd {
    Set(String, i64),
    Remove(String),
    Increment(String, i64),
    Clear,
}

#[derive(Debug, Default)]
struct Store {
    data: HashMap<String, i64>,
    history: Vec<Cmd>,
}

impl Store {
    fn execute(&mut self, cmd: Cmd) {
        // Dispatch on variant — each arm handles one command
        match &cmd {
            Cmd::Set(k, v)        => { self.data.insert(k.clone(), *v); }
            Cmd::Remove(k)        => { self.data.remove(k); }
            Cmd::Increment(k, d)  => { *self.data.entry(k.clone()).or_default() += d; }
            Cmd::Clear            => { self.data.clear(); }
        }
        self.history.push(cmd);  // record every command
    }
}

// Pure command application — no side effects, returns new state
fn apply(mut data: HashMap<String, i64>, cmd: &Cmd) -> HashMap<String, i64> {
    match cmd {
        Cmd::Set(k, v)       => { data.insert(k.clone(), *v); }
        Cmd::Remove(k)       => { data.remove(k); }
        Cmd::Increment(k, d) => { *data.entry(k.clone()).or_default() += d; }
        Cmd::Clear           => { data.clear(); }
    }
    data
}

// Replay history from scratch — event sourcing
let final_state = store.history
    .iter()
    .fold(HashMap::new(), |acc, cmd| apply(acc, cmd));
```

## What This Unlocks

- **Type-safe command encoding** — each command carries its exact payload; no stringly-typed argument unpacking.
- **Event sourcing** — the command history is the source of truth; replay any time to reconstruct state.
- **Compiler-guided extension** — add `Cmd::SetIfAbsent` and the compiler finds every match that needs a new arm.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Command type | `type cmd = Add of string * int \| ...` | `enum Cmd { Set(String, i64), ... }` |
| Dispatch | `let execute store = function \| Add(k,v) -> ...` | `match cmd { Cmd::Set(k,v) => ... }` |
| Mutable state | Immutable store + new value returned | `&mut self` — mutate in place |
| Pure replay | Natural — fold over immutable commands | `fn apply(data, cmd) -> data` — explicit pure form |
| History | Functional: list of commands is natural | `Vec<Cmd>` in the store struct |
