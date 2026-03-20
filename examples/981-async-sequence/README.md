[async-sequence on hightechmind.io](https://hightechmind.io/posts/functional-rust/async-sequence)

---

## Problem Statement

Demonstrate sequential async composition in Rust using chained `.await` calls. Each step uses the result of the previous step, mirroring OCaml's `let* x = ... in` sequential Lwt monadic binding. Implement a data-lookup pipeline: fetch a user ID, then use it to fetch the name, then use the name to fetch the email — each step depends on the previous.

## Learning Outcomes

- Implement sequential `async fn` chains where each step `let x = f().await` uses the previous result
- Recognize that `let x = f().await; let y = g(x).await; y` is OCaml's `let* x = f () in let* y = g x in y`
- Understand that sequential `.await` is monadic bind for `Future` — each binds the value from the prior computation
- Implement helper `async fn` that take ownership of parameters via `move` closures
- Compare with parallel join (982) — sequential is ordered, parallel is unordered

## Rust Application

```rust
async fn fetch_user_id() -> u32 { 42 }
async fn fetch_user_name(_id: u32) -> String { "Alice".to_string() }
async fn fetch_user_email(_name: &str) -> String { "alice@example.com".to_string() }

// Sequential: each step waits for the previous
async fn full_lookup() -> (u32, String, String) {
    let id    = fetch_user_id().await;
    let name  = fetch_user_name(id).await;
    let email = fetch_user_email(&name).await;
    (id, name, email)
}

// As a chain using and_then-style nesting
async fn full_lookup_chained() -> String {
    let id = fetch_user_id().await;
    let name = async move { fetch_user_name(id).await }.await;
    async move { fetch_user_email(&name).await }.await
}
```

Sequential `.await` chains are the direct equivalent of monadic `>>=` chains. The Rust compiler generates a state machine with one state per `.await` point — after each await, the state machine parks until the awaited future resolves.

The name/email step requires the result of the previous step: `fetch_user_name(id)` uses `id`, `fetch_user_email(&name)` uses `name`. This dependency makes parallelization impossible — the three operations must run in order.

## OCaml Approach

```ocaml
open Lwt.Syntax

let fetch_user_id () = Lwt.return 42
let fetch_user_name _id = Lwt.return "Alice"
let fetch_user_email _name = Lwt.return "alice@example.com"

(* Sequential with let* *)
let full_lookup () =
  let* id    = fetch_user_id () in
  let* name  = fetch_user_name id in
  let* email = fetch_user_email name in
  Lwt.return (id, name, email)

(* Equivalent with explicit bind *)
let full_lookup_bind () =
  Lwt.bind (fetch_user_id ()) (fun id ->
  Lwt.bind (fetch_user_name id) (fun name ->
  Lwt.bind (fetch_user_email name) (fun email ->
  Lwt.return (id, name, email))))
```

`let*` is syntactic sugar for `Lwt.bind`. The two forms are identical in semantics. Rust's `let x = f().await` is the third form of the same pattern — sequential monadic binding.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Sequential bind | `let x = f().await` | `let* x = f () in` |
| Explicit bind | `f().then(\|x\| g(x))` | `Lwt.bind (f ()) (fun x -> g x)` |
| State machine | Compiler-generated from `async fn` | Lwt heap-allocated continuations |
| Ownership | `id` moved into `fetch_user_name(id)` | GC manages lifetime |

Sequential `.await` is the building block of any async workflow. Use it when each step depends on the previous result. Use parallel join (example 982) when steps are independent.

## Exercises

1. Add error handling: change the functions to return `Result<T, String>` and use `?` inside `async fn`.
2. Implement `retry_async<F, T>(n: usize, f: F) -> Result<T, String>` that retries a failing async fn up to `n` times.
3. Implement a `timeout_async(duration, fut)` that returns `Err` if the future does not complete within `duration`.
4. Chain 10 dependent async steps and verify that the output is correct.
5. Convert `full_lookup` to use `tokio` and measure the actual concurrency behavior under a real async runtime.
