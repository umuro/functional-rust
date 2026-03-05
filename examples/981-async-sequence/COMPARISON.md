# Sequential Async Chain — Comparison

## Core Insight
Sequential async chains are **monadic do-notation**: each step depends on the previous. Both languages provide sugar for this — OCaml's `let*` (ppx_let / Lwt.Syntax) and Rust's `.await` on sequential lines. Values computed in earlier steps are in scope for later steps.

## OCaml Approach
- `let* x = fut in ...` desugars to `Lwt.bind fut (fun x -> ...)`
- Requires `open Lwt.Syntax` or ppx_let
- Short-circuit via `Lwt_result` and `let*?`
- Each step is truly sequential — Lwt schedules them one after another

## Rust Approach
- Sequential `.await` calls read like normal imperative code
- Variables from earlier awaits are in scope for later ones (captures)
- `?` operator provides short-circuit error propagation (like `let*?`)
- The compiler generates a state machine — no runtime overhead per step

## Comparison Table

| Concept               | OCaml (Lwt)                           | Rust                              |
|-----------------------|---------------------------------------|-----------------------------------|
| Sequential bind       | `let* x = f () in let* y = g x in …` | `let x = f().await; let y = g(x).await` |
| Error short-circuit   | `let*? x = f () in …`                | `let x = f().await?;`             |
| Later steps see earlier| Yes — closure captures `x`           | Yes — in same async fn scope      |
| Sugar requires        | `open Lwt.Syntax`                     | Just `async fn` + `.await`       |
| Execution order       | Strict left-to-right                  | Strict left-to-right              |
| Parallelism           | No (use `Lwt.both` / `Lwt.join`)      | No (use `join!` or threads)       |

## std vs tokio

| Aspect | std version | tokio version |
|--------|-------------|---------------|
| **Runtime** | OS threads via `std::thread` | Async tasks on tokio runtime |
| **Synchronization** | `std::sync::Mutex`, `Condvar` | `tokio::sync::Mutex`, channels |
| **Channels** | `std::sync::mpsc` (unbounded) | `tokio::sync::mpsc` (bounded, async) |
| **Blocking** | Thread blocks on lock/recv | Task yields, runtime switches tasks |
| **Overhead** | One OS thread per task | Many tasks per thread (M:N) |
| **Best for** | CPU-bound, simple concurrency | I/O-bound, high-concurrency servers |
