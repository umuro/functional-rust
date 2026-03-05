# Deque — Comparison

## Core Insight
The functional two-list deque (front, back) achieves amortized O(1) by reversing the back list into the front when the front empties. OCaml's immutable lists make this pattern natural; Rust can implement it with `Vec` but idiomatic Rust uses the built-in `VecDeque` (circular buffer, true O(1) at both ends).

## OCaml Approach
- `{ front: 'a list; back: 'a list }` — pure functional record
- `List.rev` to rebalance when front is empty
- All operations return new deques (immutable, persistent)
- Pattern matching: `match d.front with [] -> ...`
- `{ d with front = x :: d.front }` — record update syntax

## Rust Approach (two approaches)
- `VecDeque<T>` — std ring buffer, O(1) push/pop both ends, preferred
- `push_front`, `push_back`, `pop_front`, `pop_back` — direct methods
- Functional version: `Vec<T>` as stack (`.push` = push to top, `.pop` = pop from top)
- Functional version: `mut self` — consume and return new (value semantics mimicking immutability)
- `self.front.reverse()` for rebalancing

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Idiomatic impl | Two-list `{ front; back }` | `VecDeque<T>` (ring buffer) |
| Push front | `x :: d.front` (O(1)) | `push_front(x)` (O(1)) |
| Push back | `x :: d.back` (O(1)) | `push_back(x)` (O(1)) |
| Pop front | Pattern match on list | `pop_front()` → `Option<T>` |
| Rebalance | `List.rev d.back` | `front.reverse()` |
| Mutability | Immutable (persistent) | `&mut self` (mutable) |
| Value type | Record (immutable) | `VecDeque<T>` (mutable) or owned struct |
| Memory | GC'd list nodes | Contiguous ring buffer |
