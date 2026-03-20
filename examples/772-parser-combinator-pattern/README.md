📖 **[View on hightechmind.io →](https://hightechmind.io/rust/772-parser-combinator-pattern)**

---

# 772-parser-combinator-pattern — Parser Combinator Pattern
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Parser combinators compose small parsers into larger ones using functions: `and_then`, `or`, `many`, `map`. Each primitive parser handles one token; combinators wire them together to match entire grammars without a separate grammar specification or code generator. Introduced in Haskell with Parsec (1995), parser combinators are now central to Rust's `nom` crate (the most downloaded parser library), Haskell's `megaparsec`, and OCaml's `angstrom`. They enable "parsing as programming" — no DSL required.

## Learning Outcomes

- Implement primitive parsers: `char_p`, `satisfy`, `string_p`, `digit`, `alpha`
- Build combinator functions: `and_then`, `or_else`, `many`, `many1`, `map`
- Parse numbers, identifiers, and quoted strings using only combinators
- Understand `ParseResult<'a, T> = Option<(T, &'a str)>` as the parser return type
- See how complex parsers like `integer` and `identifier` emerge from primitive combinators

## Rust Application

`ParseResult<'a, T> = Option<(T, &'a str)>` represents success (value + remaining input) or failure (`None`). `char_p(c)` matches one character. `satisfy(pred)` matches any character satisfying a predicate. `and_then(p, f)` sequences two parsers. `or_else(p, q)` tries `p`, then `q` on failure. `many(p)` applies a parser zero or more times. `integer` combines `satisfy(is_digit)` + `many(...)` + conversion. Tests parse integers, identifiers, and simple expressions.

## OCaml Approach

OCaml's `angstrom` is a production-grade combinator library. `char 'a'` matches a character. `take_while is_alpha` matches sequences. `lift2 f p q` applies a binary function to two parser results. `choice [p; q; r]` tries alternatives. OCaml's `>>=` (bind) and `>>|` (map) operators make combinator code concise. `Angstrom.parse_string` drives parsing. The `sedlex` library handles Unicode lexing.

## Key Differences

1. **Parser type**: Rust uses `fn(&str) -> Option<(T, &str)>` (simple function); Angstrom uses a continuation monad for backtracking and error recovery.
2. **Backtracking**: Rust's function-based approach backtracks by just trying the next parser; Angstrom has explicit `commit` to cut backtracking for efficiency.
3. **Error messages**: Angstrom tracks position and expected tokens for error messages; this example's `None` provides no location information.
4. **Lifetimes**: Rust's combinator closures capture lifetimes; OCaml's closures automatically close over the environment without explicit lifetime annotation.

## Exercises

1. Implement `separated_by(parser, sep)` that parses `p (sep p)*` — useful for comma-separated lists.
2. Build a JSON parser using combinators: `json_value` = `json_null | json_bool | json_number | json_string | json_array | json_object`.
3. Add error position tracking by changing the parser type to `Result<(T, &str), (usize, &str)>` where the error includes the byte position of the failure.
