# OCaml vs Rust: Custom Iterator

## Pattern 1: Custom Sequence Type

### OCaml
```ocaml
type 'a counter = {
  mutable current: int;
  max: int;
  step: int;
  f: int -> 'a;
}

let counter_next c =
  if c.current >= c.max then None
  else begin
    let v = c.f c.current in
    c.current <- c.current + c.step;
    Some v
  end

let counter_to_seq c =
  Seq.unfold (fun () ->
    counter_next c |> Option.map (fun v -> (v, ()))
  ) ()
```

### Rust
```rust
struct Squares { current: u32, max: u32 }

impl Iterator for Squares {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max { return None; }
        let val = self.current * self.current;
        self.current += 1;
        Some(val)
    }
}
```

## Pattern 2: Infinite Sequence (Fibonacci)

### OCaml
```ocaml
let fib_seq =
  Seq.unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)

let first10 = Seq.take 10 fib_seq |> List.of_seq
```

### Rust
```rust
struct Fibonacci { a: u64, b: u64 }

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let val = self.a;
        self.a = self.b;
        self.b = val + self.b;
        Some(val)  // always Some — infinite
    }
}

let first10: Vec<u64> = Fibonacci::new().take(10).collect();
```

## Pattern 3: Using Iterator Adapters

### OCaml
```ocaml
let result = 
  counter_to_seq (make_counter 10 (fun i -> i * i))
  |> Seq.filter (fun x -> x > 10)
  |> List.of_seq
```

### Rust
```rust
let result: Vec<u32> = Squares::new(10)
    .filter(|&x| x > 10)
    .collect();
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Trait required | None (use `Seq.unfold`) | Implement `Iterator` trait |
| Minimum method | `unit -> 'a Seq.node` | `fn next(&mut self) -> Option<Item>` |
| State storage | Closure captures | Struct fields |
| Adapters | Separate `Seq.*` functions | Methods on `Iterator` trait |
| Method chaining | `|>` pipeline operator | `.method().method()` |
| Infinite sequences | Natural with `Seq.unfold` | Return `Some` forever, use `take()` |
