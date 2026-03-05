📖 **[View on hightechmind.io →](https://hightechmind.io/rust/451-crossbeam-select)**

---

# 451: Crossbeam select!

**Difficulty:** 3  **Level:** Intermediate

Wait on multiple channels simultaneously and handle whichever fires first — Rust's answer to Go's `select`.

## The Problem This Solves

In concurrent systems a thread often needs to react to *whichever event arrives first*: a work item from the job queue, a shutdown signal from the control channel, or a timeout. Without `select!` you're left with `try_recv` polling loops — burning CPU and adding arbitrary latency — or complex `Mutex`/`Condvar` arrangements.

Go programmers reach for `select` constantly; it's a first-class language construct there. Crossbeam brings the same power to Rust as a macro. You declare a set of channel operations; the runtime blocks until one is ready and executes exactly that branch. Fairness and correct wakeup are handled for you.

This is the primitive that makes event-driven concurrent code clean without async.

## The Intuition

You're a dispatcher waiting by several phones. You don't check each one in turn — you'd miss calls. Instead you sit with all phones in front of you, waiting. The moment *any* one rings, you answer it and handle that call. That's `select!`.

## How It Works in Rust

1. **Import and set up channels**:
   ```rust
   use crossbeam_channel::{select, bounded, tick};
   let (jobs_tx, jobs_rx) = bounded::<String>(10);
   let (stop_tx, stop_rx) = bounded::<()>(1);
   ```
2. **Write the `select!` block**:
   ```rust
   loop {
       select! {
           recv(jobs_rx) -> msg => {
               println!("job: {:?}", msg);
           }
           recv(stop_rx) -> _ => {
               println!("shutting down");
               break;
           }
       }
   }
   ```
3. **Tick channels** — `crossbeam_channel::tick(Duration)` produces a message every N seconds, useful as a timeout arm:
   ```rust
   let ticker = tick(Duration::from_secs(1));
   select! {
       recv(jobs_rx) -> msg => process(msg),
       recv(ticker)  -> _   => println!("heartbeat"),
   }
   ```
4. **Send arms** — `select!` also supports `send(tx) -> result` for non-blocking sends to any ready receiver.

## What This Unlocks

- **Clean shutdown patterns** — a dedicated stop channel pairs naturally with `select!`; workers exit the moment the signal arrives.
- **Timeout handling** — `tick` or `after(Duration)` as a select arm eliminates ad-hoc `recv_timeout` calls.
- **Fan-in** — merge multiple channels into one handler without a dedicated merger thread.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Multi-channel wait | `Event.choose` / `Event.sync` | `crossbeam::select!` macro |
| Syntax | First-class event algebra | Macro with `recv`/`send` arms |
| Timeout arm | `Event.wrap` + timer | `tick()` or `after()` channel |
| Fairness | Library-defined | Crossbeam: pseudo-random fair |
