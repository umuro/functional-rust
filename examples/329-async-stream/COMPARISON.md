# OCaml vs Rust: Async Streams

## Lazy Range

**OCaml:**
```ocaml
type 'a stream = Empty | Cons of 'a * (unit -> 'a stream)

let range_stream start stop =
  let rec loop i () = if i>=stop then Empty else Cons(i, loop (i+1))
  in loop start ()
```

**Rust:**
```rust
struct RangeStream { current: i64, end: i64 }

impl Iterator for RangeStream {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.current >= self.end { None }
        else { let v = self.current; self.current += 1; Some(v) }
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Stream type | ADT with thunks | Iterator trait |
| Laziness | Explicit closures | Implicit in `next()` |
| State | In closures | In struct fields |
| Combinators | Manual recursion | Built-in `.filter()`, `.map()` |
| Memory | GC handles thunks | No allocation (stack) |
