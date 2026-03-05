// 086: Custom Iterator with State

// Approach 1: Counter iterator
struct Counter {
    current: i32,
    step: i32,
}

impl Counter {
    fn new(start: i32, step: i32) -> Self {
        Counter { current: start, step }
    }
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        self.current += self.step;
        Some(self.current) // infinite
    }
}

// Approach 2: Fibonacci iterator
struct Fib { a: u64, b: u64 }

impl Fib {
    fn new() -> Self { Fib { a: 0, b: 1 } }
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let val = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(val)
    }
}

// Approach 3: Collatz sequence (finite)
struct Collatz { n: u64, done_: bool }

impl Collatz {
    fn new(start: u64) -> Self { Collatz { n: start, done_: false } }
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.done_ { return None; }
        let val = self.n;
        if self.n == 1 {
            self.done_ = true;
        } else if self.n % 2 == 0 {
            self.n /= 2;
        } else {
            self.n = 3 * self.n + 1;
        }
        Some(val)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let v: Vec<i32> = Counter::new(0, 2).take(3).collect();
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_counter_negative() {
        let v: Vec<i32> = Counter::new(10, -3).take(4).collect();
        assert_eq!(v, vec![7, 4, 1, -2]);
    }

    #[test]
    fn test_fibonacci() {
        let fibs: Vec<u64> = Fib::new().take(8).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_collatz() {
        let v: Vec<u64> = Collatz::new(6).collect();
        assert_eq!(v, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_collatz_one() {
        let v: Vec<u64> = Collatz::new(1).collect();
        assert_eq!(v, vec![1]);
    }
}
