# 342: Async File and Network I/O

**Difficulty:** 3  **Level:** Advanced

Perform I/O without blocking your thread — offload blocking work to threads, use non-blocking sockets for network code.

## The Problem This Solves

Standard file I/O in Rust (`std::fs`) is synchronous: your thread sits idle while the kernel copies bytes from disk. In a server or pipeline that handles many concurrent operations, this idle time compounds into latency and wasted throughput. The solution is to either use a full async runtime (Tokio/async-std) or simulate async behavior using threads and channels.

Network I/O has the same problem but a different solution: `TcpListener` and `TcpStream` support `set_nonblocking(true)`, which makes `accept()` and `read()` return immediately with `WouldBlock` instead of sleeping. This lets you multiplex many connections on one thread using an event loop or selector.

Understanding both approaches — thread-offloaded blocking I/O and non-blocking socket I/O — gives you the mental model for what async runtimes actually do under the hood.

## The Intuition

Blocking I/O is like asking a question and then freezing until you get the answer. Non-blocking I/O is like posting a sticky note ("call me when ready") and going off to do other work. Thread offload is the middle ground: you hand the blocking call to another thread so *your* thread stays free.

`mpsc::channel` is the perfect glue: the worker thread sends its result down the channel when done; your thread `recv()`s at exactly the moment it needs the value.

## How It Works in Rust

1. **`spawn_io_task`** — creates an `mpsc::channel`, spawns a thread that runs the closure and sends its result, returns the `Receiver`.
2. **Do other work** while the task runs — the main thread isn't blocked.
3. **`rx.recv()`** — block only when you actually need the result.
4. **Non-blocking listener** — `TcpListener::bind()` then `set_nonblocking(true)`. `accept()` returns `Err(WouldBlock)` immediately when no connection is waiting.
5. **Parallel I/O** — collect multiple `Receiver`s, then drain them in order.

```rust
let rx = spawn_io_task(|| expensive_read(path));
do_other_work();
let result = rx.recv().unwrap(); // only blocks here
```

## What This Unlocks

- **Async-style concurrency without a runtime** — pure `std::thread` + `mpsc` handles many real-world cases.
- **Non-blocking network primitives** — understanding `WouldBlock` demystifies how `epoll`/`kqueue`-based runtimes work.
- **Parallel pipelines** — fan out N independent I/O tasks, collect results in order.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Async file read | `Lwt_io.read_file` | `thread::spawn` + `mpsc::channel` |
| Non-blocking socket | `Unix.set_nonblock` | `listener.set_nonblocking(true)` |
| `WouldBlock` signal | `Unix.EAGAIN` | `io::ErrorKind::WouldBlock` |
| Parallel tasks | `Lwt.all` / `Lwt_list.map_p` | `Vec<Receiver<T>>` drain |
