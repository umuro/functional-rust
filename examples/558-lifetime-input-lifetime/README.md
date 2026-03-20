📖 **[View on hightechmind.io →](https://hightechmind.io/rust/558-lifetime-input-lifetime)**

---

# Input Lifetime Patterns
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Input lifetimes determine how long the references passed to a function must remain valid. A function with one `&str` input has a simple input lifetime; a function with multiple `&str` inputs has multiple potentially independent input lifetimes. Getting input lifetimes right determines how long callers must keep their data alive. Over-constraining input lifetimes (requiring longer-lived data than necessary) makes APIs rigid; under-constraining them is a compile error. This example surveys the common input lifetime patterns systematically.

## Learning Outcomes

- How single-input functions get one input lifetime (often elided)
- How `first<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str` expresses independent input lifetimes
- How `Processor<'input>` ties the struct to an input buffer's lifetime at construction
- Why input lifetimes on struct constructors determine how long the struct can live
- How the relationship between input and output lifetimes determines API flexibility

## Rust Application

`echo<'a>(s: &'a str) -> &'a str` — one input, one output, same lifetime (explicit for clarity). `first<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str` — two independent inputs, output from first only. `Processor<'input>` stores `data: &'input str` — its lifetime is bound to the input given at construction. `process(&self) -> &'input str` returns data with the stored `'input` lifetime — not `self`'s lifetime. This demonstrates how input lifetimes propagate into struct lifetimes and back out through methods.

Key patterns:
- `<'a, 'b>` — two independent input lifetimes
- Struct lifetime bound to constructor input: `fn new(data: &'input str) -> Self`
- Method returning stored-input lifetime vs self lifetime

## OCaml Approach

OCaml input "lifetimes" are managed by the GC — the compiler does not track how long input data must remain valid. Functions that store references to inputs simply keep them alive through GC references:

```ocaml
type 'a processor = { data: 'a }
let process p = String.trim p.data  (* data kept alive by GC *)
```

## Key Differences

1. **Input validity duration**: Rust input lifetimes enforce at compile time how long the caller must keep inputs alive; OCaml's GC extends lifetime automatically as needed.
2. **Independent inputs**: Rust's `<'a, 'b>` allows two inputs to have different scopes — one can go out of scope before the other; OCaml treats all GC-managed inputs uniformly.
3. **Struct-bound lifetimes**: Rust `Processor<'input>` cannot outlive its input data; OCaml's `processor` keeps its data alive as long as the processor exists.
4. **API design**: Rust API designers choose whether inputs share a lifetime or have independent lifetimes — a design decision affecting caller ergonomics; OCaml has no such choice.

## Exercises

1. **Three-input function**: Write `fn join3<'a>(sep: &str, a: &'a str, b: &'a str) -> String` — note that `sep` does not appear in the output, so it does not need a named lifetime.
2. **Input consumed by struct**: Implement `struct Parser<'src> { input: &'src str, pos: usize }` where the `'src` lifetime is set at construction and `next<'parser>(&'parser mut self) -> Option<&'src str>` returns slices of the original input.
3. **Optional input**: Write `fn with_prefix<'a>(s: &'a str, prefix: Option<&str>) -> String` that prepends `prefix` to `s` if present — observe that `prefix` needs no named lifetime since it does not appear in the output.
