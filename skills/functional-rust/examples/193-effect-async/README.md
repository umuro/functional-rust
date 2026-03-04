# 193: Effect Handlers for Async IO

**Difficulty:** 4  **Level:** Expert

Model cooperative multitasking as algebraic effects — tasks voluntarily yield control, a scheduler decides who runs next.

## The Problem This Solves

Async programming has two very different faces. The first is *what your code does*: fetch this URL, then parse it, then write the result. That logic is pure and sequential. The second is *how the runtime manages waiting*: when task A is blocked, run task B; when A's data arrives, resume A. Those two concerns get tangled together when async is built into the language syntax.

OCaml 5's algebraic effects keep them separate. A task performs an `Yield` effect — it just says "I'm willing to pause". The *handler* (the scheduler) catches that effect and decides what to do: switch to another task, log the pause, run in a single thread, or run in a thread pool. The same task code runs under any handler.

Rust's async/await compiles tasks to state machines, which is efficient but bakes in a specific model. To demonstrate the effect-handler idea, this example implements it explicitly: tasks communicate with a round-robin scheduler via `mpsc` channels. Each task's "yield" is a channel send + blocking receive. The scheduler is the handler.

## The Intuition

Imagine two cooks in a kitchen sharing one stove. Cook A starts a task, then says "I'll wait — someone else can use the stove." Cook B steps in, does some work, then says the same. The *head chef* (scheduler) decides who gets the stove next.

"Yielding" is not sleeping — it's a voluntary handoff. The scheduler queues tasks and resumes them in order. No task runs at the same time; they take turns. This is cooperative (not preemptive) multitasking.

Each task gets a `YieldHandle`. Calling `handle.yield_now()` sends a message to the scheduler ("I'm pausing") and then blocks, waiting for the scheduler to send back a resume signal. The scheduler drives the whole thing from a queue of waiting tasks.

## How It Works in Rust

```rust
struct YieldHandle {
    yield_tx: Sender<String>,  // tell scheduler "I'm pausing"
    resume_rx: Receiver<()>,   // wait for "you may continue"
}

impl YieldHandle {
    fn yield_now(&self, task_name: &str) {
        self.yield_tx.send(task_name.to_string()).ok(); // signal pause
        self.resume_rx.recv().ok();                     // block until resumed
    }
}

// Scheduler: runs tasks round-robin, one step at a time
fn run_scheduler<F1, F2>(task_a: F1, task_b: F2) -> Vec<String>
where
    F1: FnOnce(YieldHandle) + Send + 'static,
    F2: FnOnce(YieldHandle) + Send + 'static,
{
    // Each task gets: a channel to signal yield, a channel to receive resume
    let (yield_tx, yield_rx) = mpsc::channel::<String>();
    // ... set up per-task resume channels, spawn threads, round-robin loop
}
```

Tasks are regular closures that receive a `YieldHandle`. They call `handle.yield_now("task_a")` between steps. The scheduler catches each yield signal, queues the task, and sends a resume when it's that task's turn again. Output is collected from the yield sequence, giving a trace of interleaving.

## What This Unlocks

- **Pluggable schedulers** — swap the round-robin scheduler for a priority queue, work-stealing, or a single-threaded event loop without changing task code.
- **Testing async logic** — run tasks with a deterministic scheduler in tests; no timing dependencies, reproducible ordering.
- **Effect isolation** — the scheduler handles cancellation, timeout, and logging centrally, rather than scattered through each task.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Yield primitive | `perform Yield` | `handle.yield_now()` via channel |
| Scheduler | `effect_handler` continuation capture | `mpsc` channel + thread join |
| Task isolation | Continuations in heap | `std::thread::spawn` |
| Resume | Handler invokes continuation | Scheduler sends `()` on resume channel |
| Handler swap | Different `match_with` call | Different scheduler function |
