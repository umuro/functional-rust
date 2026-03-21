📖 **[View on hightechmind.io →](https://hightechmind.io/rust/905-iterator-peekable)**

---

# 905-iterator-peekable — Iterator Peekable
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Lexers and parsers routinely need to inspect the next token before deciding which production rule to apply, without consuming that token. Without peek, the only options are consuming and pushing back (requiring a separate buffer) or reading one token ahead manually. Rust's `.peekable()` adapter adds `peek()` — a reference to the next element without advancing the iterator. This is the minimal lookahead needed for LL(1) parsing, run-length encoding, and merge algorithms. OCaml's `Stream` module and the `Angstrom` parser combinator library serve similar roles.

## Learning Outcomes

- Use `.peekable()` to add one-element lookahead to any iterator
- Implement group-consecutive using `peek()` to detect group boundaries
- Build a simple tokenizer using peekable char iteration with digit accumulation
- Understand how `peek()` returns `Option<&Item>` without advancing
- Compare with OCaml's `Stream.peek` and recursive lookahead patterns

## Rust Application

`group_consecutive` uses `while iter.peek() == Some(&val)` to extend a group while the next element matches. The tokenizer loops: `while let Some(&ch) = chars.peek()` for the outer loop, then `while chars.peek().is_some_and(|c| c.is_ascii_digit())` to accumulate multi-digit numbers. The key pattern: `peek()` to inspect, `next()` to consume. The `Peekable` adapter buffers exactly one element — enough for LL(1) grammars.

## OCaml Approach

OCaml uses `Stream` (deprecated in 4.14) or manual lookahead. A common pattern: `let current = ref (Stream.next s)` with explicit rebinding after consuming. Parser combinators like `Angstrom` provide `peek_char`, `satisfy`, and `take_while` as building blocks. Manual tokenizers in OCaml typically use a position index with explicit bounds checking: `if !pos < len && s.[!pos] = c then ...`. OCaml's `Scanf` module handles many parsing cases without explicit peek.

## Key Differences

1. **One-element buffer**: Rust `Peekable` buffers exactly one element; OCaml `Stream` can buffer more; both handle LL(1) grammars.
2. **Peek returns reference**: Rust `peek()` returns `Option<&Item>` — a reference into the buffer; consuming requires a separate `next()` call; OCaml returns a copy.
3. **Composability**: Rust `Peekable<I>` is itself an iterator, composable with other adapters; OCaml `Stream` is a distinct type.
4. **Type safety**: Rust's typed `Peekable<Chars>` vs `Peekable<Tokens>` is explicit at the type level; OCaml streams are polymorphic.

## Exercises

1. Use `peekable` to implement `deduplicate<T: PartialEq + Clone>(iter: impl Iterator<Item=T>) -> Vec<T>` that removes consecutive duplicates.
2. Build a simple expression tokenizer that handles multi-character operators (`<=`, `>=`, `==`) using two-character lookahead.
3. Implement `merge_sorted<T: Ord + Clone>(a: impl Iterator<Item=T>, b: impl Iterator<Item=T>) -> Vec<T>` using peekable on both iterators.
