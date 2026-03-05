# Comparison: Custom Iterator

## Fibonacci

**OCaml:**
```ocaml
let fibonacci () =
  let rec aux a b () =
    Seq.Cons (a, aux b (a + b))
  in
  aux 0 1

let first10 = seq_take 10 (fibonacci ())
```

**Rust:**
```rust
struct Fibonacci { a: u64, b: u64 }

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let val = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(val)
    }
}

let first10: Vec<u64> = Fibonacci::new().take(10).collect();
```

## Range with Step

**OCaml:**
```ocaml
let int_step_range start stop step =
  { current = start; stop; step; add = (+); compare }
```

**Rust:**
```rust
struct StepRange<T> { current: T, end_: T, step: T }

impl Iterator for StepRange<i64> {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.current >= self.end_ { None }
        else { let v = self.current; self.current += self.step; Some(v) }
    }
}
```

## Collatz Sequence

**OCaml:**
```ocaml
let collatz_list n =
  let rec aux v acc =
    if v = 1 then List.rev (1 :: acc)
    else aux (if v mod 2 = 0 then v / 2 else 3 * v + 1) (v :: acc)
  in aux n []
```

**Rust:**
```rust
struct Collatz { current: u64, done_: bool }

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.done_ { return None; }
        let val = self.current;
        if val == 1 { self.done_ = true; }
        else if val % 2 == 0 { self.current = val / 2; }
        else { self.current = 3 * val + 1; }
        Some(val)
    }
}
```
