📖 **[View on hightechmind.io →](https://hightechmind.io/rust/159-map-parser)**

---

# Map Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Parsers produce raw strings and characters, but applications need structured data — integers, enums, structs. The `map` combinator transforms a parser's output without changing what it consumes. It is the `Functor` operation for parsers: if you have a `Parser<A>` and a function `A -> B`, `map` produces a `Parser<B>`. This functional transformation step keeps parsing (what to consume) separate from interpretation (what it means), a core principle of combinator-based parsing.

## Learning Outcomes

- Understand `map` as the fundamental output-transformation combinator (Functor)
- Learn how `map` separates parsing from interpretation
- See how character/string parsers are lifted into typed domain objects via `map`
- Practice chaining `map` with other combinators to build typed parsers

## Rust Application

`map<A, B>(parser: Parser<A>, f: impl Fn(A) -> B + 'a) -> Parser<B>` runs the inner parser and applies `f` to the result. `many1(is_digit()).map(|chars| chars.iter().collect::<String>().parse::<u32>().unwrap())` converts a `Vec<char>` of digits to a `u32`. Map is lazy — `f` is only called when the parser succeeds. Composing map transforms: `char_parser('T').map(|_| true)` parses `'T'` and produces `true`.

## OCaml Approach

OCaml's angstrom provides `map : ('a -> 'b) -> 'a t -> 'b t` and the infix `>>|`:
```ocaml
let uint_parser = many1 digit >>| (fun cs -> int_of_string (String.of_list cs))
```
OCaml's `|>` and `>>|` operators compose naturally. The `lift` family (`lift2`, `lift3`) applies functions of multiple arguments to multiple parsers simultaneously, avoiding explicit `pair` + `map` combinations.

## Key Differences

1. **Infix operator**: OCaml's `>>|` (or `<$>`) applies `map` with operator precedence; Rust uses method chaining or named `map(p, f)` functions.
2. **Lift**: OCaml's `lift2 f p q` = `pair(p, q).map(|(a, b)| f(a, b))` in Rust; OCaml's syntax is more concise for multi-parser transformations.
3. **Error preservation**: Both pass through the inner parser's error on failure — `map`'s function `f` is never called on failure.
4. **Composability**: `map` is the foundational combinator — all structured parsers are built from primitives via `map` and sequencing.

## Exercises

1. Write `digit_value() -> Parser<u32>` that parses a single ASCII digit and returns its numeric value using `map`.
2. Build a `color_parser() -> Parser<Color>` where `Color` is an enum `{Red, Green, Blue}` and inputs are `"red"`, `"green"`, `"blue"`.
3. Implement `map_err<T>(p: Parser<T>, f: impl Fn(String) -> String) -> Parser<T>` that transforms error messages.
