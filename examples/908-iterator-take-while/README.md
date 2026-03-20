📖 **[View on hightechmind.io →](https://hightechmind.io/rust/908-iterator-take-while)**

---

# 908-iterator-take-while — Iterator take_while
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Sometimes you want all elements up to the first failure, not just those satisfying a predicate anywhere in the sequence. `take_while` differs from `filter` in one critical way: it stops permanently at the first non-matching element, rather than skipping and continuing. This makes it the only safe option for bounded consumption of infinite iterators — `filter` on an infinite iterator that eventually runs out of matches would loop forever. It also models "leading prefix" queries: collect all sorted-prefix elements, consume a stream until a sentinel, read until end-of-section.

## Learning Outcomes

- Use `.take_while(pred)` to collect a leading prefix satisfying a condition
- Understand the "stops permanently" semantics vs `filter`'s "skip and continue"
- Apply `take_while` to infinite iterators safely
- Implement the functional/recursive version for OCaml comparison
- Recognize real use cases: reading until a sentinel, consuming sorted prefixes

## Rust Application

`take_while_less_than` uses `slice.iter().copied().take_while(|&x| x < threshold).collect()`. `leading_positives` uses `take_while(|&x| x > 0)`. `triangular_indices_below` applies `(1u64..).take_while(|&n| n*(n+1)/2 < limit)` on an infinite range — safe because `take_while` terminates. `take_while_rec` implements the OCaml recursive style using slice patterns: `[x, rest @ ..] => if pred(*x) { x :: take_while_rec(rest, pred) } else { [] }`.

## OCaml Approach

`List.filteri` is not the right analog — use the non-standard `take_while`. Standard OCaml lacks `List.take_while`; it requires manual recursion: `let rec take_while p = function | [] -> [] | x :: rest -> if p x then x :: take_while p rest else []`. For `Seq`: `Seq.take_while: ('a -> bool) -> 'a Seq.t -> 'a Seq.t` is available since 4.14 and has the same semantics as Rust's `.take_while()`.

## Key Differences

1. **Infinite safety**: Rust `.take_while()` terminates on infinite iterators; OCaml `Seq.take_while` also terminates; `List.take_while` requires finite input.
2. **Standard library**: Rust has `.take_while()` as a built-in method on all iterators; OCaml requires `Seq.take_while` (4.14+) or manual recursion for lists.
3. **Filter vs take_while**: Both languages have both operations; the distinction is "stops at first failure" vs "skips non-matching elements throughout."
4. **Slice patterns**: Rust's `[x, rest @ ..]` pattern in recursive implementations closely mirrors OCaml's `x :: rest` list destructuring.

## Exercises

1. Use `take_while` to implement `read_until_blank(lines: &[&str]) -> Vec<&str>` that returns lines up to but not including the first empty line.
2. Write `sorted_prefix(data: &[i32]) -> Vec<i32>` that returns the longest initial sorted (non-decreasing) subsequence.
3. Combine `take_while` and `skip_while` to implement `trim_both(data: &[i32], pred: impl Fn(&i32) -> bool) -> &[i32]` that removes matching elements from both ends.
