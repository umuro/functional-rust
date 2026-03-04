# 464: Actor Pattern

**Difficulty:** 3  **Level:** Intermediate

Encapsulate mutable state in a thread with a message inbox — no locks needed, ever.

## The Problem This Solves

Shared mutable state is the root of most concurrency bugs. You have a cache, a counter, or a connection pool that multiple threads need to read and write. The standard fix is `Arc<Mutex<T>>` — but now you're managing lock lifetimes, worrying about deadlocks, and discovering that long critical sections destroy performance.

The actor model takes a different approach: one thread *owns* the state, everyone else sends it messages. The state never moves between threads, so no locks are needed. Callers send a `Command` enum variant and optionally include a reply channel for responses. The actor processes messages one at a time — serialization is structural, not enforced by locks.

This is how Erlang, Akka, and Go's goroutines-with-channels work. In Rust, it maps directly to `std::sync::mpsc`: an actor is a thread looping on `recv()`, pattern-matching on command variants, and responding via one-shot channels embedded in the messages.

## The Intuition

An actor is a thread with a typed mailbox: all state lives inside the thread, all interaction is through message variants, and serialization is guaranteed by the single-threaded message loop — no `Mutex` required. The trade-off: simpler reasoning about state, but every interaction has message-passing overhead instead of direct memory access.

## How It Works in Rust

```rust
use std::sync::mpsc;
use std::thread;

// Commands the actor understands
enum Command {
    Increment,
    Get(mpsc::SyncSender<i32>),  // caller embeds a reply channel
    Stop,
}

let (tx, rx) = mpsc::sync_channel::<Command>(32);

// Actor thread: owns all mutable state
thread::spawn(move || {
    let mut count = 0;           // state lives here — never leaves this thread
    for cmd in rx {
        match cmd {
            Command::Increment => count += 1,
            Command::Get(reply) => reply.send(count).unwrap(),
            Command::Stop => break,
        }
    }
});

// Callers send messages — no locking, no sharing
tx.send(Command::Increment).unwrap();
tx.send(Command::Increment).unwrap();

let (reply_tx, reply_rx) = mpsc::sync_channel(1);
tx.send(Command::Get(reply_tx)).unwrap();
let value = reply_rx.recv().unwrap();  // blocks until actor replies

tx.send(Command::Stop).unwrap();
```

## What This Unlocks

- **Connection pools**: the actor owns the pool, hands out connections via `Borrow` command, returns them via `Return` — no deadlock possible.
- **Shared caches**: actor owns the `HashMap`, all threads read/write via message — consistent without locks.
- **Stateful protocol handlers**: TCP connection state machine lives in one actor, driven by incoming packet messages.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Actor state | Mutable refs inside thread | Owned local variables in message loop |
| Message type | Variant type | `enum Command` |
| Reply mechanism | Shared `Queue` + blocking wait | `mpsc::SyncSender` embedded in message |
| No-reply message | Variant with no payload | Variant with no fields |
| Shutdown | `Stop` variant | `Stop` variant or drop all senders |
| Concurrency primitive | Manual mutex avoided | `mpsc::sync_channel` — no `Mutex` needed |
