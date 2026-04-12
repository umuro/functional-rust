**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[circular-buffer on hightechmind.io](https://hightechmind.io/posts/functional-rust/circular-buffer)

---

## Problem Statement

Implement a fixed-capacity circular ring buffer (FIFO) where pushing to a full buffer overwrites the oldest element. Use separate `head`, `tail`, and `count` indices with modular arithmetic. Unlike `VecDeque` (which grows), this ring buffer has a fixed capacity — modeling hardware FIFOs, event log windows, and streaming buffers.

## Learning Outcomes

- Implement `RingBuffer<T: Default + Clone>` with `Vec<T>` and `head`/`tail`/`count` tracking
- Push: write to `tail`, advance `tail = (tail + 1) % capacity`, and when full advance `head` to overwrite oldest
- Pop: read from `head`, advance `head = (head + 1) % capacity`, decrement `count`
- Implement `is_full`, `is_empty`, `size`, and `peek` without special-casing the wrap-around
- Understand why `T: Default` is needed for pre-allocation: `vec![T::default(); capacity]`

## Rust Application

```rust
pub struct RingBuffer<T> {
    data: Vec<T>,
    capacity: usize,
    head: usize,   // next read position
    tail: usize,   // next write position
    count: usize,
}

impl<T: Default + Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        RingBuffer {
            data: vec![T::default(); capacity],
            capacity, head: 0, tail: 0, count: 0,
        }
    }
}

impl<T: Clone> RingBuffer<T> {
    pub fn push(&mut self, x: T) {
        self.data[self.tail] = x;
        self.tail = (self.tail + 1) % self.capacity;
        if self.count == self.capacity {
            self.head = (self.head + 1) % self.capacity;  // overwrite oldest
        } else {
            self.count += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 { return None; }
        let x = self.data[self.head].clone();
        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;
        Some(x)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.count == 0 { None } else { Some(&self.data[self.head]) }
    }
}
```

The `% capacity` modular arithmetic wraps both `head` and `tail` around the fixed-size array. When `push` is called on a full buffer, `head` advances (overwriting the oldest entry). No bounds check is needed beyond `% capacity`.

`T: Default + Clone` is only needed for `new` (to fill the initial `Vec`). The `push`/`pop` methods only need `T: Clone` for `pop`'s return value. Splitting the `impl` blocks makes this explicit.

## OCaml Approach

```ocaml
type 'a ring_buffer = {
  data: 'a array;
  capacity: int;
  mutable head: int;
  mutable tail: int;
  mutable count: int;
}

let create capacity default =
  { data = Array.make capacity default;
    capacity; head = 0; tail = 0; count = 0 }

let push buf x =
  buf.data.(buf.tail) <- x;
  buf.tail <- (buf.tail + 1) mod buf.capacity;
  if buf.count = buf.capacity
  then buf.head <- (buf.head + 1) mod buf.capacity
  else buf.count <- buf.count + 1

let pop buf =
  if buf.count = 0 then None
  else begin
    let x = buf.data.(buf.head) in
    buf.head <- (buf.head + 1) mod buf.capacity;
    buf.count <- buf.count - 1;
    Some x
  end
```

OCaml's `mutable` record fields allow in-place update. The algorithm is identical: `% capacity` wraps both indices, and `push` on a full buffer advances `head` to drop the oldest entry.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Pre-allocation | `vec![T::default(); capacity]` | `Array.make capacity default` (explicit default) |
| Mutable fields | `let mut` struct fields | `mutable` record fields |
| Modular wrap | `% capacity` | `mod capacity` |
| Return on pop | `Option<T>` via `Some(x.clone())` | `'a option` via `Some x` |

The circular buffer is a fixed-size sliding window over a stream of values. It is the foundation of audio/video buffers, network packet queues, and producer-consumer pipelines where only the most recent `n` items matter.

## Exercises

1. Implement `to_vec(&self) -> Vec<T>` that returns elements in FIFO order (oldest first).
2. Implement `iter(&self) -> impl Iterator<Item=&T>` that traverses from `head` to `tail`.
3. Add a `push_no_overwrite` variant that returns `Err` instead of overwriting when full.
4. Implement a sliding-window average: push each new sample, then compute the mean of all current elements.
5. Implement a thread-safe ring buffer using `Mutex<RingBuffer<T>>` and benchmark it vs `crossbeam`'s bounded channel.
