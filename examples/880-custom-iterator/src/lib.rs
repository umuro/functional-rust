// Example 086: Custom Iterator — Fibonacci and Range with Step

// === Approach 1: Fibonacci iterator ===
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let val = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(val)
    }
}

// === Approach 2: Range with step ===
struct StepRange<T> {
    current: T,
    end_: T,
    step: T,
}

impl StepRange<i64> {
    fn new(start: i64, end_: i64, step: i64) -> Self {
        StepRange { current: start, end_, step }
    }
}

impl Iterator for StepRange<i64> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.current >= self.end_ {
            None
        } else {
            let val = self.current;
            self.current += self.step;
            Some(val)
        }
    }
}

impl StepRange<f64> {
    fn new_float(start: f64, end_: f64, step: f64) -> Self {
        StepRange { current: start, end_, step }
    }
}

impl Iterator for StepRange<f64> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.current >= self.end_ {
            None
        } else {
            let val = self.current;
            self.current += self.step;
            Some(val)
        }
    }
}

// === Approach 3: Collatz sequence iterator ===
struct Collatz {
    current: u64,
    done_: bool,
}

impl Collatz {
    fn new(n: u64) -> Self {
        Collatz { current: n, done_: false }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.done_ {
            return None;
        }
        let val = self.current;
        if val == 1 {
            self.done_ = true;
        } else if val % 2 == 0 {
            self.current = val / 2;
        } else {
            self.current = 3 * val + 1;
        }
        Some(val)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_fibonacci_sum() {
        let sum: u64 = Fibonacci::new().take(10).sum();
        assert_eq!(sum, 88);
    }

    #[test]
    fn test_step_range() {
        let v: Vec<i64> = StepRange::new(0, 10, 2).collect();
        assert_eq!(v, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_step_range_by3() {
        let v: Vec<i64> = StepRange::new(0, 10, 3).collect();
        assert_eq!(v, vec![0, 3, 6, 9]);
    }

    #[test]
    fn test_step_range_float() {
        let v: Vec<f64> = StepRange::new_float(0.0, 1.0, 0.25).collect();
        assert_eq!(v.len(), 4);
        assert!((v[0] - 0.0).abs() < 1e-10);
        assert!((v[3] - 0.75).abs() < 1e-10);
    }

    #[test]
    fn test_collatz_6() {
        let v: Vec<u64> = Collatz::new(6).collect();
        assert_eq!(v, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_collatz_1() {
        let v: Vec<u64> = Collatz::new(1).collect();
        assert_eq!(v, vec![1]);
    }

    #[test]
    fn test_collatz_length() {
        let len = Collatz::new(27).count();
        assert_eq!(len, 112);
    }

    #[test]
    fn test_empty_range() {
        let v: Vec<i64> = StepRange::new(10, 5, 1).collect();
        assert!(v.is_empty());
    }
}
