📖 **[View on hightechmind.io →](https://hightechmind.io/rust/904-iterator-scan)**

---

# 904-iterator-scan — Iterator Scan

## Problem Statement

Fold produces one final value. Scan produces the whole sequence of intermediate values — one per input element. Running sums, cumulative maxima, and prefix products are all scan applications. The scan operation also supports early termination: returning `None` from the closure stops the iterator, enabling a "scan until condition" behavior that fold cannot express lazily. Haskell has `scanl`; OCaml has `Seq.scan` (since 4.14); Python has `itertools.accumulate`. Rust's `Iterator::scan` takes a mutable state reference, giving fine-grained control over both the emitted value and the accumulator.

## Learning Outcomes

- Use `.scan(init, |state, item| ...)` to produce running accumulations
- Implement running sum, product, and maximum using scan
- Use `None` return from the scan closure to stop early
- Understand the difference between scan's `&mut state` (what's tracked) and `Some(value)` (what's emitted)
- Compare with OCaml's `Seq.scan` and Python's `itertools.accumulate`

## Rust Application

`running_sum` uses `scan(0i64, |state, &x| { *state += x; Some(*state) })`. `running_product` replaces `+=` with `*=`. `running_sum_until` returns `None` when state exceeds `limit`, stopping the iterator early — no equivalent in fold. `running_max` uses `|state, &x| { *state = (*state).max(x); Some(*state) }`. The state (`*state`) is the running accumulator; the returned `Some(value)` can be different from the state — enabling running averages where state holds `(sum, count)` and emitted value is `sum/count`.

## OCaml Approach

OCaml `Seq.scan f init xs` is available since 4.14. For eager scan: `let scan_left f init xs = let r = ref [init] in let _ = List.fold_left (fun acc x -> let next = f acc x in r := next :: !r; next) init xs in List.rev !r`. The custom `scan_left` in the example preserves the initial value in the output (like Haskell's `scanl`), while Rust's `scan` does not include the initial state in the output — a subtle but important difference.

## Key Differences

1. **Initial state in output**: Haskell `scanl` and OCaml `scan_left` include the initial accumulator as the first output element; Rust `scan` does not — it starts emitting from the first transformed value.
2. **Early termination**: Rust scan stops when the closure returns `None`; OCaml `Seq.scan` requires `Seq.take_while` for early stopping.
3. **State vs emitted**: Rust scan can emit a value different from the updated state (state is `&mut S`, emitted is `Some(V)`); OCaml scan emits the updated state directly.
4. **Mutability**: Rust uses `&mut state` explicitly; OCaml threads state as an immutable function argument.

## Exercises

1. Implement `running_average` using scan with state `(sum, count)` and emitting `sum / count as f64`.
2. Write `scan_while_positive(nums: &[i64]) -> Vec<i64>` that computes a running product but stops when it first becomes negative.
3. Use scan to implement a sliding minimum where the state tracks the minimum seen in the last k elements.
