📖 **[View on hightechmind.io →](https://hightechmind.io/rust/155-many-parser)**

---

# Many Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Most grammars involve repetition: a sequence of digits forms a number, a sequence of letters forms an identifier, a sequence of statements forms a block. `many0` (zero or more) and `many1` (one or more) generalize repetition for any parser. These combinators run a parser repeatedly until it fails, collecting all results into a `Vec<T>`. `many0` always succeeds (empty input is zero repetitions); `many1` fails if the first application fails.

## Learning Outcomes

- Understand `many0` and `many1` as the standard repetition combinators
- Learn why `many0` cannot fail (the empty case is valid)
- See how many combines with character parsers to build word and number parsers
- Understand the potential infinite loop: using `many0` with a parser that always succeeds

## Rust Application

`many0` runs the inner parser in a loop. Each iteration: try the parser on the current input. If it succeeds, push the result and update the remaining input. If it fails, stop and return the accumulated results. The key invariant: the inner parser must consume at least one character on success, or `many0` loops forever. `many1` calls `many0` after verifying at least one success — this ensures the returned `Vec` is non-empty.

## OCaml Approach

OCaml's angstrom provides `many : 'a t -> 'a list t` and `many1 : 'a t -> 'a list t`. The implementation uses `fix` (the fixed-point combinator) for laziness:
```ocaml
let many p = fix (fun m -> (lift2 (fun x xs -> x :: xs) p m) <|> return [])
```
This recursive definition handles the termination naturally in OCaml's lazy evaluation model. The list-based return type in OCaml means repeated prepending — less efficient than Rust's `Vec::push` for long sequences.

## Key Differences

1. **Return type**: Rust collects into `Vec<T>` (O(1) amortized push); OCaml traditionally uses lists (O(n) for reversing if using `cons`).
2. **Termination**: Both must ensure the inner parser consumes input to avoid infinite loops; this is a convention, not a type-system guarantee.
3. **Lazy vs. eager**: OCaml's `fix`-based `many` terminates naturally in a lazy context; Rust's explicit loop is strictly eager.
4. **Error handling**: `many0` swallows failures silently (treating them as termination); `many1` propagates the first failure.

## Exercises

1. Implement `many_sep<A, B>(item: Parser<A>, sep: Parser<B>) -> Parser<Vec<A>>` that parses `a, a, a` where `,` is the separator.
2. Write `at_least<T>(n: usize, p: Parser<T>) -> Parser<Vec<T>>` that requires at least `n` successful applications.
3. Implement `at_most<T>(n: usize, p: Parser<T>) -> Parser<Vec<T>>` that applies the parser at most `n` times.
