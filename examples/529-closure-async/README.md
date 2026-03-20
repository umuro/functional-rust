📖 **[View on hightechmind.io →](https://hightechmind.io/rust/529-closure-async)**

---

# Async Closures

## Problem Statement

Async programming requires composing not just values but futures — asynchronous computations that may yield before completing. A common need is passing callbacks that themselves perform async work: an HTTP client that accepts an async retry handler, a task queue that calls an async processing function per item, or a middleware chain where each layer can await I/O. True `async |x| { ... }` closure syntax is nightly-only in Rust; the stable pattern uses `|x| async move { ... }` — a closure returning a `Future`.

## Learning Outcomes

- How `|x| async { ... }` produces a closure returning an anonymous `Future`
- How `F: FnOnce(T) -> Fut, Fut: Future<Output = U>` bounds express async callbacks
- How to implement `async_map` and `async_filter` over collections using sequential `await`
- Why true `async fn` closures require nightly and what the stable workaround looks like
- Where async callbacks appear: middleware chains, retry logic, async iterators (streams)

## Rust Application

`async_transform<T, U, F, Fut>(value, f)` accepts any `F: FnOnce(T) -> Fut` where `Fut: Future<Output = U>`. `process_with_callback<T, F, Fut>(value, callback)` calls `callback(&value).await` then returns `value` — the callback performs async side effects. `async_map<T, U, F, Fut>(items, f)` processes a `Vec` sequentially, pushing each `f(item).await` result. `async_filter` similarly filters by awaiting a predicate future per item. The pattern `F: Fn(T) -> Fut` where `Fut: Future<Output = U>` is the idiomatic stable substitute for `async Fn`.

Key patterns:
- `F: Fn(T) -> Fut, Fut: Future<Output = U>` — async callback via future-returning closure
- `|x| async move { ... }` — closure producing a named future, borrowing with `move`
- Sequential await loop instead of parallel: `for item in items { results.push(f(item).await); }`

## OCaml Approach

OCaml 5.x uses effect handlers and `Eio` or `Lwt` for async programming. An async callback in Lwt is a function returning `'a Lwt.t`:

```ocaml
let async_map f items =
  Lwt_list.map_s f items  (* map_s = sequential, map_p = parallel *)

let async_filter f items =
  Lwt_list.filter_s f items
```

Lwt's `>>=` (bind) and `let*` syntax serve the same purpose as Rust's `.await`.

## Key Differences

1. **Syntax**: Rust's `.await` is a postfix operator; OCaml/Lwt uses `>>=` infix or `let*` binding syntax — both express sequential async composition.
2. **True async closures**: Rust stable requires the workaround `|x| async { ... }` returning a `Future`; OCaml functions returning `Lwt.t` are the natural async closure form with no special syntax needed.
3. **Parallelism control**: Rust's `async_map` processes sequentially by default; switching to `futures::join_all` enables parallelism; OCaml's `Lwt_list.map_p` enables parallel futures explicitly.
4. **Type complexity**: Rust async callbacks introduce two generic parameters (`F` and `Fut`), making signatures verbose; OCaml's `'a -> 'b Lwt.t` type is concise and uniform.

## Exercises

1. **Parallel async map**: Rewrite `async_map` using `futures::future::join_all` to process items concurrently instead of sequentially, and verify both produce the same output.
2. **Retry with async**: Implement `retry_async<F, Fut, T>(attempts: usize, f: F) -> impl Future<Output = Result<T, String>> where F: Fn() -> Fut, Fut: Future<Output = Result<T, String>>`.
3. **Async fold**: Implement `async_fold<T, U, F, Fut>(items: Vec<T>, init: U, f: F) -> U where F: Fn(U, T) -> Fut, Fut: Future<Output = U>` that accumulates asynchronously.
