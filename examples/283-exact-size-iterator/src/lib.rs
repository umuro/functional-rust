//! # ExactSizeIterator for Known-Length Iterators
//!
//! `ExactSizeIterator` provides O(1) `len()`, enabling pre-allocation and size checks.

/// A range iterator that knows its exact size
pub struct FixedRange {
    current: usize,
    end: usize,
}

impl FixedRange {
    pub fn new(start: usize, end: usize) -> Self {
        FixedRange {
            current: start,
            end,
        }
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

/// Pre-allocate and transform using ExactSizeIterator
pub fn double_elements<I>(iter: I) -> Vec<i32>
where
    I: ExactSizeIterator<Item = i32>,
{
    let mut result = Vec::with_capacity(iter.len());
    result.extend(iter.map(|x| x * 2));
    result
}

/// Alternative: using collect (also uses size_hint internally)
pub fn square_elements(slice: &[i32]) -> Vec<i32> {
    slice.iter().map(|&x| x * x).collect()
}

/// Efficient single-allocation append
pub fn append_transformed<I, F>(vec: &mut Vec<i32>, iter: I, f: F)
where
    I: ExactSizeIterator<Item = i32>,
    F: Fn(i32) -> i32,
{
    vec.reserve(iter.len());
    for x in iter {
        vec.push(f(x));
    }
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

    #[test]
    fn test_fixed_range_size_hint() {
        let fr = FixedRange::new(10, 20);
        assert_eq!(fr.size_hint(), (10, Some(10)));
    }

    #[test]
    fn test_double_elements_preallocated() {
        let arr = [1i32, 2, 3, 4, 5];
        let result = double_elements(arr.iter().copied());
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_square_elements() {
        let arr = [1, 2, 3, 4];
        let result = square_elements(&arr);
        assert_eq!(result, vec![1, 4, 9, 16]);
    }

    #[test]
    fn test_append_transformed() {
        let mut vec = vec![1, 2];
        let arr = [3i32, 4, 5];
        append_transformed(&mut vec, arr.iter().copied(), |x| x * 10);
        assert_eq!(vec, vec![1, 2, 30, 40, 50]);
    }
}
