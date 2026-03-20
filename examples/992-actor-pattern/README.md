[actor-pattern on hightechmind.io](https://hightechmind.io/posts/functional-rust/actor-pattern)

---

## Problem Statement

Implement the actor model in Rust: an actor owns mutable state and processes messages sequentially from an `mpsc` channel mailbox. External code sends messages but never accesses state directly. The actor runs in its own thread. Implement a `CounterActor` with increment/decrement/get/shutdown messages, demonstrating request-reply via a one-shot `Sender<T>` in the message.

## Learning Outcomes

- Define a `enum CounterMsg { Increment(i64), Decrement(i64), GetValue(mpsc::Sender<i64>), Shutdown }` for typed messages
- Spawn an actor thread that owns `state: i64` and processes messages via `rx.iter()`
- Implement request-reply: `GetValue(mpsc::Sender<i64>)` carries a reply channel inside the message
- Implement `Shutdown`: `break` from the processing loop, closing the mailbox channel
- Recognize the actor model as a safe alternative to shared mutable state

## Rust Application

```rust
#[derive(Debug)]
enum CounterMsg {
    Increment(i64),
    Decrement(i64),
    GetValue(mpsc::Sender<i64>),
    Shutdown,
}

struct CounterActor {
    tx: mpsc::Sender<CounterMsg>,
}

impl CounterActor {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel::<CounterMsg>();
        thread::spawn(move || {
            let mut state: i64 = 0;
            for msg in rx.iter() {
                match msg {
                    CounterMsg::Increment(n) => state += n,
                    CounterMsg::Decrement(n) => state -= n,
                    CounterMsg::GetValue(reply) => { reply.send(state).ok(); }
                    CounterMsg::Shutdown => break,
                }
            }
        });
        CounterActor { tx }
    }

    fn increment(&self, n: i64) { self.tx.send(CounterMsg::Increment(n)).ok(); }
    fn get_value(&self) -> i64 {
        let (tx, rx) = mpsc::channel();
        self.tx.send(CounterMsg::GetValue(tx)).ok();
        rx.recv().unwrap_or(0)
    }
    fn shutdown(self) { self.tx.send(CounterMsg::Shutdown).ok(); }
}
```

The `GetValue` message includes a `mpsc::Sender<i64>` — a one-shot reply channel. The actor sends the value through it; the caller blocks on `rx.recv()`. This request-reply pattern is the foundation of ask-pattern actors.

State is never shared — only the `Sender<CounterMsg>` handle is shared. Multiple threads can safely call `increment` concurrently because `Sender::send` is `Send + Clone`.

## OCaml Approach

```ocaml
type counter_msg =
  | Increment of int
  | Decrement of int
  | GetValue of int Event.channel
  | Shutdown

let spawn_counter () =
  let ch = Event.new_channel () in
  let thread = Thread.create (fun () ->
    let state = ref 0 in
    let rec loop () = match Event.sync (Event.receive ch) with
      | Increment n -> state := !state + n; loop ()
      | Decrement n -> state := !state - n; loop ()
      | GetValue reply ->
        Event.sync (Event.send reply !state); loop ()
      | Shutdown -> ()
    in loop ()
  ) () in
  (ch, thread)
```

OCaml's `Event.channel` is synchronous (rendezvous). For true async actors with buffered mailboxes, `Domainslib.Chan` or `Lwt_mvar` is used. The pattern is structurally identical: state owned by the thread, messages via a channel, reply via a one-shot channel in the message.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Mailbox | `mpsc::Receiver<Msg>` — buffered, async | `Event.channel` — synchronous rendezvous |
| Reply channel | `mpsc::Sender<T>` embedded in message | `Event.channel` embedded in message |
| State ownership | Actor thread exclusively owns it | Actor thread exclusively owns it |
| Message passing | `Send + 'static` bounds | GC handles ownership |

The actor model avoids lock contention by design: no two threads ever access `state` simultaneously because messages are processed sequentially. Actors compose naturally — one actor can send messages to another.

## Exercises

1. Implement a `BankActor` with `Deposit`, `Withdraw`, `Balance`, and `Shutdown` messages. `Withdraw` returns `Err` via the reply channel if balance would go negative.
2. Implement a supervisor: if a worker actor panics, the supervisor detects the dead thread and spawns a replacement.
3. Chain two actors: `LoggingActor` forwards messages to `CounterActor` and logs each operation.
4. Implement an `ActorRef<Msg>` wrapper around `Sender<Msg>` for ergonomic message sending.
5. Rewrite using `tokio::sync::mpsc` and `tokio::task::spawn` for async actor execution.
