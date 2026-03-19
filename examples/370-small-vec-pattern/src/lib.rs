//! SmallVec Pattern
//!
//! Store small collections inline, spill to heap when needed.

/// SmallVec: inline for ≤N items, heap for more
#[derive(Debug, Clone)]
pub enum SmallVec<T, const N: usize> {
    Inline { data: [Option<T>; N], len: usize },
    Heap(Vec<T>),
}

impl<T: Clone + Default, const N: usize> SmallVec<T, N> {
    /// Create a new empty SmallVec
    pub fn new() -> Self {
        Self::Inline {
            data: std::array::from_fn(|_| None),
            len: 0,
        }
    }

    /// Push an element
    pub fn push(&mut self, val: T) {
        match self {
            Self::Inline { data, len } if *len < N => {
                data[*len] = Some(val);
                *len += 1;
            }
            Self::Inline { data, len } => {
                // Overflow: move to heap
                let mut v: Vec<T> = data[..*len].iter_mut().filter_map(|x| x.take()).collect();
                v.push(val);
                *self = Self::Heap(v);
            }
            Self::Heap(v) => v.push(val),
        }
    }

    /// Get element by index
    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            Self::Inline { data, len } => {
                if i < *len {
                    data[i].as_ref()
                } else {
                    None
                }
            }
            Self::Heap(v) => v.get(i),
        }
    }

    /// Get mutable element by index
    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        match self {
            Self::Inline { data, len } => {
                if i < *len {
                    data[i].as_mut()
                } else {
                    None
                }
            }
            Self::Heap(v) => v.get_mut(i),
        }
    }

    /// Get length
    pub fn len(&self) -> usize {
        match self {
            Self::Inline { len, .. } => *len,
            Self::Heap(v) => v.len(),
        }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if stored inline (no heap allocation)
    pub fn is_inline(&self) -> bool {
        matches!(self, Self::Inline { .. })
    }

    /// Pop last element
    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::Inline { data, len } if *len > 0 => {
                *len -= 1;
                data[*len].take()
            }
            Self::Inline { .. } => None,
            Self::Heap(v) => v.pop(),
        }
    }

    /// Clear all elements
    pub fn clear(&mut self) {
        match self {
            Self::Inline { data, len } => {
                for item in data.iter_mut().take(*len) {
                    *item = None;
                }
                *len = 0;
            }
            Self::Heap(v) => v.clear(),
        }
    }

    /// Convert to Vec
    pub fn to_vec(&self) -> Vec<T> {
        match self {
            Self::Inline { data, len } => data[..*len].iter().filter_map(|x| x.clone()).collect(),
            Self::Heap(v) => v.clone(),
        }
    }

    /// Iterate over elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            Self::Inline { data, len } => data[..*len]
                .iter()
                .filter_map(|x| x.as_ref())
                .collect::<Vec<_>>()
                .into_iter(),
            Self::Heap(v) => v.iter().collect::<Vec<_>>().into_iter(),
        }
    }
}

impl<T: Clone + Default, const N: usize> Default for SmallVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Default, const N: usize> FromIterator<T> for SmallVec<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut sv = Self::new();
        for item in iter {
            sv.push(item);
        }
        sv
    }
}

/// Specialized SmallVec for common sizes
pub type SmallVec4<T> = SmallVec<T, 4>;
pub type SmallVec8<T> = SmallVec<T, 8>;
pub type SmallVec16<T> = SmallVec<T, 16>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_small() {
        let mut sv: SmallVec<i32, 4> = SmallVec::new();
        for i in 0..4 {
            sv.push(i);
        }
        assert!(sv.is_inline());
        assert_eq!(sv.len(), 4);
    }

    #[test]
    fn test_heap_on_overflow() {
        let mut sv: SmallVec<i32, 2> = SmallVec::new();
        sv.push(1);
        sv.push(2);
        assert!(sv.is_inline());
        sv.push(3);
        assert!(!sv.is_inline());
        assert_eq!(sv.len(), 3);
    }

    #[test]
    fn test_get_items() {
        let mut sv: SmallVec<i32, 4> = SmallVec::new();
        for i in 0..6 {
            sv.push(i);
        }
        assert_eq!(sv.get(0), Some(&0));
        assert_eq!(sv.get(5), Some(&5));
        assert_eq!(sv.get(6), None);
    }

    #[test]
    fn test_pop() {
        let mut sv: SmallVec<i32, 4> = SmallVec::new();
        sv.push(1);
        sv.push(2);
        assert_eq!(sv.pop(), Some(2));
        assert_eq!(sv.pop(), Some(1));
        assert_eq!(sv.pop(), None);
    }

    #[test]
    fn test_clear() {
        let mut sv: SmallVec<i32, 4> = SmallVec::new();
        sv.push(1);
        sv.push(2);
        sv.clear();
        assert!(sv.is_empty());
    }

    #[test]
    fn test_to_vec() {
        let mut sv: SmallVec<i32, 4> = SmallVec::new();
        sv.push(1);
        sv.push(2);
        sv.push(3);
        assert_eq!(sv.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_from_iterator() {
        let sv: SmallVec<i32, 4> = vec![1, 2, 3].into_iter().collect();
        assert!(sv.is_inline());
        assert_eq!(sv.len(), 3);

        let sv2: SmallVec<i32, 2> = vec![1, 2, 3, 4, 5].into_iter().collect();
        assert!(!sv2.is_inline());
        assert_eq!(sv2.len(), 5);
    }

    #[test]
    fn test_get_mut() {
        let mut sv: SmallVec<i32, 4> = SmallVec::new();
        sv.push(1);
        *sv.get_mut(0).unwrap() = 10;
        assert_eq!(sv.get(0), Some(&10));
    }
}
