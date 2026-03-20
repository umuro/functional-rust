📖 **[View on hightechmind.io →](https://hightechmind.io/rust/151-parser-intro)**

---

# Introduction to Parser Combinators

## Problem Statement

Parsing transforms raw text into structured data. Hand-written parsers for each format are tedious, error-prone, and hard to compose. Parser combinators solve this by representing parsers as first-class values (functions) that can be combined using higher-order functions: sequence two parsers, try alternatives, repeat zero or more times. This functional approach, pioneered in Haskell by Parsec, produces parsers that closely mirror the grammar they parse, making them readable and maintainable.

## Learning Outcomes

- Understand the core type: a parser is a function from `&str` to `Result<(value, remaining), error>`
- Learn to create primitive parsers (`char_p`, `pure`, `fail`) that form the building blocks
- See how parsers compose to handle complex grammars
- Understand why parser combinators are preferred over hand-written recursive descent in functional code

## Rust Application

The core type is `type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>`. Every parser is a function: given a string slice, it either succeeds with a parsed value and the remaining input, or fails with an error message. `pure(value)` always succeeds without consuming input. `char_p(expected)` matches one character. These simple parsers become the alphabet from which all complex parsers are built via combinators.

## OCaml Approach

OCaml's parser combinator tradition predates Rust: the `angstrom` library and Menhir use similar ideas. The core type is typically `type 'a parser = string -> int -> ('a * int, string) result`. OCaml's lighter syntax for closures (`fun input -> ...`) and partial application make combinator definitions more concise. The `let ( >>= )` and `let ( <|> )` operators integrate naturally with OCaml's infix operator system.

## Key Differences

1. **Core type**: Both represent parsers as functions on string input; Rust uses `Box<dyn Fn>` due to lack of implicit heap allocation; OCaml uses plain functions or named types.
2. **Lifetime annotations**: Rust's `'a` lifetime ties the parser to the input string's lifetime; OCaml has no lifetime concept — the GC manages the input string.
3. **Operator syntax**: OCaml naturally defines custom infix operators (`>>=`, `<|>`) for sequencing and choice; Rust uses method chaining or named functions.
4. **Performance**: Rust's parser combinators are typically faster due to zero-allocation `&str` slices; OCaml's allocate more intermediate values.

## Exercises

1. Implement `always_succeed<T: Clone>(val: T) -> Parser<T>` and verify it consumes no input.
2. Write `one_of_chars(chars: Vec<char>) -> Parser<char>` that matches any character in the set.
3. Implement `expect_empty() -> Parser<()>` that succeeds only at the end of input.
