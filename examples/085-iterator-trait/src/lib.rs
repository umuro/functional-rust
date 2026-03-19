#![allow(clippy::all)]
// 085: Iterator Trait — implement from scratch

// Approach 1: Range iterator
struct MyRange {
    current: i32,
    end_: i32,
}

impl MyRange {
    fn new(start: i32, end_: i32) -> Self {
        MyRange {
            current: start,
            end_,
        }
    }
}

impl Iterator for MyRange {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end_ {
            None
        } else {
            let val = self.current;
            self.current += 1;
            Some(val)
        }
    }
}

// Approach 2: Fibonacci iterator
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
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(val) // infinite iterator
    }
}

// Approach 3: Use free methods from implementing next()
fn demo_free_methods() -> Vec<i32> {
    MyRange::new(0, 10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let v: Vec<i32> = MyRange::new(0, 5).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_empty_range() {
        let v: Vec<i32> = MyRange::new(5, 5).collect();
        assert!(v.is_empty());
    }

    #[test]
    fn test_fibonacci() {
        let fibs: Vec<u64> = Fibonacci::new().take(8).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_free_methods() {
        assert_eq!(demo_free_methods(), vec![0, 4, 16, 36, 64]);
    }

    #[test]
    fn test_sum() {
        assert_eq!(MyRange::new(1, 6).sum::<i32>(), 15);
    }

    #[test]
    fn test_filter_map() {
        let v: Vec<i32> = MyRange::new(1, 4).map(|x| x * 2).collect();
        assert_eq!(v, vec![2, 4, 6]);
    }
}
