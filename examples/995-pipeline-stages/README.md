[pipeline-stages on hightechmind.io](https://hightechmind.io/posts/functional-rust/pipeline-stages)

---

## Problem Statement

Build a composable N-stage streaming pipeline where each stage is independently reusable: `map_stage`, `filter_stage`, `flat_map_stage`, and `batch_stage`. Each stage returns a `Receiver<T>` that can be threaded into the next stage. This is a richer version of example 984, adding filtering and batching primitives.

## Learning Outcomes

- Implement `map_stage<T, U, F>(rx, f) -> Receiver<U>` — already seen in 984; revisit as a building block
- Implement `filter_stage<T, F: Fn(&T) -> bool>(rx, pred) -> Receiver<T>` — forward only matching items
- Implement `flat_map_stage<T, U, F: Fn(T) -> Vec<U>>(rx, f) -> Receiver<U>` — expand one item into many
- Implement `batch_stage<T>(rx, n) -> Receiver<Vec<T>>` — collect `n` items then forward as a batch
- Compose all four stages into a concrete pipeline and verify output

## Rust Application

```rust
fn map_stage<T, U, F>(rx: mpsc::Receiver<T>, f: F) -> mpsc::Receiver<U>
where T: Send + 'static, U: Send + 'static, F: Fn(T) -> U + Send + 'static,
{
    let (tx, out) = mpsc::channel();
    thread::spawn(move || { for item in rx.iter() { tx.send(f(item)).unwrap(); } });
    out
}

fn filter_stage<T, F>(rx: mpsc::Receiver<T>, pred: F) -> mpsc::Receiver<T>
where T: Send + 'static, F: Fn(&T) -> bool + Send + 'static,
{
    let (tx, out) = mpsc::channel();
    thread::spawn(move || {
        for item in rx.iter() { if pred(&item) { tx.send(item).unwrap(); } }
    });
    out
}

fn flat_map_stage<T, U, F>(rx: mpsc::Receiver<T>, f: F) -> mpsc::Receiver<U>
where T: Send + 'static, U: Send + 'static, F: Fn(T) -> Vec<U> + Send + 'static,
{
    let (tx, out) = mpsc::channel();
    thread::spawn(move || {
        for item in rx.iter() {
            for x in f(item) { tx.send(x).unwrap(); }
        }
    });
    out
}

fn batch_stage<T: Send + 'static>(rx: mpsc::Receiver<T>, n: usize) -> mpsc::Receiver<Vec<T>> {
    let (tx, out) = mpsc::channel();
    thread::spawn(move || {
        let mut batch = Vec::with_capacity(n);
        for item in rx.iter() {
            batch.push(item);
            if batch.len() == n {
                tx.send(std::mem::replace(&mut batch, Vec::with_capacity(n))).unwrap();
            }
        }
        if !batch.is_empty() { tx.send(batch).unwrap(); }
    });
    out
}
```

`filter_stage` preserves type — `T` in, `T` out, only dropping items. `flat_map_stage` calls `f(item)` which returns `Vec<U>`; each element of the vec is forwarded individually — one input can produce 0, 1, or many outputs.

`batch_stage` uses `std::mem::replace` to swap the accumulated batch with a fresh `Vec` — avoids copying by taking ownership of the full batch.

## OCaml Approach

```ocaml
let map_stage rx f =
  let (tx, out) = Domainslib.Chan.make_unbounded () in
  Domain.spawn (fun () ->
    let rec loop () = match Domainslib.Chan.recv rx with
      | None -> Domainslib.Chan.close tx
      | Some v -> Domainslib.Chan.send tx (Some (f v)); loop ()
    in loop ()
  ) |> ignore;
  out

let filter_stage rx pred =
  let (tx, out) = Domainslib.Chan.make_unbounded () in
  Domain.spawn (fun () ->
    let rec loop () = match Domainslib.Chan.recv rx with
      | None -> Domainslib.Chan.close tx
      | Some v ->
        if pred v then Domainslib.Chan.send tx (Some v);
        loop ()
    in loop ()
  ) |> ignore;
  out
```

OCaml uses `None` as a sentinel to close channels (since `Domainslib.Chan` lacks automatic close on owner drop). Rust's channel closes when the `Sender` is dropped — automatic sentinel via ownership.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Channel close | Automatic on `Sender` drop | Manual `None` sentinel |
| `filter` predicate | `Fn(&T) -> bool` — borrows | `'a -> bool` — same concept |
| `flat_map` expansion | `Vec<U>` return, forward each | `list` return, forward each |
| Batch | `std::mem::replace` to swap | `Buffer` or `Queue` based |

Each stage in this pipeline runs in its own OS thread — true parallel streaming. The channel acts as a bounded FIFO between stages (unbounded `mpsc`; use `sync_channel(n)` for backpressure).

## Exercises

1. Add a `reduce_stage<T, R, F>(rx, init, f) -> R` that is the terminal sink stage.
2. Implement `tee_stage<T: Clone>(rx) -> (Receiver<T>, Receiver<T>)` — broadcast each item to two downstream stages.
3. Add backpressure: change all stage channels to `sync_channel(64)` and observe producer slowing.
4. Build a complete text-processing pipeline: source → split_words → filter_stopwords → count_frequencies → top10.
5. Benchmark the N-stage pipeline processing 1,000,000 integers against a sequential iterator chain with the same operations.
