// 1044: Sorted Vec — Binary Search Insert with partition_point
// Maintain a sorted Vec with O(log n) search and O(n) insert

/// A sorted vector that maintains order on insertion
struct SortedVec<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    fn new() -> Self {
        SortedVec { data: Vec::new() }
    }

    /// Insert maintaining sorted order — O(log n) search + O(n) shift
    fn insert(&mut self, value: T) {
        let pos = self.data.partition_point(|x| x < &value);
        self.data.insert(pos, value);
    }

    /// Binary search — O(log n)
    fn contains(&self, value: &T) -> bool {
        self.data.binary_search(value).is_ok()
    }

    /// Find index of value — O(log n)
    fn find(&self, value: &T) -> Option<usize> {
        self.data.binary_search(value).ok()
    }

    /// Remove a value — O(log n) search + O(n) shift
    fn remove(&mut self, value: &T) -> bool {
        if let Ok(pos) = self.data.binary_search(value) {
            self.data.remove(pos);
            true
        } else {
            false
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Range query: elements in [lo, hi]
    fn range(&self, lo: &T, hi: &T) -> &[T] {
        let start = self.data.partition_point(|x| x < lo);
        let end = self.data.partition_point(|x| x <= hi);
        &self.data[start..end]
    }
}

fn basic_sorted_vec() {
    let mut sv = SortedVec::new();
    sv.insert(5);
    sv.insert(3);
    sv.insert(7);
    sv.insert(1);
    sv.insert(4);

    assert_eq!(sv.as_slice(), &[1, 3, 4, 5, 7]);
    assert!(sv.contains(&4));
    assert!(!sv.contains(&6));
    assert_eq!(sv.find(&5), Some(3));
}

fn partition_point_demo() {
    let data = vec![1, 3, 5, 7, 9, 11];

    // partition_point: first index where predicate is false
    let pos = data.partition_point(|&x| x < 6);
    assert_eq!(pos, 3); // Insert point for 6

    let pos = data.partition_point(|&x| x < 5);
    assert_eq!(pos, 2); // 5 would go at index 2

    // binary_search returns Ok(index) or Err(insert_point)
    assert_eq!(data.binary_search(&5), Ok(2));
    assert_eq!(data.binary_search(&6), Err(3));
}

fn range_query() {
    let mut sv = SortedVec::new();
    for x in [1, 3, 5, 7, 9, 11] {
        sv.insert(x);
    }

    assert_eq!(sv.range(&3, &9), &[3, 5, 7, 9]);
    assert_eq!(sv.range(&4, &8), &[5, 7]);
    assert_eq!(sv.range(&20, &30), &[] as &[i32]);
}

fn remove_test() {
    let mut sv = SortedVec::new();
    for x in [1, 2, 3, 4, 5] {
        sv.insert(x);
    }

    assert!(sv.remove(&3));
    assert_eq!(sv.as_slice(), &[1, 2, 4, 5]);
    assert!(!sv.remove(&3)); // Already removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        basic_sorted_vec();
    }

    #[test]
    fn test_partition_point() {
        partition_point_demo();
    }

    #[test]
    fn test_range() {
        range_query();
    }

    #[test]
    fn test_remove() {
        remove_test();
    }

    #[test]
    fn test_duplicates() {
        let mut sv = SortedVec::new();
        sv.insert(1);
        sv.insert(1);
        sv.insert(2);
        assert_eq!(sv.as_slice(), &[1, 1, 2]);
    }
}
