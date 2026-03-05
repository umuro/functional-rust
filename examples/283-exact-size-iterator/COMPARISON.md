# OCaml vs Rust: ExactSizeIterator

## Pattern 1: Known-Length Collection

### OCaml
```ocaml
let arr = [|1; 2; 3; 4; 5|] in
Printf.printf "Length: %d\n" (Array.length arr)
(* Arrays always know their length *)
```

### Rust
```rust
let arr = [1i32, 2, 3, 4, 5];
let mut iter = arr.iter();
println!("Length: {}", iter.len()); // 5
iter.next();
println!("Remaining: {}", iter.len()); // 4 - tracks consumed
```

## Pattern 2: Pre-allocation

### OCaml
```ocaml
let src = Array.init 100 (fun i -> i * i) in
let target = Array.make (Array.length src) 0 in
Array.blit src 0 target 0 (Array.length src)
```

### Rust
```rust
let source = vec![1i32, 2, 3, 4, 5];
let mut dest = Vec::with_capacity(source.iter().len());
dest.extend(source.iter().map(|&x| x * 2));
// Single allocation - no reallocs
```

## Pattern 3: Custom ExactSizeIterator

### Rust
```rust
struct FixedRange { current: usize, end: usize }

impl Iterator for FixedRange {
    type Item = usize;
    
    fn next(&mut self) -> Option<usize> { /* ... */ }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.current);
        (remaining, Some(remaining))  // exact bounds
    }
}

impl ExactSizeIterator for FixedRange {
    fn len(&self) -> usize {
        self.end.saturating_sub(self.current)
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Length access | `Array.length` (O(1)) | `.len()` on `ExactSizeIterator` |
| List length | `List.length` (O(n)) | N/A (iterators can be O(1)) |
| Iterator size | Not tracked | `size_hint()` + `len()` |
| Pre-allocation | Manual size tracking | `Vec::with_capacity(iter.len())` |
| Dynamic tracking | No | `len()` decreases as items consumed |
