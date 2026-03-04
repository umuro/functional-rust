# 040: Dotstring Parse

**Difficulty:** ⭐⭐  **Level:** Foundations

Parse a dotstring into a binary tree with proper error handling using `Result`.

## The Problem This Solves

Example 039 showed the dotstring format. This example focuses on the parsing direction — and adds something critical for real software: **error handling**.

What if the input is an empty string? What if there are extra characters after a valid tree? What if the string is malformed? In a library or API, you can't just panic — you need to return a meaningful error that callers can handle.

Rust's `Result<T, E>` type makes error handling a first-class concern. The parser either returns `Ok(tree)` or `Err(reason)`. The compiler forces you to handle both cases. No exceptions that bubble up silently, no null returns to check "or was it valid?".

## The Intuition

In Python or Java, a parser might throw an exception on bad input:
```python
def parse(s):
    if not s:
        raise ValueError("empty input")
    ...
```

You catch it somewhere (maybe) or you don't (oops).

In Rust, errors are values in the type system:
```rust
fn parse_dotstring(s: &str) -> Result<Tree, ParseError>
```

The return type *says* this can fail. Callers must handle both `Ok` and `Err`. You can't accidentally ignore an error — the compiler won't let you use the tree without unwrapping the `Result`.

This is especially important for the "trailing characters" case. `"x..extra"` is a valid tree (`x`) followed by garbage. A strict parser should reject this. Our parser checks that it consumed *all* the input:

```rust
if consumed != chars.len() {
    return Err(ParseError::TrailingChars(consumed));
}
```

## How It Works in Rust

```rust
#[derive(Debug, PartialEq)]
enum ParseError {
    UnexpectedEnd,          // ran out of input mid-tree
    TrailingChars(usize),   // valid tree, but leftover input
}

fn parse_dotstring(s: &str) -> Result<Tree, ParseError> {
    let chars: Vec<char> = s.chars().collect();
    let (tree, consumed) = parse_inner(&chars, 0)?;  // '?' propagates errors

    if consumed != chars.len() {
        Err(ParseError::TrailingChars(consumed))
    } else {
        Ok(tree)
    }
}

fn parse_inner(chars: &[char], pos: usize)
    -> Result<(Tree, usize), ParseError>
{
    if pos >= chars.len() {
        return Err(ParseError::UnexpectedEnd);  // ran off the end
    }
    match chars[pos] {
        '.' => Ok((Tree::leaf(), pos + 1)),

        c => {
            let (left, pos2) = parse_inner(chars, pos + 1)?;  // ? propagates
            let (right, pos3) = parse_inner(chars, pos2)?;
            Ok((Tree::node(left, c, right), pos3))
        }
    }
}
```

**The `?` operator** is Rust's shorthand for "if this is `Err`, return it immediately; if `Ok`, unwrap the value." It threads errors up the call stack without explicit `match` at every step. This is equivalent to checked exceptions, but explicit in the type signature.

**Using the parser:**
```rust
match parse_dotstring("abd..e..") {
    Ok(tree)  => println!("Parsed: {:?}", tree),
    Err(e)    => println!("Error: {:?}", e),
}

parse_dotstring("")          // Err(UnexpectedEnd)
parse_dotstring("x..extra")  // Err(TrailingChars(3))
parse_dotstring(".")         // Ok(Leaf)
parse_dotstring("x..")       // Ok(Node('x', Leaf, Leaf))
```

## What This Unlocks

- **Real parsers**: every production parser uses `Result` (or equivalent) for error propagation.
- **`?` operator**: once you understand this pattern, Rust error handling feels clean and ergonomic.
- **Library APIs**: returning `Result` instead of panicking makes your code usable in all contexts.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error type | `exception` or `result` type | `Result<T, E>` enum — `Ok(T)` or `Err(E)` |
| Error propagation | `raise` / exception | `?` operator — explicit in function signature |
| Trailing input check | Manual check on position | Same — compare `consumed` to `chars.len()` |
| Custom errors | Variant of exception type | Custom `enum ParseError { ... }` |
| Force caller to handle | Not enforced (exceptions) | Compiler enforces — can't use `T` without unwrapping `Result<T,E>` |
