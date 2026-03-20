📖 **[View on hightechmind.io →](https://hightechmind.io/rust/464-actor-pattern)**

---

# 464: Actor Pattern

## Problem Statement

Shared mutable state with locks leads to deadlocks, priority inversion, and complex reasoning. The actor model offers an alternative: each actor is an isolated entity with private state, communicating only through message passing. No locks, no shared memory — just messages. An actor processes messages sequentially, ensuring its state is never concurrently modified. This model was popularized by Erlang and is the foundation of `actix`, `tokio::actor`, and any system requiring encapsulated concurrent state.

Actor systems power chat servers (each user is an actor), game entities (each NPC is an actor), distributed systems (each node is an actor), and the Actix web framework.

## Learning Outcomes

- Understand the actor model: isolated state, message-driven behavior, no shared memory
- Learn how `mpsc::Sender<Msg>` serves as the actor handle (message address)
- See how an actor loop (`for m in rx`) processes messages sequentially
- Understand request-response with `mpsc::SyncSender<i64>` in the message for synchronous queries
- Learn why actors eliminate lock-based concurrency bugs

## Rust Application

In `src/lib.rs`, `Actor` holds a `Sender<Msg>`. The actor thread runs `for m in rx { match m { ... } }`, updating `state` sequentially. `Msg::Get(reply_tx)` demonstrates request-response: the caller creates a one-shot `sync_channel(1)`, sends it in the message, and waits for the reply. `Msg::Stop` breaks the loop, ending the actor. The actor's private `s: i64` is inaccessible to other threads.

## OCaml Approach

OCaml's actor model is natural with `Event` channels and the `Thread` module: each actor is a thread with its own channel. `Erlang`-style actors in OCaml use the `Eio` or `Mirage` effect-based runtimes. The `actor` library provides Erlang-style process spawning. OCaml's lightweight threads in OCaml 5.x (via `Eio.Fiber`) make actor systems more efficient than OS-thread-based implementations.

## Key Differences

1. **Message enum**: Rust uses a `Msg` enum for all messages to one actor; Erlang/OCaml typically use dynamic dispatch or `'a` polymorphic messages.
2. **Request-response**: Rust requires a reply channel in the message; Erlang has built-in `receive` with `!` for replies.
3. **Actor identity**: Rust actors are `Sender<Msg>` handles; Erlang has process IDs (PIDs) as first-class values.
4. **Supervision**: Erlang has built-in supervision trees; Rust's `tokio::actor` pattern requires manual error handling.

## Exercises

1. **Bank account**: Implement a `BankAccount` actor with `Deposit(Amount)`, `Withdraw(Amount)`, `Balance(SyncSender<i64>)` messages. Verify concurrent deposits and withdrawals produce correct balances without locks.
2. **Actor supervision**: Create a supervisor that monitors an actor by joining its thread handle. If the actor panics, the supervisor automatically restarts it with fresh state. Implement a counter of restarts.
3. **Actor network**: Implement a simple pub-sub system where a `Broker` actor routes `Subscribe(topic, reply_tx)` and `Publish(topic, payload)` messages. Subscribers receive published messages for their topics.
