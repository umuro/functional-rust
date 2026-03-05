//! 283. ExactSizeIterator for known-length
//!
//! `ExactSizeIterator` provides O(1) `len()`, enabling pre-allocation and size checks.

struct FixedRange {
    current: usize,
    end: usize,
}

impl FixedRange {
    fn new(start: usize, end: usize) -> Self {
        FixedRange { current: start, end }
    }
}

impl Iterator for FixedRange {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.current >= self.end {
            return None;
        }
        let v = self.current;
        self.current += 1;
        Some(v)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.current);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for FixedRange {
    fn len(&self) -> usize {
        self.end.saturating_sub(self.current)
    }
}

fn main() {
    // Built-in ExactSizeIterator: slice iter, range
    let arr = [1i32, 2, 3, 4, 5];
    let mut iter = arr.iter();
    println!("Slice iter len: {}", iter.len());
    iter.next();
    println!("After one next: {}", iter.len());

    let range = 0i32..10;
    println!("Range len: {}", range.len());

    // Custom ExactSizeIterator
    let fr = FixedRange::new(3, 8);
    println!("FixedRange len: {}", fr.len());
    let collected: Vec<usize> = FixedRange::new(3, 8).collect();
    println!("Collected: {:?}", collected);

    // Pre-allocate using len
    let source = vec![1i32, 2, 3, 4, 5];
    let mut dest = Vec::with_capacity(source.iter().len());
    dest.extend(source.iter().map(|&x| x * 2));
    println!("Pre-allocated doubled: {:?}", dest);

    // size_hint for Vec collection optimization
    let large: Vec<i32> = (0..1000).collect();
    println!("Large vec len: {}", large.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_exact_size() {
        let arr = [1i32, 2, 3, 4, 5];
        assert_eq!(arr.iter().len(), 5);
    }

    #[test]
    fn test_exact_size_after_next() {
        let arr = [1i32, 2, 3];
        let mut it = arr.iter();
        it.next();
        assert_eq!(it.len(), 2);
    }

    #[test]
    fn test_custom_fixed_range_len() {
        let fr = FixedRange::new(0, 5);
        assert_eq!(fr.len(), 5);
    }

    #[test]
    fn test_custom_fixed_range_collect() {
        let result: Vec<usize> = FixedRange::new(2, 5).collect();
        assert_eq!(result, vec![2, 3, 4]);
    }
}
