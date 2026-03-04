# 094: Peekable Iterator

**Difficulty:** 3  **Level:** Intermediate

Look at the next element without consuming it — essential for parsers, tokenizers, and run-grouping.

## The Problem This Solves

Sometimes you need to make a decision based on what's *coming next* in a sequence without committing to consuming it. A tokenizer needs to know if the next character continues a number. A grouping function needs to know if the next element belongs to the current group. A parser needs to decide which rule to apply before reading.

Without lookahead, you'd consume the element, realize it doesn't fit, and scramble to "push it back" — which standard iterators don't support. Peekable gives you a one-element buffer for free.

OCaml has no built-in peekable — you'd implement a `peeked` ref field manually. Rust builds it into the standard library.

## The Intuition

`.peekable()` wraps any iterator and adds a one-slot buffer. `.peek()` fills that slot (if empty) and returns a reference to it — without advancing. `.next()` drains the buffer and advances as normal.

Key detail: `.peek()` returns `Option<&Item>` — a reference to the buffered item. This means you can inspect but not move the value until you call `.next()`.

`.next_if(pred)` is the power move: advance only if the predicate passes. Consumed on match, untouched on mismatch.

## How It Works in Rust

```rust
// Consume while condition holds — peek before deciding to advance
fn sum_while_positive(data: &[i32]) -> i32 {
    let mut iter = data.iter().peekable();
    let mut sum = 0;
    while let Some(&&val) = iter.peek() {
        if val <= 0 { break; }  // stop but don't consume the negative element
        sum += iter.next().unwrap();
    }
    sum
}

// Tokenizer: group consecutive digits into a number token
fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' => { chars.next(); }
            '0'..='9' => {
                let mut num = String::new();
                // Keep consuming digits as long as the NEXT char is a digit
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num.push(c);
                        chars.next();  // now consume it
                    } else {
                        break;  // next char isn't a digit — leave it for next token
                    }
                }
                tokens.push(Token::Num(num.parse().unwrap()));
            }
            '+' => { chars.next(); tokens.push(Token::Plus); }
            _ => { chars.next(); }
        }
    }
    tokens
}

// next_if: conditional advance — returns Some(item) on match, None otherwise
fn consume_if_digit(iter: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Option<char> {
    iter.next_if(|c| c.is_ascii_digit())
}
```

## What This Unlocks

- **Tokenizers and parsers**: group multi-character tokens (numbers, identifiers) cleanly.
- **Run-length encoding**: peek to check if the next element continues the current run.
- **Streaming protocols**: consume a frame header, then decide how many bytes to read next.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Built-in peekable | No (manual buffer) | `.peekable()` adapter |
| Peek return type | `'a option` (owned) | `Option<&Item>` (borrowed ref) |
| Conditional advance | Manual check + next | `.next_if(pred)` |
| Lookahead depth | Custom (any) | 1 element only |
| Push-back | Manual `peeked` field | Implicit in Peekable buffer |
