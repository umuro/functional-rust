📖 **[View on hightechmind.io →](https://hightechmind.io/rust/462-pipeline-concurrency)**

---

# 462: Pipeline Concurrency
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Data processing often has sequential stages: read → parse → validate → transform → write. Running stages sequentially wastes time — the parser waits idle while the reader fetches the next batch. Pipelining runs stages concurrently: stage 1 processes item 1 while stage 2 processes item 0. With N stages each taking time T, throughput improves from T per item to T per item (after startup) with N stages running simultaneously. This is the assembly line model applied to software.

Pipeline concurrency appears in video encoding (decode→filter→encode stages), ETL pipelines, network packet processing, compiler stages (lex→parse→typecheck→codegen), and build systems.

## Learning Outcomes

- Understand how channel-connected stages enable pipeline concurrency
- Learn how each stage runs in its own thread consuming from one channel and producing to the next
- See how `Sender<O>` + `Receiver<I>` connect stages via channels
- Understand pipeline throughput vs. latency: throughput improves, per-item latency increases
- Learn the `Pipeline` builder pattern for constructing multi-stage pipelines

## Rust Application

In `src/lib.rs`, `Pipeline::new` creates input/output channels for the first stage. Each stage spawns a thread that reads from its input channel, applies the processing function, and sends to the output channel. `add_stage` chains additional stages. When the input sender is dropped, the shutdown propagates through all stages automatically as each channel closes. The `JoinHandle` vector enables clean shutdown.

## OCaml Approach

OCaml implements pipelines with sequences of `Thread.create` connected by channels or queues. `Lwt` and `Async` have stream combinators (`Lwt_stream.map`, `Pipe.map`) for async pipeline stages. OCaml 5.x's `Domainslib` enables parallel pipeline stages across domains. The functional style naturally expresses pipelines as function composition: `data |> stage1 |> stage2 |> stage3`.

## Key Differences

1. **Channel chaining**: Rust pipelines use explicit channel pairs between stages; OCaml's `Lwt_stream.map` chains lazily.
2. **Backpressure**: Rust's bounded channels provide backpressure between stages; OCaml's async streams propagate backpressure through demand.
3. **Error handling**: Rust pipeline errors propagate via `Result` in channel messages; OCaml's `Lwt` uses promise rejection.
4. **Stage composition**: Rust's `Pipeline::add_stage` builder enables composing stages; OCaml's function composition `|>` is more natural for pure transformations.

## Exercises

1. **Three-stage pipeline**: Build a text processing pipeline: stage 1 splits text into words, stage 2 filters words longer than 5 characters, stage 3 converts to uppercase. Use `mpsc::channel` between stages. Verify with `"the quick brown fox jumps over the lazy dog"`.
2. **Parallel stage**: Implement a stage that fans out to N worker threads (like fan-out/fan-in) then collects results. This enables a single slow stage to have parallelism without affecting the pipeline structure.
3. **Pipeline metrics**: Add per-stage counters tracking items processed and processing time. Expose a `metrics() -> Vec<StageMetrics>` method on the pipeline to identify bottleneck stages.
