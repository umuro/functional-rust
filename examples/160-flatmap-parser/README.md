📖 **[View on hightechmind.io →](https://hightechmind.io/rust/160-flatmap-parser)**

---

# FlatMap Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Sometimes the choice of what to parse next depends on what was just parsed — a context-sensitive grammar. For example, a length-prefixed string `"3:abc"` requires first parsing the length `3`, then using that to parse exactly 3 more characters. `flat_map` (also called `bind` or `and_then`) enables this: it runs a parser, passes the result to a function that returns a new parser, and runs that parser on the remaining input. This is the monadic bind (`>>=`) for parsers, enabling context-sensitive parsing.

## Learning Outcomes

- Understand `flat_map` as monadic bind for parsers, enabling context-sensitive parsing
- Learn why `flat_map` is more powerful than `map` (it can choose the next parser dynamically)
- See the length-prefixed string pattern as a canonical `flat_map` example
- Understand the relationship between parser monads and other Rust monads (`Option`, `Result`)

## Rust Application

`flat_map<A, B>(parser: Parser<A>, f: impl Fn(A) -> Parser<B> + 'a) -> Parser<B>` runs the first parser, then calls `f` with the result to get a second parser, then runs the second parser. For length-prefixed parsing: `uint_parser.flat_map(|n| take_exactly(n))` — the second parser is created dynamically from the first's output. Without `flat_map`, this context-sensitive pattern cannot be expressed with `map` and `pair` alone.

## OCaml Approach

OCaml's `angstrom` provides `bind : 'a t -> ('a -> 'b t) -> 'b t` and the infix `>>=`:
```ocaml
let length_prefixed = uint_parser >>= fun n -> take n
```
Monadic `do`-notation via `let*` (OCaml 4.08+) makes context-sensitive parsers readable:
```ocaml
let length_prefixed =
  let* n = uint_parser in
  take n
```
This is cleaner than Rust's closure-based `flat_map` for complex sequences.

## Key Differences

1. **Notation**: OCaml's `let*` desugars to `>>=`, giving near-imperative parser code; Rust uses closure-based chaining, which becomes nested for long sequences.
2. **Power**: `flat_map` makes parsers a full monad — every combinator (`map`, `pair`, `opt`) is derivable from `flat_map` and `pure`; some libraries do exactly this.
3. **Performance**: `flat_map` prevents certain parser optimizations (streaming, streaming allocation) because the next step is unknown until the first is complete.
4. **Context sensitivity**: Both `angstrom` and Rust's `flat_map` handle context-sensitive grammars; PEG parsers without `flat_map` cannot.

## Exercises

1. Parse a Pascal-style string `'n:content'` where `n` is the length: `"5:hello"` → `"hello"`.
2. Implement a parser that reads a type tag `"i"` or `"s"` and then parses either an integer or a string accordingly.
3. Write `flat_map` in terms of `pure` and a hypothetical `join : Parser<Parser<T>> -> Parser<T>` to demonstrate the monadic structure.
