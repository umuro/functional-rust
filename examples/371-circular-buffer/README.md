📖 **[View on hightechmind.io →](https://hightechmind.io/rust/371-circular-buffer)**

---

# 371: Circular Buffer (Ring Buffer)

## Problem Statement

Audio processing, network packet buffering, logging with bounded history, and real-time telemetry all need fixed-size FIFO queues where old data is overwritten when the buffer is full. A circular buffer (ring buffer) achieves this with a fixed `Vec<Option<T>>` and two indices (`head`, `tail`) that wrap around modulo capacity. Operations are always O(1) with no allocation after construction. This is more efficient than `VecDeque` when the capacity is fixed at creation time — the modular arithmetic avoids the reallocation and copying that `VecDeque` might perform when growing.

## Learning Outcomes

- Implement a circular buffer with `head`, `tail`, `size`, and `capacity` fields
- Use modular arithmetic `(tail + 1) % capacity` for index wrapping
- Track fullness with a separate `size` counter (avoids the "full vs empty" ambiguity)
- Implement `push` returning `Err` when full and `pop` returning `None` when empty
- Add `push_overwrite` that drops the oldest element when full
- Understand why circular buffers are used in audio and network I/O

## Rust Application

```rust
pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            data: (0..capacity).map(|_| None).collect(),
            head: 0, tail: 0, size: 0, capacity,
        }
    }

    pub fn push(&mut self, val: T) -> Result<(), &'static str> {
        if self.is_full() { return Err("buffer full"); }
        self.data[self.tail] = Some(val);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let val = self.data[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        val
    }

    pub fn push_overwrite(&mut self, val: T) {
        if self.is_full() {
            self.head = (self.head + 1) % self.capacity; // discard oldest
            self.size -= 1;
        }
        self.data[self.tail] = Some(val);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
    }

    pub fn is_full(&self) -> bool { self.size == self.capacity }
    pub fn is_empty(&self) -> bool { self.size == 0 }
    pub fn len(&self) -> usize { self.size }
}
```

The `size` counter avoids the classic ring buffer ambiguity: `head == tail` is ambiguous between "empty" and "full" without it. Alternative: reserve one slot (capacity - 1 usable) and use `(tail + 1) % capacity == head` for full — but tracking `size` is cleaner.

## OCaml Approach

```ocaml
type 'a cbuf = {
  data: 'a option array;
  mutable head: int;
  mutable tail: int;
  mutable size: int;
  capacity: int;
}

let make capacity =
  { data = Array.make capacity None; head = 0; tail = 0; size = 0; capacity }

let push buf v =
  if buf.size = buf.capacity then Error "full"
  else begin
    buf.data.(buf.tail) <- Some v;
    buf.tail <- (buf.tail + 1) mod buf.capacity;
    buf.size <- buf.size + 1;
    Ok ()
  end

let pop buf =
  if buf.size = 0 then None
  else begin
    let v = buf.data.(buf.head) in
    buf.data.(buf.head) <- None;
    buf.head <- (buf.head + 1) mod buf.capacity;
    buf.size <- buf.size - 1;
    v
  end
```

The algorithm is identical — circular buffers are inherently imperative, mapping cleanly to both languages' mutable array operations.

## Key Differences

| Aspect | Rust `CircularBuffer<T>` | OCaml `'a cbuf` |
|--------|-------------------------|-----------------|
| Storage | `Vec<Option<T>>` | `'a option array` |
| Index wrap | `% capacity` (modulo) | `mod capacity` |
| Full detection | `size == capacity` | `size = capacity` |
| Push error | `Result<(), &str>` | `Result` variant or exception |
| Production | `VecDeque` or `ringbuf` crate | `Queue` module (unbounded) |

## Exercises

1. **Overwrite mode audio**: Implement a `AudioBuffer` with `push_overwrite` that holds the last 4096 samples; show that oldest samples are silently discarded when the buffer is full.
2. **Peek without pop**: Add `peek(&self) -> Option<&T>` that returns a reference to the head element without removing it; useful for lookahead parsing.
3. **Iterator**: Implement `IntoIterator` for `CircularBuffer<T>` that drains elements from head to tail in O(1) per step (no reallocation), consuming the buffer.
