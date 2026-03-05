# 414: Recursive Macro Patterns

**Difficulty:** 4  **Level:** Expert

Macros that call themselves — process token lists one element at a time to implement algorithms purely at compile time.

## The Problem This Solves

Some macro operations are inherently sequential: count elements, reverse a list, build a computation step by step. Repetition patterns (`$(...)*`) handle parallel expansion well, but they can't accumulate state across iterations — there's no loop variable, no counter, no accumulator. For algorithms that need to process elements one at a time and carry state, recursion is the answer.

Recursive macros also let you implement conditional logic: "if this pattern matches, do X; otherwise, recurse and try Y." This is the basis of complex macro DSLs — grammars that can't be expressed with a single match arm but unfold naturally through recursive matching.

The standard library uses recursive macros for `vec![]`, `println!` argument parsing, and the `matches!` macro. Third-party crates use them to implement entire embedded languages inside Rust syntax.

## The Intuition

Recursive macro patterns mirror recursive functions: there's a base case (the simplest input, no recursion) and a recursive case (consume one element, recurse on the rest). The "accumulator" pattern is common: an internal arm carries state forward through a private `@tag` to distinguish internal calls from the public entry point.

The `@tag` convention (e.g., `@acc`) is unofficial but universal — it's a common token that won't appear in normal user code, preventing accidental matches. The entry arm is the clean public interface; the `@acc` arms are the implementation.

## How It Works in Rust

```rust
// Count elements by recursing and adding 1 per element
macro_rules! count {
    () => { 0usize };                          // base case: empty → 0
    ($head:expr $(, $tail:expr)*) => {
        1 + count!($($tail),*)                 // consume head, recurse on tail
    };
}

// Reverse a list using an accumulator
// @acc carries the reversed list built so far
macro_rules! reverse_list {
    // Base: accumulator is the result
    (@acc [$($acc:expr),*]) => {
        [$($acc),*]
    };
    // Recursive: move head to front of accumulator
    (@acc [$($acc:expr),*] $head:expr $(, $tail:expr)*) => {
        reverse_list!(@acc [$head $(, $acc)*] $($tail),*)
    };
    // Public entry: start with empty accumulator
    ($($x:expr),* $(,)?) => {
        reverse_list!(@acc [] $($x),*)
    };
}

// Match a value against multiple options
macro_rules! one_of {
    ($val:expr, $first:expr) => { $val == $first };  // base: single option
    ($val:expr, $first:expr $(, $rest:expr)+) => {
        $val == $first || one_of!($val $(, $rest)+)  // recurse on remaining
    };
}

// Join strings with a separator, recursively
macro_rules! concat_with {
    ($sep:expr; $a:expr) => { $a.to_string() };  // base: single element
    ($sep:expr; $a:expr $(, $rest:expr)+) => {
        format!("{}{}{}", $a, $sep, concat_with!($sep; $($rest),+))
    };
}

fn main() {
    // count: 0, 1, 3, 5 elements
    assert_eq!(count!(), 0);
    assert_eq!(count!(a), 1);
    assert_eq!(count!(a, b, c), 3);

    // reverse
    let rev = reverse_list![1, 2, 3, 4, 5];
    println!("reversed: {:?}", rev);  // [5, 4, 3, 2, 1]

    // one_of: membership test
    let x = 5;
    println!("x in {{1,3,5}}: {}", one_of!(x, 1, 3, 5));   // true
    println!("x in {{2,4,6}}: {}", one_of!(x, 2, 4, 6));   // false

    // concat_with separator
    println!("{}", concat_with!(", "; "one", "two", "three"));
    // "one, two, three"
}
```

**Recursion depth**: the compiler limits macro recursion (default 128 levels). Deep DSLs can hit this; `#![recursion_limit = "256"]` raises it. Recursion is evaluated at compile time — no runtime stack involvement.

## What This Unlocks

- **Compile-time algorithms** — count, reverse, sum, zip — run at zero runtime cost; results are constants or stack-allocated arrays.
- **Pattern-driven DSLs** — recursive matching enables grammars: parse `a + b * c` by consuming one operator at a time, building AST nodes at compile time.
- **Self-contained boilerplate elimination** — `one_of!(x, A, B, C, D)` expands to `x == A || x == B || x == C || x == D` — cleaner than writing it out, and scales to any length.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive list processing | `let rec f acc = function [] -> acc \| x::xs -> f (x::acc) xs` — runtime | `macro_rules!` recursion — compile time, expands to flat code |
| Accumulator pattern | `reverse_acc acc list` — standard recursion style | `@acc` tagged arm — internal convention, same concept |
| Recursion depth | Stack depth (runtime) | Compiler macro recursion limit (default 128, configurable) |
| Result type | OCaml value | Rust tokens → any Rust construct (expressions, types, items) |
