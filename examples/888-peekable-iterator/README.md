📖 **[View on hightechmind.io →](https://hightechmind.io/rust/888-peekable-iterator)**

---

# 888-peekable-iterator — Peekable Iterator

## Problem Statement

Many parsing and streaming algorithms need to look ahead at the next element before deciding whether to consume it. A lexer scanning multi-character numbers must peek at the next character without consuming it. A run-length encoder must check if the next element matches the current group. Implementing lookahead without peekable iterators requires awkward "push-back" buffers or read-one-ahead state variables. Rust's `.peekable()` adapter adds a `peek()` method that returns a reference to the next element without advancing the iterator. OCaml handles this with explicit option state or stream parsers from the `Stream` module.

## Learning Outcomes

- Use `.peekable()` to add lookahead to any Rust iterator
- Use `peek()` inside a while loop to implement conditional consumption
- Build a simple tokenizer for multi-character numbers using peekable char iterators
- Implement run-length grouping using peek to detect group boundaries
- Compare with OCaml's stream-based parsing approach

## Rust Application

`sum_while_positive` uses `iter.peek().is_some_and(|&&v| v > 0)` to sum elements until a non-positive is encountered — peek decides, `next()` consumes. `group_consecutive` uses `iter.peek().is_some_and(|next| *next == item)` to extend a group while the next element matches. The tokenizer scans digits by peeking: `while chars.peek().is_some_and(|c| c.is_ascii_digit()) { num_str.push(chars.next().unwrap()); }` accumulates the full number before emitting a `Token::Num`.

## OCaml Approach

OCaml's `Stream` module provides `Stream.peek`, `Stream.next`, and `Stream.junk` for similar lookahead. More commonly, OCaml parsers use the `Scanf` module or write recursive descent parsers with explicit state. The `Angstrom` library provides monadic combinators. For ad-hoc tokenization, OCaml often uses a `Buffer.t` for accumulation and a `ref` for the current character position — more explicit state management than Rust's peekable iterator.

## Key Differences

1. **Borrow semantics**: Rust `peek()` returns `Option<&Self::Item>` — a reference into the iterator; consuming requires a separate `next()` call. OCaml `Stream.peek` returns a copy.
2. **Mutable iterator state**: Rust's peekable stores one buffered element; OCaml's Stream maintains its own internal buffer.
3. **Combinator style**: OCaml parsers often use applicative/monadic combinators (`let*`, `>>=`); Rust peekable enables hand-written recursive descent.
4. **Integration**: Rust peekable works with any iterator (char iterators, token iterators, etc.); OCaml Stream requires wrapping the source.

## Exercises

1. Implement a simple arithmetic expression tokenizer for `+`, `-`, `*`, `/`, integers, and parentheses using `.peekable()`.
2. Write `merge_sorted_peekable` that merges two sorted peekable iterators into a single sorted sequence without collecting.
3. Implement `parse_csv_field` that uses a peekable char iterator to correctly handle quoted fields containing commas.
