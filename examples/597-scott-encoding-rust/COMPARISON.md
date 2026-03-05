# OCaml vs Rust: Scott Encoding

Scott encoding represents data by its eliminator (pattern match).

## Key Idea

Instead of storing data, store what you would do with each case:
- `True` = `λt.λf. t`
- `False` = `λt.λf. f`
- `Some(x)` = `λnone.λsome. some(x)`
- `None` = `λnone.λsome. none()`

## Rust Implementation

```rust
fn scott_match_option<A, T>(
    opt: Option<A>,
    on_none: impl Fn() -> T,
    on_some: impl Fn(A) -> T,
) -> T {
    match opt { None => on_none(), Some(a) => on_some(a) }
}
```

## vs Church Encoding

| Aspect | Church | Scott |
|--------|--------|-------|
| **Focus** | Iteration | Pattern matching |
| **Numbers** | Apply f N times | Match zero/succ |
| **Lists** | Fold | Match nil/cons |
