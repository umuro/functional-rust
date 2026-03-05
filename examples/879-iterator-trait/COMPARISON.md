# Comparison: Iterator Trait

## Core Implementation

**OCaml — Seq:**
```ocaml
let range start stop =
  let rec aux n () =
    if n >= stop then Seq.Nil
    else Seq.Cons (n, aux (n + 1))
  in
  aux start
```

**Rust — Iterator trait:**
```rust
struct Range { current: i32, end_: i32 }

impl Iterator for Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.current >= self.end_ { None }
        else { let v = self.current; self.current += 1; Some(v) }
    }
}
```

## Map/Filter (Manual vs Free)

**OCaml — Must implement manually for custom iterators:**
```ocaml
let iter_map f it =
  { next = fun () -> match it.next () with
    | None -> None | Some v -> Some (f v) }
```

**Rust — Free from Iterator trait:**
```rust
// Just implementing next() gives you:
Range::new(1, 6).map(|x| x * 2).filter(|x| x > 5).collect::<Vec<_>>()
```

## Infinite Sequences

**OCaml:**
```ocaml
let counter_from n =
  let c = ref n in
  { next = fun () -> let v = !c in c := v + 1; Some v }

let first5 = take 5 (counter_from 0)
```

**Rust:**
```rust
impl Iterator for Counter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let v = self.current; self.current += 1; Some(v)
    }
}

let first5: Vec<u64> = Counter::from(0).take(5).collect();
```
