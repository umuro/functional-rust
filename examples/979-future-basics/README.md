[future-basics on hightechmind.io](https://hightechmind.io/posts/functional-rust/future-basics)

---

## Problem Statement

Introduce Rust's `Future` trait and `async`/`await` syntax by implementing a minimal synchronous executor (`block_on`) using `Pin`, `Context`, and `Waker`. Show that `async fn` desugars to a state machine implementing `Future`, that `.await` is monadic bind, and that sequential `async` code resembles OCaml's `Lwt` monad with `let*` syntax.

## Learning Outcomes

- Implement `block_on<F: Future>(fut: F) -> F::Output` using `Pin::new_unchecked` and a no-op waker
- Understand that `async fn compute() -> T` desugars to `fn compute() -> impl Future<Output=T>`
- Recognize `.await` as monadic bind: `x.await` extracts `x`'s value and continues the computation
- Implement sequential `async` chains: `let x = f().await; let y = g(x).await; Ok(y)`
- Understand the connection to OCaml's `Lwt.bind` and `let*` syntax for async code

## Rust Application

```rust
fn block_on<F: Future>(mut fut: F) -> F::Output {
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };
    fn noop(_: *const ()) {}
    fn noop_clone(p: *const ()) -> RawWaker { RawWaker::new(p, &VTABLE) }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("Future not ready — use a real executor"),
    }
}

async fn compute_value() -> i32 { 42 }

async fn compute_and_add() -> i32 {
    let x = compute_value().await;  // bind: await = >>=
    x + 1
}
```

The minimal `block_on` creates a no-op waker (all callbacks do nothing) and polls the future once. For immediately-ready futures (which return `Poll::Ready` on first poll), this is correct. Real executors handle `Poll::Pending` by parking the thread and re-polling when the waker fires.

`async fn f() -> T` desugars to an anonymous struct implementing `Future<Output=T>`. The compiler generates a state machine with one state per `.await` point. Each `poll` call advances the state machine by one `.await`.

## OCaml Approach

```ocaml
(* OCaml: Lwt for async *)
open Lwt.Syntax

let compute_value () = Lwt.return 42

let compute_and_add () =
  let* x = compute_value () in  (* x = await compute_value() *)
  Lwt.return (x + 1)

(* Sequential chain *)
let full_pipeline () =
  let* a = step_one () in
  let* b = step_two a in
  let* c = step_three b in
  Lwt.return c

(* Lwt.bind is >>=; let* is syntactic sugar *)
(* Rust .await ≡ OCaml let* ≡ Haskell >>= *)
```

OCaml's `Lwt.return` ↔ `async { value }`. `Lwt.bind p f` ↔ `async { f(p.await) }`. The `let*` syntax reads identically to Rust's `let x = p.await; ...` — both are sequential monadic composition.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Async primitive | `Future` trait + state machine | `Lwt.t` promise/deferred |
| Sequencing | `.await` | `let*` / `Lwt.bind` |
| Parallel | `tokio::join!` | `Lwt.both` / `Lwt.all` |
| Runtime | `tokio` / `async-std` (external) | `Lwt` (external) or `Eio` (newer) |
| Stack pinning | `Pin<&mut F>` required | GC manages lifetime |

`async`/`await` in Rust compiles to zero-overhead state machines — no heap allocation per future by default. OCaml's `Lwt` allocates a heap promise for each deferred computation.

## Exercises

1. Write an `async fn pipeline(x: i32) -> String` that chains three `async` transformations sequentially.
2. Implement `async_map<T, U, F: Future<Output=T>>(fut: F, f: impl Fn(T) -> U) -> U` as `async { f(fut.await) }`.
3. Implement `async_and_then<T, U>(fut: impl Future<Output=T>, f: impl Fn(T) -> impl Future<Output=U>) -> U`.
4. Verify the monad left-identity law: `async { f(x).await }` equals `f(x)` for pure `f`.
5. Add a real runtime dependency (`tokio` or `smol`) and rewrite `compute_and_add` using it.
