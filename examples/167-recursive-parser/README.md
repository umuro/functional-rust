# 167: Recursive Parser

**Difficulty:** 3  **Level:** Advanced

Handle nesting — `(1 + (2 * 3))` — where a parser must call itself to parse its own contents.

## The Problem This Solves

Some grammars are recursive by nature: a list can contain lists, an expression can contain parenthesized expressions, an S-expression can contain S-expressions. The grammar rule for a list is literally `list = "[" (item | list)* "]"`. The parser for `list` needs to call `list` itself.

In OCaml, `let rec` handles this naturally — you write `let rec parse_expr input = ... parse_expr ...` and it works. In Rust, closures can't reference themselves directly. A closure is a value stored in a variable; you can't capture a variable that holds the closure you're defining.

This example shows three approaches to work around Rust's restriction: (1) regular named functions, which *can* call themselves; (2) `Rc<RefCell<Option<...>>>` for deferred initialization; (3) a `fix`-point combinator that encapsulates the pattern.

## The Intuition

The simplest fix: use named functions instead of closures. Named functions in Rust can call themselves recursively — the restriction only applies to closures. If your recursive parser doesn't need to capture variables from the environment, this is all you need.

```rust
// This works — named function can call itself
fn parse_sexp(input: &str) -> ParseResult<Sexp> {
    if input.starts_with('(') {
        // ... parse list items by calling parse_sexp recursively
        let (item, rest) = parse_sexp(item_input)?;
    }
    // ...
}
```

## How It Works in Rust

```rust
// Approach 1: Named recursive functions (simplest)
fn parse_list(input: &str) -> ParseResult<Vec<i64>> {
    let input = input.trim_start();
    let input = input.strip_prefix('[')
        .ok_or("expected '['")?;
    let mut items = Vec::new();
    let mut remaining = input.trim_start();
    while !remaining.starts_with(']') {
        if remaining.is_empty() {
            return Err("unexpected end of input".to_string());
        }
        // Recursive call to parse_value — which may call parse_list again
        let (val, rest) = parse_value(remaining)?;
        items.push(val);
        remaining = rest.trim_start();
        remaining = remaining.strip_prefix(',').unwrap_or(remaining).trim_start();
    }
    Ok((items, &remaining[1..]))
}

// Approach 2: Rc for recursive closures
use std::rc::Rc;
use std::cell::RefCell;

fn make_recursive_parser() -> impl Fn(&str) -> ParseResult<i64> {
    // Forward declaration — initialized after we build the closure
    let parser_ref: Rc<RefCell<Option<Box<dyn Fn(&str) -> ParseResult<i64>>>>> =
        Rc::new(RefCell::new(None));

    let parser_ref_clone = Rc::clone(&parser_ref);
    let parser = move |input: &str| {
        // Borrow the inner parser through the Rc
        let inner = parser_ref_clone.borrow();
        let p = inner.as_ref().unwrap();
        p(input)  // call the real implementation
    };

    // Now set the actual implementation
    *parser_ref.borrow_mut() = Some(Box::new(/* real parser */));
    parser
}
```

## What This Unlocks

- **Nested expressions** — `(1 + (2 * 3))` requires an expression parser that calls itself.
- **Tree-structured data** — JSON arrays containing JSON values, S-expressions containing S-expressions.
- **Any context-free grammar** — recursive grammars are the norm, not the exception.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive functions | `let rec parse input = ... parse ...` | Named functions can self-call; closures cannot |
| Forward declaration | `let p = ref (fun _ -> failwith "unset")` | `Rc<RefCell<Option<Box<dyn Fn(...)>>>>` |
| Fix-point combinator | `let rec fix f x = f (fix f) x` | `rc_fix` wraps with `Rc` to break the self-reference |
| Mutual recursion | `let rec a ... and b ...` | Two named functions calling each other — works fine |
