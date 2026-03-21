**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[channel-pipeline on hightechmind.io](https://hightechmind.io/posts/functional-rust/channel-pipeline)

---

## Problem Statement

Build a multi-stage processing pipeline where each stage reads from an input channel, applies a transformation, and writes to an output channel. Each stage runs in its own thread. Implement `pipeline_stage<T, U, F>` as a reusable building block that returns the output `Receiver`, enabling declarative pipeline construction.

## Learning Outcomes

- Implement `pipeline_stage<T, U, F>(rx: Receiver<T>, f: F) -> Receiver<U>` that spawns a worker thread
- Chain stages: `rx1 = pipeline_stage(rx0, double); rx2 = pipeline_stage(rx1, add1)`
- Use `rx.iter()` inside each stage — naturally stops when the upstream channel closes
- Recognize that dropping `tx_out` when the thread exits closes the downstream channel automatically
- Connect to Unix pipes and OCaml's `Lwt_stream.map`

## Rust Application

```rust
fn pipeline_stage<T, U, F>(rx: mpsc::Receiver<T>, f: F) -> mpsc::Receiver<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + 'static,
{
    let (tx_out, rx_out) = mpsc::channel();
    thread::spawn(move || {
        for item in rx.iter() {       // stops when upstream closes
            tx_out.send(f(item)).unwrap();
        }
        // tx_out drops here → closes next stage
    });
    rx_out
}

fn run_pipeline(inputs: Vec<i32>) -> Vec<String> {
    let (tx_source, rx0) = mpsc::channel::<i32>();
    let rx1 = pipeline_stage(rx0, |x| x * 2);      // Stage 1: double
    let rx2 = pipeline_stage(rx1, |x| x + 1);       // Stage 2: add 1
    let rx3 = pipeline_stage(rx2, |x: i32| x.to_string()); // Stage 3: stringify

    let producer = thread::spawn(move || {
        for v in inputs { tx_source.send(v).unwrap(); }
    });

    let results: Vec<String> = rx3.iter().collect();
    producer.join().unwrap();
    results
}
```

Each stage thread owns its `Receiver<T>` (input) and `Sender<U>` (output). When the upstream channel closes (producer thread exits → `tx_source` drops), `rx0.iter()` terminates → stage 1's `tx_out` drops → `rx1.iter()` terminates → and so on through all stages.

This automatic teardown through channel closure is the Rust equivalent of Unix pipe EOF propagation. No sentinel values or explicit shutdown signals are needed.

`pipeline_stage` returns `Receiver<U>` — the caller composes stages by threading the returned receiver into the next stage call.

## OCaml Approach

```ocaml
open Lwt_stream

(* Lwt_stream.map is the direct equivalent *)
let run_pipeline inputs =
  let source = of_list inputs in
  let stage1 = map (fun x -> x * 2) source in
  let stage2 = map (fun x -> x + 1) stage1 in
  let stage3 = map (fun x -> string_of_int x) stage2 in
  to_list stage3  (* lazy evaluation starts here *)

(* Thread-based pipeline with Domainslib *)
let pipeline_stage rx f =
  let (tx_out, rx_out) = Domainslib.Chan.make_unbounded () in
  Domain.spawn (fun () ->
    let rec loop () = match Domainslib.Chan.recv rx with
      | None -> Domainslib.Chan.close tx_out
      | Some v -> Domainslib.Chan.send tx_out (Some (f v)); loop ()
    in loop ()
  ) |> ignore;
  rx_out
```

OCaml's `Lwt_stream.map` is lazy — transformation happens on demand as elements are consumed, not eagerly in parallel threads. For true parallel pipeline stages, `Domainslib.Chan` is needed.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Stage pattern | `pipeline_stage(rx, f) -> Receiver<U>` | `Lwt_stream.map f stream` (lazy) |
| Parallelism | True OS threads — stages run in parallel | `Lwt_stream` — cooperative scheduling |
| Shutdown | Channel close propagates automatically | `None` sentinel or stream close |
| Composition | Thread the returned `Receiver` | Wrap stream in next `map` |

Each stage in the Rust pipeline runs in its own OS thread — stages truly overlap in execution. Stage 2 can process an item while stage 1 is processing the next item and stage 3 is processing the previous item.

## Exercises

1. Add a filter stage: `filter_stage(rx, pred) -> Receiver<T>` that drops items where `pred(item)` is false.
2. Add a buffer stage: `buffer_stage(rx, n) -> Receiver<Vec<T>>` that batches `n` items before passing downstream.
3. Add error handling: change stage functions to return `Result<U, E>` and propagate errors through the pipeline.
4. Implement a fan-out stage: `broadcast_stage(rx, n) -> Vec<Receiver<T>>` that sends each item to `n` downstream stages.
5. Benchmark a 5-stage pipeline processing 100,000 items against a sequential fold over the same transformations.
