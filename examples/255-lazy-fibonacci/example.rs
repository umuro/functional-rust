// Example 255: Lazy Fibonacci — Infinite stream using closures/iterators

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — Iterator
// ---------------------------------------------------------------------------

pub struct FibIter {
    a: u64,
    b: u64,
}

impl FibIter {
    pub fn new(a: u64, b: u64) -> Self {
        Self { a, b }
    }
}

impl Iterator for FibIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let value = self.a;
        let next_b = self.a + self.b;
        self.a = self.b;
        self.b = next_b;
        Some(value)
    }
}

pub fn fibs_take(n: usize) -> Vec<u64> {
    FibIter::new(0, 1).take(n).collect()
}

// ---------------------------------------------------------------------------
// Solution 2: Thunk-based stream — mirrors OCaml's stream type
//
//   OCaml: type 'a stream = Cons of 'a * (unit -> 'a stream)
// ---------------------------------------------------------------------------

pub struct Stream<T> {
    pub head: T,
    tail: Box<dyn Fn() -> Stream<T>>,
}

impl<T: Copy> Stream<T> {
    pub fn take(&self, n: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(n);
        if n == 0 {
            return result;
        }
        result.push(self.head);
        let mut next = (self.tail)();
        for _ in 1..n {
            result.push(next.head);
            next = (next.tail)();
        }
        result
    }
}

pub fn fibs_stream(a: u64, b: u64) -> Stream<u64> {
    Stream {
        head: a,
        tail: Box::new(move || fibs_stream(b, a + b)),
    }
}

// ---------------------------------------------------------------------------

fn main() {
    // Iterator approach
    let first_ten: Vec<u64> = FibIter::new(0, 1).take(10).collect();
    println!("Iterator — first 10 fibs: {:?}", first_ten);

    // Convenience helper
    println!("fibs_take(5)             : {:?}", fibs_take(5));

    // Stream (thunk) approach
    let stream = fibs_stream(0, 1);
    println!("Stream   — first 10 fibs: {:?}", stream.take(10));

    // Composing with other Iterator adapters
    let evens: Vec<u64> = FibIter::new(0, 1)
        .filter(|x| x % 2 == 0)
        .take(5)
        .collect();
    println!("Even fibs (first 5)      : {:?}", evens);
}

/* Output:
   Iterator — first 10 fibs: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
   fibs_take(5)             : [0, 1, 1, 2, 3]
   Stream   — first 10 fibs: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
   Even fibs (first 5)      : [0, 2, 8, 34, 144]
*/
