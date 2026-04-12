📖 **[View on hightechmind.io →](https://hightechmind.io/rust/158-sequence-parser)**

---

# Sequence Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Most grammar rules require multiple things in order: a key-value pair is `identifier "=" value`, a function call is `identifier "(" args ")"`. Sequence combinators run two or more parsers in order, combining their results. `pair` keeps both results. `preceded` discards the first (e.g., parse `"("` then `value` — keep `value`). `terminated` discards the second. `delimited` discards both ends and keeps the middle — the most common case for parenthesized expressions.

## Learning Outcomes

- Implement `pair`, `preceded`, `terminated`, and `delimited` sequence combinators
- Understand how sequence combinators propagate failures: if any parser fails, the whole sequence fails
- Learn the "discard wrapper, keep content" idiom (delimited, preceded, terminated)
- Practice building compound parsers: parenthesized expressions, key-value pairs

## Rust Application

`pair<A, B>(pa: Parser<A>, pb: Parser<B>) -> Parser<(A, B)>` runs `pa` on the input, passes the remaining input to `pb`, and returns both results as a tuple. `preceded(skip, keep)` runs both but discards `skip`'s result. `delimited(open, content, close)` is `preceded(open, terminated(content, close))`. Error from any step short-circuits — if `open` parses but `content` fails, the whole `delimited` fails with `content`'s error.

## OCaml Approach

Angstrom provides `both : 'a t -> 'b t -> ('a * 'b) t`, `preceded`, `terminated`, `delimited` directly. The infix `*>` (sequence, keep right) and `<*` (sequence, keep left) operators give the most concise syntax:
```ocaml
let delimited open_ p close = open_ *> p <* close
```
OCaml's operator syntax makes sequence parsing arguably the most readable of all combinator styles.

## Key Differences

1. **Operator syntax**: OCaml's `*>` and `<*` operators make sequence parsing compact and directional; Rust uses named combinators (`preceded`, `terminated`) which are more explicit.
2. **Error propagation**: Both short-circuit on failure and return the failing parser's error; the sequence is always left-to-right.
3. **Tuple results**: Rust's `pair` returns `(A, B)`; OCaml's `both` returns `(a * b)` — structurally the same.
4. **Arity**: Both languages' sequence combinators are binary; for more than two, nesting or `tuple3`/`tuple4` variants are used.

## Exercises

1. Parse a key-value pair `"name: Alice"` using `pair(identifier_parser(), preceded(tag(": "), identifier_parser()))`.
2. Write a parenthesized expression parser using `delimited(char_parser('('), expr_parser(), char_parser(')'))`.
3. Implement `sequence_all<T>(parsers: Vec<Parser<T>>) -> Parser<Vec<T>>` that runs all parsers in order.
