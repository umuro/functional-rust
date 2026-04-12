**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[async-map on hightechmind.io](https://hightechmind.io/posts/functional-rust/async-map)

---

## Problem Statement

Demonstrate mapping over async futures in Rust — the async equivalent of `Lwt.map f promise`. Show that `async { fut.await * 2 }` is the Rust idiom for `Lwt.map (fun x -> x * 2) fut`. Implement typed async transformations (`map_double`, `map_to_string`) and chains of maps. Connect to the Functor typeclass: `Future` is a functor where `map` preserves the computational context.

## Learning Outcomes

- Implement `async fn map_double(fut: impl Future<Output=i32>) -> i32` as `async { fut.await * 2 }`
- Implement `async fn map_to_string(fut: impl Future<Output=i32>) -> String` as `async { fut.await.to_string() }`
- Chain maps: `map_to_string(map_double(base_value()))` — no explicit bind needed
- Recognize `async { f(fut.await) }` as the Rust equivalent of OCaml's `Lwt.map f fut`
- Understand that `async fn` returning a value is `Lwt.return`; `.await` is `Lwt.bind`

## Rust Application

```rust
async fn base_value() -> i32 { 5 }

// Lwt.map (fun x -> x * 2) fut  ≡  async { fut.await * 2 }
async fn map_double(fut: impl Future<Output = i32>) -> i32 {
    fut.await * 2
}

async fn map_to_string(fut: impl Future<Output = i32>) -> String {
    fut.await.to_string()
}

// Chain: map_to_string(map_double(base_value()))
async fn map_chain() -> String {
    let doubled = map_double(base_value());
    map_to_string(doubled).await
}

// Compose: f then g as a single async transformation
async fn compose_maps() -> String {
    let result = async { base_value().await * 3 + 1 }.await;
    result.to_string()
}
```

`async { fut.await * 2 }` is the minimal "map over a future" expression. The `async` block creates a new future that: polls `fut` until ready, multiplies by 2, and returns `Poll::Ready(value * 2)`.

The chain `map_to_string(map_double(base_value()))` requires only the outer `.await` — the intermediate future is awaited inside `map_to_string`. This is the async equivalent of `g(f(x))` where `g = map_to_string` and `f = map_double`.

## OCaml Approach

```ocaml
open Lwt

let base_value () = return 5

(* Lwt.map: transform the result of a promise *)
let map_double fut = Lwt.map (fun x -> x * 2) fut
let map_to_string fut = Lwt.map string_of_int fut

(* Chain maps *)
let map_chain () =
  base_value ()
  |> map_double
  |> map_to_string

(* Using let* *)
let map_chain_letstar () =
  let* x = base_value () in
  let* y = return (x * 2) in
  return (string_of_int y)
```

`Lwt.map f p` allocates a new promise that resolves with `f(v)` when `p` resolves with `v`. In Rust, `async { p.await |> f }` achieves the same without a named function. The `|>` pipeline in OCaml reads as "take base_value, double it, stringify it" — identical to the Rust chain.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Map a future | `async { fut.await * 2 }` | `Lwt.map (fun x -> x * 2) fut` |
| Chain maps | Compose async fns | `\|>` with `Lwt.map` |
| Named map | `async fn map_double(fut) -> i32 { fut.await * 2 }` | `let map_double = Lwt.map (fun x -> x * 2)` |
| Functor law | Not enforced by type system | Not enforced |

`async { fut.await * 2 }` is a "lifted" function application. It demonstrates that `Future` forms a functor: `map id = id` and `map (g ∘ f) = map g ∘ map f` hold by construction for `async` blocks.

## Exercises

1. Implement a generic `future_map<T, U, F: Future<Output=T>>(fut: F, f: impl FnOnce(T) -> U) -> impl Future<Output=U>`.
2. Verify the functor identity law: `future_map(fut, |x| x)` equals `fut` in output value.
3. Implement `future_map2<A, B, C>(fa, fb, f)` — combine two independent futures with a binary function.
4. Build a pipeline of five async transformations chained with `future_map`.
5. Show how `future_map(future_map(fut, f), g)` equals `future_map(fut, |x| g(f(x)))` with a test.
