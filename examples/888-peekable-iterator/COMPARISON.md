# OCaml vs Rust: Peekable Iterator

## Side-by-Side Code

### OCaml — manual peekable buffer

```ocaml
type 'a peekable = {
  mutable peeked : 'a option;
  next_fn : unit -> 'a option;
}

let peek p =
  match p.peeked with
  | Some _ as v -> v
  | None ->
    let v = p.next_fn () in
    p.peeked <- v; v

let next p =
  match p.peeked with
  | Some _ as v -> p.peeked <- None; v
  | None -> p.next_fn ()
```

### Rust (idiomatic) — `.peekable()` from std

```rust
pub fn sum_while_positive(data: &[i32]) -> i32 {
    let mut iter = data.iter().peekable();
    let mut sum = 0;
    while iter.peek().map_or(false, |&&v| v > 0) {
        sum += iter.next().unwrap();
    }
    sum
}
```

### Rust (functional/recursive) — `next_if` combinator

```rust
// next_if: consume and return next element only if predicate holds.
// Returns None and leaves iterator untouched if predicate fails.
while let Some(d) = chars.next_if(|c| c.is_ascii_digit()) {
    num_str.push(d);
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Peekable type | `'a peekable` (custom record) | `Peekable<I>` (std wrapper) |
| Peek result | `'a option` | `Option<&I::Item>` (reference) |
| Conditional advance | manual: check peeked, call next | `iter.next_if(pred)` |
| Iterator item | `'a` | `I::Item` |

## Key Insights

1. **Built-in vs manual:** OCaml has no standard peekable — you must build a mutable record with a `peeked` ref field yourself. Rust ships `Peekable<I>` in `std::iter`, wrapping any iterator with a one-slot buffer at zero cost.

2. **Reference semantics on peek:** Rust's `.peek()` returns `Option<&Item>` — a *reference* to the buffered item, not ownership. This prevents double-consume bugs at the type level; you can only move the value by calling `.next()`.

3. **`next_if` is the power move:** `iter.next_if(|v| pred(v))` atomically peeks and conditionally advances — perfect for tokenizer digit runs. There is no equivalent in OCaml's standard library; you'd call `peek` then `next` manually, risking logic errors.

4. **No mutation of outer state:** OCaml's manual peekable requires a `mutable peeked` field and a captured `ref` to the list tail. Rust's `Peekable<I>` owns its internal buffer with no external `ref` cells — mutation is localized inside the iterator wrapper.

5. **Composability:** Because `Peekable<I>` implements `Iterator`, it chains with `.map()`, `.filter()`, `.take_while()`, etc. without losing lookahead capability. OCaml's custom record does not compose with `List` higher-order functions directly.

## When to Use Each Style

**Use `.peek()` when:** you need to inspect the next value and make a branching decision — parsers, tokenizers, run-length grouping — before committing to consumption.

**Use `.next_if(pred)` when:** the advance decision is a pure predicate; this eliminates the peek-then-next two-step and makes intent explicit in one combinator call.
