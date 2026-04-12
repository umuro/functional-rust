📖 **[View on hightechmind.io →](https://hightechmind.io/rust/909-iterator-skip-while)**

---

# 909-iterator-skip-while — Iterator skip_while
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Many data processing tasks need to skip a leading section before the relevant data begins: skip log header lines until the first data row, skip sorted leading zeros before meaningful values, strip leading whitespace from a string. `skip_while` is the complement of `take_while`: it discards elements from the front until the predicate first fails, then yields all remaining elements — including later ones that match the predicate. This "once it switches, it never switches back" behavior is the key distinction from `filter`. It models a state machine with two states: skipping and yielding.

## Learning Outcomes

- Use `.skip_while(pred)` to skip a leading prefix satisfying a condition
- Understand that elements matching the predicate after the skip point are still yielded
- Apply skip_while to strip leading zeros, whitespace, and sentinels
- Implement the recursive OCaml-style version for comparison
- Combine skip_while with take_while for two-sided trimming

## Rust Application

`skip_less_than` uses `slice.iter().copied().skip_while(|&x| x < threshold).collect()`. `lstrip_chars` uses `s.chars().skip_while(|c| c.is_whitespace()).collect::<String>()`. `strip_leading_zeros` uses `skip_while(|&x| x == 0)` — a trailing zero in the sequence is preserved because skipping already stopped at the first non-zero. `skip_while_recursive` uses slice patterns to match OCaml's list head/tail destructuring, recursing into the tail when the head matches.

## OCaml Approach

Standard OCaml lacks `List.skip_while`; manual recursion: `let rec skip_while p = function | [] -> [] | x :: rest -> if p x then skip_while p rest else x :: rest`. For `Seq`: `Seq.drop_while: ('a -> bool) -> 'a Seq.t -> 'a Seq.t` (since 4.14). String lstrip: `String.sub s (let i = ref 0 in while !i < String.length s && s.[!i] = ' ' do incr i done; !i) (String.length s - !i)`. OCaml's lack of built-in `skip_while` for lists is a notable gap.

## Key Differences

1. **Once-switch semantics**: Both Rust `.skip_while()` and OCaml `Seq.drop_while` switch from skipping to yielding exactly once — later-matching elements are included in the output.
2. **filter distinction**: `skip_while` stops skipping permanently; `filter` checks every element — they produce different results on non-monotone input.
3. **Standard library**: Rust has first-class `.skip_while()` on all iterators; OCaml requires `Seq.drop_while` (4.14+) or manual recursion.
4. **String application**: Rust's `chars().skip_while(whitespace).collect()` is idiomatic for lstrip; OCaml requires explicit index arithmetic.

## Exercises

1. Implement `skip_until<T: PartialEq + Clone>(data: &[T], sentinel: &T) -> Vec<T>` that skips elements until the sentinel is found, then yields all remaining (excluding the sentinel).
2. Write `trim_zeros(data: &[i32]) -> &[i32]` using `skip_while` for leading zeros and a manual slice end-trim for trailing zeros.
3. Implement a CSV reader that uses `skip_while` to skip comment lines (starting with `#`) before processing data rows.
