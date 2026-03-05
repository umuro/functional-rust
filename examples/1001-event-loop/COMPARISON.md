# Simple Event Loop — Comparison

## Core Insight
An event loop is an **infinite fold**: `state = fold dispatch initial_state event_stream`. Making `dispatch` a pure function `(State, Event) -> State` gives testability, reproducibility, and the foundation for time-travel debugging (like Redux).

## OCaml Approach
- Event type as algebraic type: `type event = Click | KeyPress | ...`
- Handler record: `{ on_click; on_key; on_timer; on_network }`
- `run_event_loop` is recursive `loop state events` — structural recursion
- Pattern match on `Quit` to stop
- State as immutable record with record update syntax `{ s with clicks = ... }`

## Rust Approach
- `enum Event { Click { x, y }, KeyPress(char), ... }` — same ADT pattern
- `dispatch(state: AppState, event: &Event) -> AppState` — pure function
- `run_until_quit` uses a `for` loop with `break` on `Quit`
- `EventLoop` struct wraps `VecDeque<Event>` for real-world queue usage
- `AppState { clicks: state.clicks + 1, ..state }` struct update syntax mirrors OCaml

## Comparison Table

| Concept              | OCaml                                 | Rust                                   |
|----------------------|---------------------------------------|----------------------------------------|
| Event type           | `type event = Click \| KeyPress \| ...` | `enum Event { Click { x, y }, ... }` |
| State update         | `{ s with clicks = s.clicks + 1 }`    | `AppState { clicks: s.clicks + 1, ..s }` |
| Dispatch function    | `handler.on_click x y state`          | `dispatch(state, &event)` match       |
| Loop idiom           | Tail-recursive `loop state events`    | `for event in events { match event }` |
| Stop at Quit         | `Quit :: _ -> state` (base case)      | `Event::Quit => break`                 |
| Queue-based          | `Queue.pop` + `while`                | `VecDeque::pop_front()` + `while let` |
| Testability          | Pure `run_event_loop` function        | Pure `dispatch` + pure `run_until_quit` |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
