**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[select-pattern on hightechmind.io](https://hightechmind.io/posts/functional-rust/select-pattern)

---

## Problem Statement

Implement a select pattern that multiplexes over multiple channels — analogous to Unix `select(2)` or OCaml's `Event.choose`. Use `try_recv` polling with `thread::yield_now` to non-blockingly check each channel in a loop, returning the first available value. Model the result as an enum `Selected<A, B>` that indicates which channel produced a value.

## Learning Outcomes

- Implement `select<A, B>(rx1, rx2) -> Selected<A, B>` using `try_recv` on each channel
- Handle `TryRecvError::Empty` (channel has no item yet) vs `TryRecvError::Disconnected` (channel closed)
- Track `r1_closed` / `r2_closed` flags and return `Selected::BothClosed` when all channels close
- Use `thread::yield_now()` to avoid busy-spinning the CPU
- Understand the limitations vs real `select`/`epoll` and why `crossbeam::channel::select!` is preferred for production

## Rust Application

```rust
#[derive(Debug, PartialEq)]
enum Selected<A, B> { Left(A), Right(B), BothClosed }

fn select<A, B>(rx1: &mpsc::Receiver<A>, rx2: &mpsc::Receiver<B>) -> Selected<A, B> {
    let (mut r1_closed, mut r2_closed) = (false, false);
    loop {
        if !r1_closed {
            match rx1.try_recv() {
                Ok(v)                            => return Selected::Left(v),
                Err(TryRecvError::Disconnected)  => r1_closed = true,
                Err(TryRecvError::Empty)         => {}
            }
        }
        if !r2_closed {
            match rx2.try_recv() {
                Ok(v)                            => return Selected::Right(v),
                Err(TryRecvError::Disconnected)  => r2_closed = true,
                Err(TryRecvError::Empty)         => {}
            }
        }
        if r1_closed && r2_closed { return Selected::BothClosed; }
        thread::yield_now();
    }
}
```

`try_recv` is non-blocking: it returns immediately with `Ok(v)`, `Err(Empty)`, or `Err(Disconnected)`. The polling loop alternates between channels until one delivers a value or both are closed.

`thread::yield_now()` hints to the OS scheduler to run other threads, avoiding 100% CPU usage in the polling loop. A `Duration::from_micros(10)` sleep would reduce CPU usage further at the cost of latency.

The `crossbeam` crate provides `crossbeam::select!` — a macro that efficiently blocks on multiple channels without polling, using OS-level synchronization.

## OCaml Approach

```ocaml
open Event

type ('a, 'b) selected = Left of 'a | Right of 'b | BothClosed

let select ch1 ch2 =
  let e1 = wrap (receive ch1) (fun v -> Left v) in
  let e2 = wrap (receive ch2) (fun v -> Right v) in
  sync (choose [e1; e2])

(* Lwt version *)
let lwt_select p1 p2 =
  Lwt.pick [
    Lwt.map (fun v -> `Left v)  p1;
    Lwt.map (fun v -> `Right v) p2;
  ]
```

OCaml's `Event.choose` selects non-deterministically from a list of events — whichever event fires first is returned. `Lwt.pick` does the same for promises. Both are blocking (no polling loop), making them more efficient than the Rust try_recv loop.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Standard select | None — use `crossbeam::select!` | `Event.choose` or `Lwt.pick` |
| Polling approach | `try_recv` loop + `yield_now` | Not needed — blocking select |
| CPU usage | Polling wastes CPU | Blocking is efficient |
| Multiple channels | Manual loop | `choose [e1; e2; e3; ...]` list |

For production use, `crossbeam::channel::select!` is the correct tool — it uses OS synchronization primitives for efficient multi-channel blocking. The polling approach here illustrates the mechanics.

## Exercises

1. Extend `select` to work with three channels using a three-way enum `Selected3<A, B, C>`.
2. Add a timeout: return `Selected::Timeout` if no message arrives within a given `Duration`.
3. Implement `select_all<T>(receivers: &[Receiver<T>]) -> Option<T>` that returns the first available value from any channel in a list.
4. Rewrite `select` using `crossbeam::select!` and compare the implementation complexity.
5. Implement a load balancer: N producers send to one channel; M consumers each call `select` over N channels and process the first available item.
