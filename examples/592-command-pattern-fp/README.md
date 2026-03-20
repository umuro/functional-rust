📖 **[View on hightechmind.io →](https://hightechmind.io/rust/592-command-pattern-fp)**

---

# Command Pattern (Functional Style)
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

The OOP command pattern encapsulates actions as objects with `execute` and `undo` methods. The functional version is simpler: commands are data (enum variants), interpretation is a separate function. This separation — data from behavior — makes commands serializable, loggable, testable, and replayable without coupling them to execution context. It is the foundation of event sourcing, undo/redo systems, game replay, and distributed systems that need to log and replay operations.

## Learning Outcomes

- How commands as enum variants separate data from execution
- How a `DrawState` interpreter executes commands against mutable state
- How storing `Vec<DrawCmd>` enables replay, undo, and serialization
- How functional commands differ from OOP commands (no `dyn Trait` needed)
- Where functional command pattern appears: event sourcing, game replay, undo history, CQRS

## Rust Application

`DrawCmd` enum variants represent all possible drawing operations. `DrawState` holds the current rendering state. `interpret(state: &mut DrawState, cmd: &DrawCmd)` matches on the command and updates state. A `Vec<DrawCmd>` is a complete "script" that can be replayed identically. Commands can be serialized (with `serde`), filtered, compressed, and distributed.

Key patterns:
- Enum variants as command data: `DrawCmd::MoveTo(x, y)`
- `fn interpret(state: &mut State, cmd: &Cmd)` — pure command execution
- `Vec<Cmd>` as a replayable log
- Undo: store inverse commands or snapshot state before each command

## OCaml Approach

```ocaml
type draw_cmd = MoveTo of float * float | LineTo of float * float | SetColor of string
type state = { mutable x: float; mutable y: float; mutable color: string }
let interpret state = function
  | MoveTo (x, y) -> state.x <- x; state.y <- y
  | LineTo (x, y) -> (* draw line from state.{x,y} to x,y *) state.x <- x; state.y <- y
  | SetColor c -> state.color <- c
let replay state cmds = List.iter (interpret state) cmds
```

## Key Differences

1. **Enum vs class hierarchy**: Rust enum variants are the natural command representation; OCaml uses variant types identically; OOP would use `interface Command` with subclasses.
2. **Interpreter as function**: Both languages use a plain function for interpretation — no `dyn Trait` indirection or virtual dispatch needed for the simple case.
3. **Serialization**: Rust commands with `#[derive(serde::Serialize)]` become JSON-serializable; OCaml uses `ppx_serde` or manual serialization.
4. **Undo**: Both languages handle undo by storing inverse commands or snapshots — the data-oriented approach makes this straightforward.

## Exercises

1. **Undo stack**: Add an `undo_stack: Vec<DrawCmd>` to `DrawState` that stores inverse commands; implement `undo(state: &mut DrawState)` that pops and executes the inverse.
2. **Command filter**: Write `fn remove_color_changes(cmds: Vec<DrawCmd>) -> Vec<DrawCmd>` that removes all `SetColor` commands from a command list.
3. **Command serialization**: Add `#[derive(serde::Serialize, serde::Deserialize)]` to `DrawCmd` and write functions to save/load a command sequence to/from JSON.
