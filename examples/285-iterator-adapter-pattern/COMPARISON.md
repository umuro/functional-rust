# OCaml vs Rust: Iterator Adapter Pattern

## Pattern 1: Every Nth Element

### OCaml
```ocaml
let every_nth n seq =
  Seq.unfold (fun (i, rest) ->
    let rec skip_to k s =
      if k = 0 then
        match Seq.uncons s with
        | Some (v, rest') -> Some (v, (n-1, rest'))
        | None -> None
      else
        match Seq.uncons s with
        | Some (_, rest') -> skip_to (k-1) rest'
        | None -> None
    in
    skip_to i rest
  ) (0, seq)
```

### Rust
```rust
struct EveryNth<I> { inner: I, n: usize, count: usize }

impl<I: Iterator> Iterator for EveryNth<I> {
    type Item = I::Item;
    
    fn next(&mut self) -> Option<I::Item> {
        loop {
            let item = self.inner.next()?;
            let emit = self.count % self.n == 0;
            self.count += 1;
            if emit { return Some(item); }
        }
    }
}
```

## Pattern 2: Sliding Window Pairs

### OCaml
```ocaml
let pairs seq =
  Seq.unfold (fun s ->
    match Seq.uncons s with
    | None -> None
    | Some (a, rest) ->
      match Seq.uncons rest with
      | None -> None
      | Some (b, _) -> Some ((a, b), Seq.drop 1 s)
  ) seq
```

### Rust
```rust
struct Pairs<I: Iterator> { inner: I, prev: Option<I::Item> }

impl<I: Iterator> Iterator for Pairs<I> 
where I::Item: Clone 
{
    type Item = (I::Item, I::Item);
    
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        let prev = self.prev.replace(next.clone())?;
        Some((prev, next))
    }
}
```

## Pattern 3: Extension Trait

### Rust
```rust
trait IteratorExt: Iterator + Sized {
    fn every_nth(self, n: usize) -> EveryNth<Self> {
        EveryNth::new(self, n)
    }
    fn pairs(self) -> Pairs<Self> where Self::Item: Clone {
        Pairs::new(self)
    }
}
impl<I: Iterator> IteratorExt for I {}

// Now usable on any iterator:
let result = (0..20).every_nth(3).pairs().collect();
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Adapter type | Function over `Seq` | Struct + `impl Iterator` |
| Extension method | Module or `|>` pipeline | Extension trait |
| Composability | `Seq` functions via `|>` | Method chaining |
| Type signature | `'a Seq.t -> 'b Seq.t` | `Adapter<I> where I: Iterator` |
| Lazy evaluation | `Seq` is lazy | All iterators lazy |
