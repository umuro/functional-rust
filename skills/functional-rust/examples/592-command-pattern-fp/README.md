# 592: Command Pattern as Data

**Difficulty:** 3  **Level:** Intermediate

Encode operations as enum variants so they can be queued, replayed, undone, or logged — separating intent from execution.

## The Problem This Solves

In most codebases, "do X" and "doing X right now" are the same thing — you call the function and it runs immediately. This is fine until you need to queue operations for later, replay a sequence of actions, implement undo/redo, log what happened for audit, or synchronize state between distributed nodes.

The functional command pattern decouples *describing* an operation from *executing* it. Commands are enum variants — plain data. They can be stored in a `Vec`, sent over a channel, serialized to disk, filtered, or inspected before execution. The executor is a separate function that pattern-matches on variants.

This is the foundation of event sourcing (your database is a log of commands), CQRS (commands and queries are separate), undo/redo systems (maintain a history stack), and remote procedure calls (serialize the command, send it, execute on the other side).

## The Intuition

When a command is data (an enum variant), "now" vs "later" is just a matter of when you pass it to the executor — the operation itself is fully described, storable, and inspectable before it runs. The trade-off: you add a layer of indirection (describe then execute) which costs a little in simplicity but gains enormous flexibility.

## How It Works in Rust

```rust
use std::collections::VecDeque;

// Commands are data — fully described, no side effects yet
#[derive(Debug, Clone)]
enum EditorCommand {
    InsertChar { pos: usize, ch: char },
    DeleteChar { pos: usize },
    MoveCursor { delta: i32 },
}

struct Editor {
    text: String,
    cursor: usize,
    history: Vec<EditorCommand>,    // undo stack
}

impl Editor {
    fn execute(&mut self, cmd: EditorCommand) {
        self.history.push(cmd.clone());   // record before executing
        match cmd {
            EditorCommand::InsertChar { pos, ch } => self.text.insert(pos, ch),
            EditorCommand::DeleteChar { pos } => { self.text.remove(pos); }
            EditorCommand::MoveCursor { delta } => {
                self.cursor = (self.cursor as i32 + delta).max(0) as usize;
            }
        }
    }

    fn undo(&mut self) {
        if let Some(cmd) = self.history.pop() {
            // apply inverse — commands are data so we can reason about inverses
            match cmd {
                EditorCommand::InsertChar { pos, .. } => { self.text.remove(pos); }
                EditorCommand::DeleteChar { pos } => { /* restore from somewhere */ }
                _ => {}
            }
        }
    }
}

// Commands can be queued, replayed, filtered
let mut queue: VecDeque<EditorCommand> = VecDeque::new();
queue.push_back(EditorCommand::InsertChar { pos: 0, ch: 'H' });
queue.push_back(EditorCommand::InsertChar { pos: 1, ch: 'i' });
```

## What This Unlocks

- **Undo/redo**: history stack is a `Vec<Command>` — replay forward or apply inverses backward.
- **Event sourcing**: persist the command log to disk; rebuild state by replaying from any checkpoint.
- **Distributed sync**: serialize commands (with `serde`), send to peers, execute in order — CRDTs and OT are elaborations of this.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Command as value | Variant type | `enum` variant |
| Undo | Inverse variant or delta | Inverse variant or stored old state |
| History queue | `list` or `Queue.t` | `Vec` or `VecDeque` |
| Serialization | `ppx_serde` / `ppx_yojson` | `#[derive(Serialize, Deserialize)]` |
| Execution | Pattern match function | `match cmd { ... }` in executor method |
| Replay | Re-execute list | Iterate history and re-execute |
