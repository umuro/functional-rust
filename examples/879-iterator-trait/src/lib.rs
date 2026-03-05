// Example 085: Iterator Trait
// OCaml Seq → Rust Iterator

// === Approach 1: Implementing Iterator for a custom type ===
struct Range {
    current: i32,
    end_: i32,
}

impl Range {
    fn new(start: i32, end_: i32) -> Self {
        Range { current: start, end_ }
    }
}

impl Iterator for Range {
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

// === Approach 2: Iterator with map/filter (free from trait) ===
struct Counter {
    current: u64,
}

impl Counter {
    fn from(start: u64) -> Self {
        Counter { current: start }
    }
}

impl Iterator for Counter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current;
        self.current += 1;
        Some(val) // infinite!
    }
}

// === Approach 3: Iterator via IntoIterator ===
struct Repeat<T: Clone> {
    value: T,
    remaining: usize,
}

impl<T: Clone> Repeat<T> {
    fn new(value: T, count: usize) -> Self {
        Repeat { value, remaining: count }
    }
}

impl<T: Clone> Iterator for Repeat<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(self.value.clone())
        }
    }
}

// Generic function working with any iterator
fn sum_first_n<I: Iterator<Item = i32>>(iter: I, n: usize) -> i32 {
    iter.take(n).sum()
}

fn collect_mapped<I, F, B>(iter: I, f: F) -> Vec<B>
where
    I: Iterator,
    F: Fn(I::Item) -> B,
{
    iter.map(f).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let v: Vec<i32> = Range::new(1, 6).collect();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_empty_range() {
        let v: Vec<i32> = Range::new(5, 5).collect();
        assert!(v.is_empty());
    }

    #[test]
    fn test_range_map() {
        let v: Vec<i32> = Range::new(1, 4).map(|x| x * 10).collect();
        assert_eq!(v, vec![10, 20, 30]);
    }

    #[test]
    fn test_range_filter() {
        let v: Vec<i32> = Range::new(1, 11).filter(|x| x % 3 == 0).collect();
        assert_eq!(v, vec![3, 6, 9]);
    }

    #[test]
    fn test_counter_take() {
        let v: Vec<u64> = Counter::from(10).take(3).collect();
        assert_eq!(v, vec![10, 11, 12]);
    }

    #[test]
    fn test_counter_chain() {
        let sum: u64 = Counter::from(1).take(100).sum();
        assert_eq!(sum, 5050);
    }

    #[test]
    fn test_repeat() {
        let v: Vec<i32> = Repeat::new(42, 4).collect();
        assert_eq!(v, vec![42, 42, 42, 42]);
    }

    #[test]
    fn test_sum_first_n() {
        let sum = sum_first_n(Range::new(1, 100), 10);
        assert_eq!(sum, 55);
    }

    #[test]
    fn test_collect_mapped() {
        let v = collect_mapped(Range::new(1, 4), |x| x.to_string());
        assert_eq!(v, vec!["1", "2", "3"]);
    }
}
