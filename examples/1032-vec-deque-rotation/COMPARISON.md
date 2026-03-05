# VecDeque Rotation — Comparison

## Core Insight
Rust provides a purpose-built ring buffer (`VecDeque`) with O(1) operations at both ends and built-in rotation. OCaml's standard library lacks this; the common workaround is a two-list queue (amortized O(1)) or a simple list (O(1) front, O(n) back).

## OCaml Approach
- List: O(1) push/pop front, O(n) back operations
- Two-list deque: `{ front; back }` with lazy reversal for amortized O(1)
- Manual rotation: split list at index, append halves
- `Queue` module exists but is mutable and not a deque

## Rust Approach
- `VecDeque<T>`: ring buffer backed by contiguous array
- O(1) `push_front`, `push_back`, `pop_front`, `pop_back`
- `rotate_left(n)` / `rotate_right(n)`: built-in rotation
- Indexed access via `dq[i]`
- `make_contiguous()` to linearize internal buffer
- Excellent for sliding windows

## Comparison Table

| Feature | OCaml (list/two-list) | Rust (`VecDeque`) |
|---|---|---|
| Push front | O(1) | O(1) |
| Push back | O(n) list / amortized O(1) | O(1) |
| Pop front | O(1) | O(1) |
| Pop back | O(n) list / amortized O(1) | O(1) |
| Rotation | Manual O(n) | Built-in O(n) |
| Random access | O(n) | O(1) |
| Memory layout | Linked nodes | Contiguous ring buffer |
| Cache friendly | No | Yes |
