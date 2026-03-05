## Core Insight

Tail recursion with an accumulator prevents stack overflow for large inputs. OCaml guarantees tail-call optimization. Rust does NOT guarantee TCO, making explicit loops the idiomatic replacement.

## OCaml Approach
- Naive: `let rec sum = function [] -> 0 | x::xs -> x + sum xs`
- Tail-recursive: `let rec aux acc = function [] -> acc | x::xs -> aux (acc+x) xs`
- TCO guaranteed by the compiler

## Rust Approach
- Recursive version works but may overflow stack
- Idiomatic: `iter().fold()` or explicit loop
- No guaranteed TCO in Rust

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| TCO guaranteed | Yes | No |
| Accumulator pattern | `aux acc rest` | loop + mutable acc |
| Idiomatic | Tail recursion | `.fold()` or `for` loop |
| Stack overflow risk | No (with TCO) | Yes (with recursion) |
