# Comparison: Lazy Sequences

## Infinite Naturals

**OCaml:**
```ocaml
let naturals () =
  let rec aux n () = Seq.Cons (n, aux (n + 1)) in
  aux 0
```

**Rust:**
```rust
fn naturals() -> impl Iterator<Item = u64> {
    0u64..
}
```

## Derived Sequences

**OCaml:**
```ocaml
let squares () = Seq.map (fun n -> n * n) (naturals ())
let primes () = Seq.filter is_prime (naturals ())
```

**Rust:**
```rust
fn squares() -> impl Iterator<Item = u64> { naturals().map(|n| n * n) }
fn primes() -> impl Iterator<Item = u64> { naturals().filter(|&n| is_prime(n)) }
```

## Recursive Generation

**OCaml:**
```ocaml
let powers_of base =
  let rec aux p () = Seq.Cons (p, aux (p * base)) in
  aux 1
```

**Rust:**
```rust
fn powers_of(base: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(1u64), move |&prev| prev.checked_mul(base))
}
```

## Consuming Infinite Sequences

**OCaml:**
```ocaml
let small_primes = seq_take_while (fun x -> x < 20) (primes ())
let first_big = seq_drop_while (fun x -> x <= 100) (primes ())
```

**Rust:**
```rust
let small_primes: Vec<_> = primes().take_while(|&p| p < 20).collect();
let first_big = primes().find(|&p| p > 100);
```
