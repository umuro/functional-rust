📖 **[View on hightechmind.io →](https://hightechmind.io/rust/260-iterator-scan)**

---

# 260: Stateful Accumulation with scan()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Computing running totals, prefix sums, or any sequence where each value depends on all previous values is a common need in financial systems, signal processing, and data streaming. A plain `fold()` produces only the final accumulated result. The `scan()` adapter solves this by emitting each intermediate accumulated state as an element — turning a `fold` into a stream of partial results.

## Learning Outcomes

- Understand `scan()` as `fold()` that yields each intermediate state rather than only the final value
- Use mutable accumulator state inside a `scan()` closure
- Return `None` from a `scan()` closure to terminate the sequence early
- Build running totals, prefix products, and other prefix-sum structures

## Rust Application

`scan(initial, f)` takes an initial accumulator value and a closure `|state: &mut S, item: T| -> Option<B>`. It mutates `state` in place and yields `Some(value)` for each step. Returning `None` terminates the iterator:

```rust
// Running sum (prefix sums)
let result: Vec<i32> = [1, 2, 3, 4, 5]
    .iter()
    .scan(0i32, |sum, &x| { *sum += x; Some(*sum) })
    .collect();
// [1, 3, 6, 10, 15]

// Early termination: stop when sum exceeds 6
let capped: Vec<i32> = [1, 2, 3, 4, 5]
    .iter()
    .scan(0i32, |s, &x| { *s += x; if *s > 6 { None } else { Some(*s) } })
    .collect();
// [1, 3, 6]
```

## OCaml Approach

OCaml's `List.scan_left` (introduced in recent versions) directly mirrors this pattern, or one can build it with `List.fold_left` accumulating into a reversed list of intermediates:

```ocaml
(* Manual scan: fold collecting intermediates *)
let scan_left f init xs =
  List.rev (snd (List.fold_left (fun (acc, lst) x ->
    let acc' = f acc x in (acc', acc' :: lst)
  ) (init, []) xs))
```

OCaml's `Seq` module makes this natural for lazy streams.

## Key Differences

1. **Output**: `scan()` emits every intermediate state; OCaml's equivalent must explicitly accumulate intermediates into a list.
2. **Early termination**: Rust's `scan()` can return `None` to stop the sequence; a fold-based OCaml approach requires extra logic.
3. **Mutable reference**: Rust's closure receives `&mut state`, making mutation explicit; OCaml uses functional update via return value.
4. **Real-world use**: Used in NumPy as `np.cumsum()`, in databases as window functions, and in streaming analytics as running aggregates.

## Exercises

1. Compute prefix products of a slice using `scan()` — the product of all elements up to and including each position.
2. Use `scan()` to implement a running maximum: each output is the maximum seen so far.
3. Build a balance tracker: given a sequence of deposit/withdrawal amounts, use `scan()` to emit the running account balance and terminate when it goes negative.
