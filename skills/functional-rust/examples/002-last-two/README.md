# 002: Last Two Elements

**Difficulty:** ⭐  **Level:** Beginner

Grab the last two items from a list as a pair — safely.

## The Problem This Solves

Sometimes you need the last two elements together — the previous and current state, two endpoints of a line, the penultimate and final item in a sequence. You need *both*, or nothing at all if the list is too short.

The naïve approach in most languages is: check the length, then index. But length checks and indexing are two separate operations — you can get them out of sync, and they're verbose. Python's `lst[-2], lst[-1]` is clean for two variables but doesn't naturally bundle them as a pair, and still crashes on short lists.

Rust's slice patterns let you express this intent directly: "if this list has at least two elements, give me the last two as a tuple." The compiler ensures you handle the case where it doesn't.

## The Intuition

In Python you'd write:

```python
def last_two(lst):
    if len(lst) < 2:
        return None
    return lst[-2], lst[-1]
```

Rust's pattern matching expresses the same logic more directly. The pattern `[.., a, b]` reads as: "match any list that ends with two elements `a` and `b`, regardless of what comes before the `..`."

The return type `Option<(&T, &T)>` is exactly what Python's `Optional[Tuple[T, T]]` would be — except the compiler guarantees you handle the `None` case before using the values.

## How It Works in Rust

```rust
fn last_two<T>(list: &[T]) -> Option<(&T, &T)> {
    match list {
        [.., a, b] => Some((a, b)),  // 2+ elements: return last two
        _          => None,           // 0 or 1 elements: nothing
    }
}
```

The `[.., a, b]` pattern is Rust's slice pattern syntax. `..` means "zero or more elements I don't care about." This is O(1) — Rust doesn't scan the list, it just looks at the last two positions.

Alternative using `split_last` (more explicit, reads like prose):

```rust
fn last_two_split<T>(list: &[T]) -> Option<(&T, &T)> {
    let (last, rest) = list.split_last()?;        // peel off last element
    let (second_last, _) = rest.split_last()?;    // peel off second-to-last
    Some((second_last, last))
}
```

The `?` operator short-circuits: if `split_last()` returns `None` (list is empty), the function immediately returns `None`. It's Rust's clean alternative to nested `if let` chains.

Using the result:

```rust
match last_two(&[1, 2, 3, 4]) {
    Some((a, b)) => println!("Last two: {} and {}", a, b),
    None         => println!("Need at least 2 elements"),
}
```

## What This Unlocks

- **Pairwise comparisons** — comparing consecutive states, moves, or readings
- **Sliding windows** — `windows(2)` is the iterator version for processing all adjacent pairs across a full list
- **Decoding protocols** — many binary formats use a pair of bytes for type+length headers

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Pattern for last two | `[x; y]` at end of recursion | `[.., a, b]` slice pattern |
| Return type | `'a * 'a option` | `Option<(&T, &T)>` |
| Tuple syntax | `(x, y)` | `(x, y)` |
| Traversal needed | O(n) linked list walk | O(1) slice indexing |
| Null propagation | Manual match | `?` operator |
