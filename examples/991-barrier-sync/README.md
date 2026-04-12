**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐⭐  

[barrier-sync on hightechmind.io](https://hightechmind.io/posts/functional-rust/barrier-sync)

---

## Problem Statement

Demonstrate `std::sync::Barrier` — a synchronization primitive that makes `N` threads wait at a point until all `N` have arrived, then releases all simultaneously. This enables phased computation: Phase 1 runs independently across all threads, then all synchronize at the barrier, then Phase 2 starts simultaneously in all threads.

## Learning Outcomes

- Create a `Barrier::new(n)` and share via `Arc::clone`
- Call `barrier.wait()` — blocks until all `n` threads have called `wait`, then all are released
- Recognize that `BarrierWaitResult::is_leader()` returns true for exactly one thread — useful for single-threaded post-phase work
- Understand the use case: parallel algorithms with synchronization points (e.g., parallel BFS level synchronization)
- Compare with `CountDownLatch` (Java) and OCaml's manual `Mutex + Condvar + counter` equivalent

## Rust Application

```rust
fn barrier_demo() -> (Vec<String>, Vec<String>) {
    let n = 5;
    let barrier = Arc::new(Barrier::new(n));
    let phase1_log = Arc::new(Mutex::new(Vec::new()));
    let phase2_log = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..n)
        .map(|i| {
            let barrier = Arc::clone(&barrier);
            let p1 = Arc::clone(&phase1_log);
            let p2 = Arc::clone(&phase2_log);
            thread::spawn(move || {
                // Phase 1: independent work (different durations)
                thread::sleep(Duration::from_millis(i as u64 * 2));
                p1.lock().unwrap().push(format!("p1:{}", i));

                barrier.wait();  // BARRIER — all must arrive before any proceed

                // Phase 2: all start together
                p2.lock().unwrap().push(format!("p2:{}", i));
            })
        })
        .collect();

    for h in handles { h.join().unwrap(); }
    // ...
}
```

`Barrier::new(5)` means exactly 5 threads must call `wait()` before any proceeds. Thread 0 finishes phase 1 in 0ms; thread 4 finishes in 8ms. All 5 wait at the barrier until thread 4 arrives, then all 5 enter phase 2 simultaneously.

`barrier.wait()` returns a `BarrierWaitResult`. `is_leader()` is true for exactly one thread — the last to arrive. This allows one thread to perform cleanup or setup between phases without a separate synchronization mechanism.

## OCaml Approach

```ocaml
type barrier = {
  target: int;
  mutable count: int;
  mutex: Mutex.t;
  cond: Condition.t;
  mutable generation: int;
}

let create n = { target = n; count = 0; mutex = Mutex.create ();
                  cond = Condition.create (); generation = 0 }

let wait b =
  Mutex.lock b.mutex;
  let gen = b.generation in
  b.count <- b.count + 1;
  if b.count = b.target then begin
    b.count <- 0;
    b.generation <- gen + 1;
    Condition.broadcast b.cond
  end else begin
    while b.generation = gen do
      Condition.wait b.cond b.mutex
    done
  end;
  Mutex.unlock b.mutex
```

OCaml's barrier is manually implemented with `Mutex + Condition + generation counter`. The `generation` counter prevents spurious wakeups from releasing threads before all arrive. `Condition.broadcast` wakes all waiting threads at once — unlike `notify_one`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Built-in barrier | `std::sync::Barrier` | No stdlib; manual with `Mutex + Condition` |
| Leader election | `is_leader()` on `BarrierWaitResult` | Manual with first/last thread logic |
| Broadcast wakeup | Internally uses `notify_all` | `Condition.broadcast` explicitly |
| Generation counter | Internal (handles reuse) | Manual `generation` field |

Barriers enable clean phased parallel algorithms. They are used in parallel BFS (synchronize after each level), parallel matrix operations (synchronize after each row/column pass), and multi-stage simulation.

## Exercises

1. Use the `is_leader()` result to have exactly one thread print "phase barrier reached" between phases.
2. Implement a reusable barrier (can `wait` multiple times for multiple phases) and verify phase isolation.
3. Build a parallel word count: Phase 1 = each thread counts its chunk, barrier, Phase 2 = one thread merges.
4. Implement a `timeout_barrier` that releases all threads if one hasn't arrived within a `Duration`.
5. Demonstrate that removing the barrier causes Phase 2 to start before Phase 1 completes on some threads.
