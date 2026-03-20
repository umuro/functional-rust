[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 094 — Peekable Iterator

## Problem Statement

Use `.peekable()` to look ahead at the next iterator element without consuming it. Implement `dedup` — removing consecutive duplicates — as a demonstration: consume the current value, then skip ahead while the peek matches. Compare with OCaml's manual `peekable` wrapper built on `Seq`.

## Learning Outcomes

- Convert any iterator to a `Peekable<I>` with `.peekable()`
- Use `iter.peek()` to inspect the next item as `Option<&Item>` without advancing
- Combine `next()` and `peek()` in a `while let` loop for look-ahead parsing
- Understand the double-reference in `peek()` — `Some(&&val)` for `&[i32]`
- Map Rust's built-in `Peekable` to OCaml's manually implemented `peekable` wrapper
- Recognise peekable iterators as essential for tokenizers and parsers

## Rust Application

`v.iter().peekable()` wraps the iterator in a `Peekable` adapter. `peek()` returns `Option<&&i32>` — a reference to the next item's reference. The `dedup` loop advances with `next()` to get the current value, then uses a nested `while iter.peek() == Some(&&val)` loop to skip duplicates. The outer `while let Some(&val)` destructures through the reference layer. The implementation is O(n) with no allocation beyond the result vector.

## OCaml Approach

OCaml's `Seq` has no built-in `peek`. The `peekable` record holds `peeked: 'a option` and `seq: 'a Seq.t ref`. `peek` checks the peeked slot, or forces the next thunk and caches it. `next` drains the peeked slot or forces the seq. This is a complete, correct implementation — just more code than Rust's built-in.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Built-in | `Iterator::peekable()` | Manual `peekable` struct |
| Peek type | `Option<&Item>` (reference) | `'a option` (value) |
| Double ref | `Some(&&val)` for `&[T]` iterators | `Some x` directly |
| Mutable | Internally mutable in adapter | Mutable fields in record |
| Parser use | Common in tokenisers | Same manual wrapper |
| Standard | Yes | No |

Peekable iterators are the foundation of hand-written parsers and tokenisers. By looking ahead without consuming, you can make branching decisions based on context — the backbone of recursive descent parsing.

## Exercises

1. Implement a simple tokeniser that groups consecutive digit characters into `Token::Number(u64)` and other characters into `Token::Other(char)` using a peekable iterator.
2. Write `group_by_peekable<T: PartialEq>(v: &[T]) -> Vec<Vec<T>>` that groups consecutive equal elements using `peekable`.
3. Implement `parse_csv_field(iter: &mut Peekable<impl Iterator<Item=char>>) -> String` that reads until a comma or end of input.
4. Use `peek` to implement a `take_while_peek` that advances as long as the peeked element satisfies a predicate, stopping before consuming the first non-matching element.
5. In OCaml, add a `peek_nth` function to the `peekable` wrapper that allows looking `n` elements ahead by caching a `VecDeque`-style buffer.
