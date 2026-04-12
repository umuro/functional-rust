**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[deque on hightechmind.io](https://hightechmind.io/posts/functional-rust/deque)

---

## Problem Statement

Implement a double-ended queue (deque) that supports O(1) amortized push and pop at both front and back. Present two approaches: wrapping Rust's standard `VecDeque` (ring buffer internally) and a functional two-stack deque that mirrors OCaml's approach — two lists where the front list and reversed back list together represent the queue contents.

## Learning Outcomes

- Use `VecDeque<T>` from `std::collections` for a production-ready O(1) amortized deque
- Understand the two-stack functional deque: `(front_list, back_list)` where `pop_front` consumes `front`, and when `front` is empty it reverses `back` into `front`
- Implement the rebalancing step: when `front` is empty on `pop_front`, reverse `back` and swap lists
- Recognize that the two-stack deque is amortized O(1) per operation (each element is reversed at most once)
- Compare mutable ring buffer vs persistent functional deque

## Rust Application

```rust
// Approach 1: VecDeque — idiomatic Rust
pub struct Deque<T> {
    inner: VecDeque<T>,
}
impl<T> Deque<T> {
    pub fn new() -> Self { Deque { inner: VecDeque::new() } }
    pub fn push_front(&mut self, x: T) { self.inner.push_front(x); }
    pub fn push_back(&mut self, x: T)  { self.inner.push_back(x); }
    pub fn pop_front(&mut self) -> Option<T> { self.inner.pop_front() }
    pub fn pop_back(&mut self) -> Option<T>  { self.inner.pop_back() }
    pub fn peek_front(&self) -> Option<&T> { self.inner.front() }
    pub fn peek_back(&self)  -> Option<&T> { self.inner.back() }
    pub fn size(&self) -> usize { self.inner.len() }
}

// Approach 2: Two-stack functional deque
pub struct FuncDeque<T> {
    front: Vec<T>,  // front elements (head = last element)
    back:  Vec<T>,  // back elements (head = last element)
}
impl<T> FuncDeque<T> {
    pub fn push_back(&mut self, x: T)  { self.back.push(x); }
    pub fn push_front(&mut self, x: T) { self.front.push(x); }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.front.is_empty() {
            // rebalance: reverse back into front
            self.front = self.back.drain(..).rev().collect();
        }
        self.front.pop()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.back.is_empty() {
            self.back = self.front.drain(..).rev().collect();
        }
        self.back.pop()
    }
}
```

`VecDeque` is a ring buffer: `head` and `tail` indices wrap around a fixed-capacity array, giving O(1) push/pop at both ends. It is the idiomatic choice for any deque in Rust.

The two-stack deque stores elements as two `Vec`s. `push_back` adds to `back`; `push_front` adds to `front`. `pop_front` pops from `front` (its last element is logically the front). When `front` is empty, `back` is drained in reverse into `front` — amortized O(1) since each element crosses the boundary at most once.

## OCaml Approach

```ocaml
(* Functional two-list deque *)
type 'a deque = { front: 'a list; back: 'a list }

let empty = { front = []; back = [] }

let push_back x d = { d with back = x :: d.back }
let push_front x d = { d with front = x :: d.front }

let pop_front d = match d.front with
  | x :: rest -> Some (x, { d with front = rest })
  | [] -> match List.rev d.back with
    | [] -> None
    | x :: rest -> Some (x, { front = rest; back = [] })

let pop_back d = match d.back with
  | x :: rest -> Some (x, { d with back = rest })
  | [] -> match List.rev d.front with
    | [] -> None
    | x :: rest -> Some (x, { back = rest; front = [] })
```

OCaml's functional deque is persistent — `pop_front` returns a new deque value rather than mutating. Structural sharing means `push_back` O(1) allocation. List reversal still occurs O(n) occasionally but amortizes to O(1).

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Standard deque | `VecDeque` — ring buffer, O(1) | No stdlib deque; `Queue` is FIFO only |
| Functional variant | Two `Vec`s, mutable | Two lists, immutable |
| Reversal | `drain(..).rev().collect()` | `List.rev` |
| Persistence | Mutable — no history | Persistent — old versions accessible |

`VecDeque` is the right default. The two-stack approach illuminates the functional deque structure but has higher constant factors due to occasional full-list reversal.

## Exercises

1. Implement `to_vec(&self) -> Vec<T>` for `FuncDeque` — collect elements front-to-back.
2. Implement a sliding window maximum using `VecDeque` as a monotone deque.
3. Implement the two-stack deque with `Rc<Vec<T>>` for structural sharing (persistent Rust version).
4. Benchmark `VecDeque` vs `FuncDeque` for 1,000,000 alternating push_back/pop_front operations.
5. Implement `rotate_left(n)` on `VecDeque` — move the first `n` elements to the back.
