**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

[work-queue on hightechmind.io](https://hightechmind.io/posts/functional-rust/work-queue)

---

## Problem Statement

Implement a thread pool / work queue with `N` worker threads sharing a single `mpsc::Receiver<Task>` wrapped in `Arc<Mutex<Receiver>>`. Worker threads loop: lock the receiver, dequeue one task, unlock, execute. When the `ThreadPool` is dropped, the channel closes and workers exit cleanly.

## Learning Outcomes

- Define `type Task = Box<dyn FnOnce() + Send + 'static>` for type-erased, owned closures
- Share a single `Receiver<Task>` across all workers using `Arc<Mutex<Receiver<Task>>>`
- Workers loop: `lock` → `recv()` (blocks for next task) → `unlock` → execute task
- Implement `Drop` for `ThreadPool` — joining all workers ensures tasks complete before the pool is destroyed
- Understand why `Mutex` is needed: `Receiver<T>` is not `Sync` (only one thread can receive at a time)

## Rust Application

```rust
type Task = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    sender: mpsc::Sender<Task>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Task>();
        let receiver = Arc::new(Mutex::new(receiver));  // shared across workers
        let workers = (0..size)
            .map(|_| {
                let rx = Arc::clone(&receiver);
                thread::spawn(move || loop {
                    let task = {
                        let lock = rx.lock().unwrap();
                        lock.recv()  // blocks until task arrives or channel closes
                    };  // lock released here
                    match task {
                        Ok(f) => f(),
                        Err(_) => break,  // channel closed → exit
                    }
                })
            })
            .collect();
        ThreadPool { sender, workers }
    }

    fn submit<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.sender.send(Box::new(f)).unwrap();
    }
}
```

The critical pattern: lock the `Receiver`, call `recv()` to dequeue one task, then drop the lock *before* executing the task. If the task were executed while holding the lock, only one worker would ever run (the others would spin on the lock).

When `ThreadPool` is dropped: `sender` drops → channel closes → `recv()` returns `Err` in all workers → workers exit their loops → `JoinHandle::join()` in `Drop` waits for all to finish.

## OCaml Approach

```ocaml
(* OCaml 5.0+: Domainslib.Task.pool *)
let pool = Domainslib.Task.setup_pool ~num_domains:4 ()

let submit_task pool f =
  Domainslib.Task.run pool (fun () -> f ())

(* Manual thread pool with Queue + Mutex + Condition *)
type 'a pool = {
  queue: 'a Queue.t;
  mutex: Mutex.t;
  cond: Condition.t;
  mutable running: bool;
}

let dequeue p =
  Mutex.lock p.mutex;
  while Queue.is_empty p.queue && p.running do
    Condition.wait p.cond p.mutex
  done;
  let task = if Queue.is_empty p.queue then None else Some (Queue.pop p.queue) in
  Mutex.unlock p.mutex;
  task
```

OCaml's `Domainslib.Task` provides a production-ready parallel task pool for OCaml 5.0+. The manual implementation mirrors Rust's `Arc<Mutex<Receiver>>` pattern using `Queue + Mutex + Condition`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Task type | `Box<dyn FnOnce() + Send + 'static>` | `unit -> unit` function |
| Shared queue | `Arc<Mutex<Receiver<Task>>>` | `Queue + Mutex + Condition` |
| Channel close | `Sender` drop propagates to `recv()` | Manual `running = false` + broadcast |
| `Drop` for join | `impl Drop for ThreadPool { ... }` | Explicit `shutdown + join` |

The `Arc<Mutex<Receiver>>` trick is idiomatic Rust for fan-out from a single channel to multiple consumers. The lock is held only during the `recv()` call (microseconds), so contention is minimal.

## Exercises

1. Add a `submit_with_result<T: Send + 'static>(f: FnOnce() -> T) -> impl Future<Output=T>` using `oneshot` channels.
2. Implement `ThreadPool::shutdown_graceful()` that waits for all queued tasks to complete before joining workers.
3. Add a task priority queue: use `BinaryHeap<(Priority, Task)>` instead of the FIFO channel.
4. Track in-flight task count with `Arc<AtomicUsize>` and expose `pending_tasks() -> usize`.
5. Benchmark the thread pool against `rayon::ThreadPool` for 10,000 CPU-bound tasks.
