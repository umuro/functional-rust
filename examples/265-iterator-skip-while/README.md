📖 **[View on hightechmind.io →](https://hightechmind.io/rust/265-iterator-skip-while)**

---

# 265: Conditional Skipping with skip_while()

## Problem Statement

Data streams often begin with a preamble, header, or leading values that should be ignored before processing begins. Log files start with timestamps; CSV files may have metadata rows; sorted lists may have leading zeros. The `skip_while()` adapter solves this by discarding elements from the front of an iterator until the predicate first returns false, then yielding all remaining elements unconditionally — including any that would again match the original predicate.

## Learning Outcomes

- Understand that `skip_while(pred)` discards elements until predicate first fails, then yields everything after
- Distinguish `skip_while()` from `filter()`: later matching elements are still included
- Use `skip_while()` to skip headers, leading whitespace, or sentinel values
- Combine `skip_while()` with `take_while()` to extract a middle segment of a sequence

## Rust Application

`Iterator::skip_while(pred)` yields elements after the first `false`. Crucially, once the predicate fails, subsequent elements are never tested again — they pass through regardless:

```rust
// Skip leading zeros, include everything after first non-zero (even later zeros)
let result: Vec<i32> = [0, 0, 1, 0, 2, 0].iter().copied()
    .skip_while(|&x| x == 0)
    .collect();
// [1, 0, 2, 0] — later zeros are included

// Skip a CSV header row
let lines = ["name,age", "alice,30", "bob,25"];
let data: Vec<_> = lines.iter().skip_while(|l| l.contains(',') && l.contains("name")).collect();
```

## OCaml Approach

OCaml's `List.drop_while` (in `Base`/`Core`) or a recursive equivalent serves this role:

```ocaml
let rec skip_while pred = function
  | [] -> []
  | x :: xs -> if pred x then skip_while pred xs else x :: xs
```

`Seq.drop_while` provides the lazy equivalent for sequences, identical semantically to Rust's `skip_while`.

## Key Differences

1. **"Then all"**: Both languages' `skip_while` yield everything after the first false, including later matches — this is by design for ordered prefix stripping.
2. **Standard library**: Built into Rust's `Iterator`; OCaml's standard `List` module lacks it (third-party `Base` provides `List.drop_while`).
3. **Complementary to `take_while`**: Together they split a sequence: `take_while(p)` takes the prefix, `skip_while(p)` takes the suffix.
4. **Stateful termination**: Like `take_while`, it is stateful and not equivalent to a filter — order matters.

## Exercises

1. Strip leading whitespace from a character iterator using `skip_while(|c| c.is_whitespace())`.
2. Implement a function that extracts the body of a log file by skipping all lines that start with `#` (comment lines at the top).
3. Combine `skip_while()` and `take_while()` to extract only the elements between two sentinel values in a sequence.
