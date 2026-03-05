// 775. Fixed-Size Arrays with const N Parameter
// Ring buffer, fixed stack, sliding-window stats — all stack-allocated

// ── Ring buffer with const capacity ───────────────────────────────────────────

#[derive(Debug)]
pub struct RingBuffer<T: Copy + Default, const N: usize> {
    data: [T; N],
    head: usize,
    count: usize,
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self { data: [T::default(); N], head: 0, count: 0 }
    }

    pub fn push(&mut self, val: T) {
        let tail = (self.head + self.count) % N;
        self.data[tail] = val;
        if self.count < N {
            self.count += 1;
        } else {
            self.head = (self.head + 1) % N;
        }
    }

    pub fn len(&self) -> usize { self.count }
    pub fn capacity(&self) -> usize { N }
    pub fn is_full(&self) -> bool { self.count == N }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.count).map(|i| &self.data[(self.head + i) % N])
    }

    pub fn to_array(&self) -> [T; N] where T: Default {
        let mut out = [T::default(); N];
        for (i, v) in self.iter().enumerate() {
            out[i] = *v;
        }
        out
    }
}

// ── Fixed-capacity stack ───────────────────────────────────────────────────────

#[derive(Debug)]
pub struct FixedStack<T: Copy + Default, const N: usize> {
    data: [T; N],
    top: usize,
}

impl<T: Copy + Default, const N: usize> FixedStack<T, N> {
    pub fn new() -> Self { Self { data: [T::default(); N], top: 0 } }
    pub fn push(&mut self, v: T) -> bool {
        if self.top >= N { return false; }
        self.data[self.top] = v;
        self.top += 1;
        true
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 { return None; }
        self.top -= 1;
        Some(self.data[self.top])
    }
    pub fn peek(&self) -> Option<T> {
        if self.top == 0 { None } else { Some(self.data[self.top - 1]) }
    }
    pub fn len(&self) -> usize { self.top }
    pub fn is_empty(&self) -> bool { self.top == 0 }
}

// ── Sliding-window stats ───────────────────────────────────────────────────────

pub struct SlidingStats<const W: usize> {
    buf: RingBuffer<f64, W>,
}

impl<const W: usize> SlidingStats<W> {
    pub fn new() -> Self { Self { buf: RingBuffer::new() } }

    pub fn push(&mut self, v: f64) { self.buf.push(v); }

    pub fn mean(&self) -> f64 {
        if self.buf.len() == 0 { return 0.0; }
        let sum: f64 = self.buf.iter().sum();
        sum / self.buf.len() as f64
    }

    pub fn max(&self) -> Option<f64> {
        self.buf.iter().cloned().reduce(f64::max)
    }
}

fn main() {
    // Ring buffer
    let mut ring: RingBuffer<i32, 5> = RingBuffer::new();
    for i in 1..=7 { ring.push(i); }
    print!("Ring (cap=5, pushed 1..7): ");
    for v in ring.iter() { print!("{v} "); }
    println!("← only last 5");

    // Fixed stack
    let mut stack: FixedStack<&str, 4> = FixedStack::new();
    stack.push("a"); stack.push("b"); stack.push("c");
    println!("\nStack peek: {:?}", stack.peek());
    println!("Stack pop : {:?}", stack.pop());
    println!("Stack len : {}", stack.len());

    // Sliding stats with window=3
    let mut stats: SlidingStats<3> = SlidingStats::new();
    let data = [1.0, 5.0, 3.0, 7.0, 2.0];
    for (i, &v) in data.iter().enumerate() {
        stats.push(v);
        println!("After pushing {v}: mean={:.2}, max={:?}", stats.mean(), stats.max());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_wraps_correctly() {
        let mut r: RingBuffer<i32, 3> = RingBuffer::new();
        for i in 1..=5 { r.push(i); }
        let v: Vec<i32> = r.iter().cloned().collect();
        assert_eq!(v, vec![3, 4, 5]); // last 3
    }

    #[test]
    fn ring_capacity_full() {
        let mut r: RingBuffer<i32, 4> = RingBuffer::new();
        for i in 0..4 { r.push(i); }
        assert!(r.is_full());
    }

    #[test]
    fn stack_overflow_returns_false() {
        let mut s: FixedStack<i32, 2> = FixedStack::new();
        assert!(s.push(1));
        assert!(s.push(2));
        assert!(!s.push(3)); // overflow
    }

    #[test]
    fn stack_underflow_returns_none() {
        let mut s: FixedStack<i32, 2> = FixedStack::new();
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn sliding_mean() {
        let mut st: SlidingStats<3> = SlidingStats::new();
        st.push(1.0); st.push(2.0); st.push(3.0); st.push(4.0);
        // window = [2, 3, 4]
        assert!((st.mean() - 3.0).abs() < 1e-10);
    }
}
