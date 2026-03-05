# OCaml vs Rust: Iterator Trait Deep Dive

## Side-by-Side Code

### OCaml — Seq module
```ocaml
let range a b = Seq.init (b - a) (fun i -> a + i)

let sum_of_squares_of_evens =
  range 1 11
  |> Seq.filter (fun x -> x mod 2 = 0)
  |> Seq.map (fun x -> x * x)
  |> Seq.fold_left (+) 0

(* flat_map *)
let pairs =
  range 1 4
  |> Seq.flat_map (fun x -> Seq.map (fun y -> (x, y)) (range 1 4))
  |> Seq.filter (fun (x, y) -> x < y)
  |> List.of_seq

(* zip *)
let names = List.to_seq ["Alice"; "Bob"; "Carol"]
let scores = List.to_seq [95; 87; 91]
let combined = Seq.zip names scores
```

### Rust — Iterator trait
```rust
// Pipeline
let sum: i32 = (1..=10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .sum();

// flat_map
let pairs: Vec<(i32, i32)> = (1..=3)
    .flat_map(|x| (1..=3).map(move |y| (x, y)))
    .filter(|(x, y)| x < y)
    .collect();

// zip
let names = ["Alice", "Bob", "Carol"];
let scores = [95, 87, 91];
let combined: Vec<_> = names.iter().zip(scores.iter()).collect();
```

---

## Comparison Table

| Adapter | OCaml (Seq) | Rust (Iterator) |
|---------|-------------|-----------------|
| Transform | `Seq.map f` | `.map(f)` |
| Filter | `Seq.filter p` | `.filter(p)` |
| Reduce | `Seq.fold_left f init` | `.fold(init, f)` |
| Flatten | `Seq.flat_map f` | `.flat_map(f)` |
| Take first n | `Seq.take n` | `.take(n)` |
| Zip | `Seq.zip a b` | `a.zip(b)` |
| Collect | `List.of_seq` | `.collect()` |
| Sum | `Seq.fold_left (+) 0` | `.sum()` |
| Find | `Seq.find p` | `.find(p)` |

---

## Custom Iterators

### OCaml — Using Seq
```ocaml
let rec fibonacci a b =
  fun () -> Seq.Cons (a, fibonacci b (a + b))

let fibs = fibonacci 1 1
let first_10 = fibs |> Seq.take 10 |> List.of_seq
```

### Rust — Implementing Iterator
```rust
struct Fibonacci { curr: u64, next: u64 }

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

let fibs: Vec<u64> = Fibonacci { curr: 0, next: 1 }.take(10).collect();
```

---

## Laziness

Both OCaml's `Seq` and Rust's `Iterator` are **lazy** — no work is done until consumed:

```rust
// Nothing happens yet
let iter = (1..1_000_000).filter(|x| x % 2 == 0).map(|x| x * x);

// Now it runs, but only processes first 5
let result: Vec<_> = iter.take(5).collect();
```

OCaml's `Seq` is similarly lazy:
```ocaml
let iter = range 1 1000000
  |> Seq.filter (fun x -> x mod 2 = 0)
  |> Seq.map (fun x -> x * x)

let result = iter |> Seq.take 5 |> List.of_seq
```

---

## Stateful Iteration

### Rust — scan
```rust
// Running sum
let running: Vec<i32> = (1..=4)
    .scan(0, |state, x| { *state += x; Some(*state) })
    .collect();
// [1, 3, 6, 10]
```

### OCaml — scan equivalent
```ocaml
let scan f init seq =
  let state = ref init in
  Seq.map (fun x -> state := f !state x; !state) seq

let running = range 1 5 |> scan (+) 0 |> List.of_seq
```

---

## 5 Takeaways

1. **Both are lazy by default.**
   No computation until `.collect()` / `List.of_seq`.

2. **Rust's `move` keyword captures variables in closures.**
   `flat_map(|x| (1..=n).map(move |y| (x, y)))` — `move` gives the inner closure ownership.

3. **Custom iterators are straightforward in Rust.**
   Implement `Iterator` with `type Item` and `fn next()`.

4. **Rust has more built-in adapters.**
   `scan`, `peekable`, `enumerate`, `partition`, `unzip` — all in std.

5. **Type inference works across iterator chains.**
   Rust knows the types through the entire pipeline.
