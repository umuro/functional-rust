# OCaml vs Rust: FusedIterator

## Pattern 1: Guaranteed Termination

### OCaml
```ocaml
(* OCaml Seq.t is naturally fused by convention *)
let countdown n =
  Seq.unfold (fun i -> if i <= 0 then None else Some (i, i-1)) n

(* Once None, stays None - but convention, not enforced *)
```

### Rust
```rust
use std::iter::FusedIterator;

struct Countdown { n: i32 }

impl Iterator for Countdown {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.n <= 0 { return None; }
        let val = self.n;
        self.n -= 1;
        Some(val)
    }
}

// Marker trait - enforces the guarantee
impl FusedIterator for Countdown {}
```

## Pattern 2: Safety Wrapper

### Rust
```rust
// If you don't trust an iterator, wrap it
let safe = weird_iterator.fuse();
// Now guaranteed: once None, always None
```

## Pattern 3: All Standard Iterators

### Rust
```rust
let mut v = vec![1, 2, 3].into_iter();
v.next(); // Some(1)
v.next(); // Some(2) 
v.next(); // Some(3)
v.next(); // None
v.next(); // None - all std iterators are fused
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Guarantee | Convention only | `FusedIterator` marker trait |
| Enforcement | Not by type system | Compiler-checked |
| Standard library | All well-behaved | All implement `FusedIterator` |
| Safety wrapper | N/A | `.fuse()` adapter |
| Implementation | Programmer discipline | `impl FusedIterator for T {}` |
