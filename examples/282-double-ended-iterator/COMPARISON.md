# OCaml vs Rust: DoubleEndedIterator

## Pattern 1: Reversing a Sequence

### OCaml
```ocaml
let nums = [1; 2; 3; 4; 5] in
let reversed = List.rev nums  (* allocates new list *)
```

### Rust
```rust
let reversed: Vec<i32> = (1..=5).rev().collect();
// No allocation until collect() - rev() just swaps direction
```

## Pattern 2: Consuming from Both Ends

### OCaml
```ocaml
let arr = Array.of_list nums in
let n = Array.length arr in
let front = ref 0 and back = ref (n - 1) in
while !front <= !back do
  Printf.printf "%d %d " arr.(!front) arr.(!back);
  incr front; decr back
done
```

### Rust
```rust
let mut counter = Counter::new(5);
loop {
    match (counter.next(), counter.next_back()) {
        (Some(f), Some(b)) => println!("{} {}", f, b),
        (Some(x), None) | (None, Some(x)) => println!("{}", x),
        (None, None) => break,
    }
}
```

## Pattern 3: Implementing DoubleEndedIterator

### Rust
```rust
struct Counter { front: i32, back: i32 }

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.front > self.back { return None; }
        let v = self.front;
        self.front += 1;
        Some(v)
    }
}

impl DoubleEndedIterator for Counter {
    fn next_back(&mut self) -> Option<i32> {
        if self.front > self.back { return None; }
        let v = self.back;
        self.back -= 1;
        Some(v)
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Reverse | `List.rev` allocates new list | `.rev()` is zero-allocation |
| Both ends | Manual index tracking | `next()` + `next_back()` |
| Trait | None built-in | `DoubleEndedIterator` |
| Custom impl | N/A | Implement `next_back()` |
| Mechanism | Creates new collection | Swaps roles of front/back |
