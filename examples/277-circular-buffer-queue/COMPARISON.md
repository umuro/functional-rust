# OCaml vs Rust: Circular Buffer — Functional Queue

## Side-by-Side Code

### OCaml
```ocaml
type 'a queue = { front: 'a list; back: 'a list }

let empty = { front = []; back = [] }
let is_empty q = q.front = [] && q.back = []
let enqueue x q = { q with back = x :: q.back }

let dequeue q = match q.front with
  | h :: t -> Some (h, { q with front = t })
  | [] -> match List.rev q.back with
    | [] -> None
    | h :: t -> Some (h, { front = t; back = [] })

let to_list q = q.front @ List.rev q.back
```

### Rust (idiomatic)
```rust
#[derive(Debug, Clone)]
pub struct Queue<T> {
    front: Vec<T>,
    back: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self { Queue { front: Vec::new(), back: Vec::new() } }
    pub fn is_empty(&self) -> bool { self.front.is_empty() && self.back.is_empty() }
    pub fn enqueue(mut self, x: T) -> Self { self.back.push(x); self }

    pub fn dequeue(mut self) -> Option<(T, Self)> {
        if self.front.is_empty() {
            if self.back.is_empty() { return None; }
            self.back.reverse();
            std::mem::swap(&mut self.front, &mut self.back);
        }
        let head = self.front.remove(0);
        Some((head, self))
    }
}
```

### Rust (functional/recursive drain)
```rust
pub fn drain_recursive<T>(queue: Queue<T>) -> Vec<T> {
    match queue.dequeue() {
        None => Vec::new(),
        Some((x, rest)) => {
            let mut result = vec![x];
            result.extend(drain_recursive(rest));
            result
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Queue type | `'a queue` (record with two lists) | `Queue<T>` (struct with two Vecs) |
| Empty check | `val is_empty : 'a queue -> bool` | `fn is_empty(&self) -> bool` |
| Enqueue | `val enqueue : 'a -> 'a queue -> 'a queue` | `fn enqueue(self, x: T) -> Self` |
| Dequeue | `val dequeue : 'a queue -> ('a * 'a queue) option` | `fn dequeue(self) -> Option<(T, Self)>` |
| To list | `val to_list : 'a queue -> 'a list` | `fn to_vec(&self) -> Vec<&T>` |

## Key Insights

1. **Consuming self mirrors immutability:** Rust's `fn dequeue(self)` takes ownership, preventing use of the old queue — this enforces the same "one version at a time" discipline as OCaml's immutable approach, but through the type system rather than runtime copying
2. **`std::mem::swap` for efficient reversal:** Instead of OCaml's `List.rev` which allocates a new list, Rust reverses `back` in place and swaps it into `front` with zero allocation
3. **Tuple return pattern:** Both languages return `Option<(T, Queue)>` — the element and the remaining queue. This is natural in OCaml; in Rust it works because `Self` is moved, not copied
4. **Record update vs mutation:** OCaml's `{ q with back = x :: q.back }` creates a new record sharing most fields; Rust's `mut self` modifies the existing struct in place — same semantics from the caller's perspective
5. **Amortized cost:** Both implementations achieve amortized O(1) per operation — each element is reversed at most once. The Rust version additionally benefits from cache-friendly Vec layout

## When to Use Each Style

**Use idiomatic Rust when:** You need a production queue — use `VecDeque` from std which provides O(1) push/pop on both ends with a ring buffer. The two-Vec approach is primarily pedagogical.

**Use recursive Rust when:** Teaching functional data structure concepts — the recursive `drain` mirrors OCaml's recursive traversal pattern and makes the "peel one element at a time" structure explicit.
