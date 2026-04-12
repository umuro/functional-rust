📖 **[View on hightechmind.io →](https://hightechmind.io/rust/912-iterator-inspect)**

---

# 912-iterator-inspect — Iterator Inspect
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Debugging a multi-step iterator pipeline is difficult: the lazy evaluation means no intermediate values exist until the consumer runs. Inserting `println!` calls requires breaking the pipeline into named variables. Rust's `.inspect(f)` solves this: it taps into the pipeline at any point, passing each element to a side-effect closure while passing it through unchanged. This is the "tap" operator from Haskell and RxJS, `(|>)` with side effects in F#, and the `;` expression evaluation in many languages. It is primarily a debugging tool but also useful for logging, metrics counting, and audit trails in production pipelines.

## Learning Outcomes

- Use `.inspect(f)` to observe elements at any point in an iterator pipeline
- Collect elements seen at each stage using captured Vec references
- Count elements passing through each stage using atomic counters
- Understand inspect as a "tap" that does not affect the pipeline's output
- Compare with OCaml's lack of a standard tap operator

## Rust Application

`inspect_pipeline` inserts two `.inspect()` calls into a range-filter-map pipeline: the first captures all input elements, the second captures only elements that passed the filter. The result is three outputs: all seen, evens seen, and final squared values. `count_stages` uses `AtomicUsize` counters shared across closures to count how many elements passed through each stage — useful for performance analysis. The key property: removing all `.inspect()` calls leaves the pipeline's output unchanged.

## OCaml Approach

OCaml lacks a standard `tap` operator for sequences. The closest idiom: insert `(fun x -> Printf.eprintf "debug: %d\n" x; x)` in a `List.map` chain, which both observes and passes through. For `Seq`: `Seq.map (fun x -> let () = f x in x) seq` is the manual tap. OCaml's side-effect-explicit style makes `inspect` less common — one would typically extract the debug information into separate operations rather than embedding side effects in a pipeline.

## Key Differences

1. **Explicit tap**: Rust `.inspect()` is a named, purpose-built debugging method; OCaml uses a manual identity-with-side-effect in `.map()`.
2. **Atomic counters**: Rust's thread-safe `AtomicUsize` enables inspect-based metrics in multi-threaded pipelines; OCaml uses `ref` for single-threaded counting.
3. **Production use**: `.inspect()` with logging is acceptable in Rust production code (lightweight); OCaml's equivalent requires careful handling to avoid mutation in otherwise-pure pipelines.
4. **No-op removal**: Removing `.inspect()` preserves the pipeline's type and output — it is truly transparent; OCaml's manual tap changes the `.map()` return type if not careful.

## Exercises

1. Use `.inspect()` to implement a pipeline profiler that measures the fraction of elements passing each filter stage.
2. Add logging to a multi-stage data transformation using `.inspect()` to log a sample (every 100th element) without logging everything.
3. Write `inspect_changes<T: PartialEq + Clone>(iter: impl Iterator<Item=T>) -> impl Iterator<Item=T>` that logs when the value changes between consecutive elements.
