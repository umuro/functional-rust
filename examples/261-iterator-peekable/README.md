📖 **[View on hightechmind.io →](https://hightechmind.io/rust/261-iterator-peekable)**

---

# 261: Lookahead with Peekable

## Problem Statement

Many parsing and grouping algorithms need to examine the next element before deciding whether to consume it. A lexer must decide if the current `<` starts `<`, `<=`, or `<<` by looking ahead. A run-length encoder must check if the next element continues the current run. Without lookahead, you must buffer elements manually or restructure the algorithm. `Peekable` wraps any iterator to add a one-element lookahead without consuming the peeked element.

## Learning Outcomes

- Understand how `peek()` inspects the next element without advancing the iterator
- Use `Peekable` to implement run-length grouping and lexer-style tokenization
- Recognize that `peek()` returns `Option<&Item>` — a reference to the next element
- Combine `peek()` with `while let` to consume elements conditionally

## Rust Application

`Iterator::peekable()` wraps an iterator in `Peekable<I>`. The key method is `peek() -> Option<&I::Item>` which buffers the next element internally and returns a reference to it without advancing:

```rust
let data = [1i32, 1, 2, 3, 3];
let mut iter = data.iter().peekable();
let mut groups: Vec<Vec<i32>> = Vec::new();
while let Some(&val) = iter.peek() {
    let mut group = Vec::new();
    while iter.peek() == Some(&val) {
        group.push(*iter.next().unwrap());
    }
    groups.push(group);
}
// [[1,1], [2], [3,3]]
```

## OCaml Approach

OCaml's standard library does not provide a built-in peekable abstraction. The idiomatic approach is to use a `ref` holding an optional buffered element, or to restructure the algorithm to pass the "next" element as an argument to recursive calls:

```ocaml
(* Manual lookahead with a buffer ref *)
let peekable seq =
  let buf = ref None in
  let peek () = match !buf with
    | Some v -> Some v
    | None -> let v = Seq.uncons seq in buf := Option.map fst v; Option.map fst v
  in ...
```

## Key Differences

1. **Built-in support**: Rust provides `Peekable<I>` as a standard adapter; OCaml requires manual buffering.
2. **Reference semantics**: `peek()` returns `Option<&Item>` — a reference — so the item remains available for `next()`; no copying occurs.
3. **One-element buffer**: `Peekable` buffers exactly one element; for deeper lookahead you need a different approach.
4. **Parser combinators**: Libraries like `nom` and `pest` build on this pattern for full recursive-descent parsing.

## Exercises

1. Use `Peekable` to implement a simple tokenizer that merges consecutive digit characters into a single number token.
2. Build a `group_consecutive` function that groups adjacent equal elements without allocating a combined collection first.
3. Use `peek()` to implement a `merge_sorted` function that merges two sorted iterators into one sorted output without a temporary buffer.
