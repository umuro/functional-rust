// 1045: Small Vector Optimization Concept
// Stack up to N elements, heap beyond — like SmallVec (concept, std only)

/// SmallVec: stores up to N elements on the stack, spills to heap
enum SmallVec<T, const N: usize> {
    Inline {
        data: [Option<T>; N], // Using Option since we can't use MaybeUninit safely
        len: usize,
    },
    Heap(Vec<T>),
}

impl<T: Clone + Default, const N: usize> SmallVec<T, N> {
    fn new() -> Self {
        SmallVec::Inline {
            data: std::array::from_fn(|_| None),
            len: 0,
        }
    }

    fn push(&mut self, value: T) {
        match self {
            SmallVec::Inline { data, len } if *len < N => {
                data[*len] = Some(value);
                *len += 1;
            }
            SmallVec::Inline { data, len } => {
                // Spill to heap
                let mut vec = Vec::with_capacity(*len + 1);
                for item in data.iter_mut().take(*len) {
                    if let Some(val) = item.take() {
                        vec.push(val);
                    }
                }
                vec.push(value);
                *self = SmallVec::Heap(vec);
            }
            SmallVec::Heap(vec) => {
                vec.push(value);
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            SmallVec::Inline { len, .. } => *len,
            SmallVec::Heap(vec) => vec.len(),
        }
    }

    fn is_inline(&self) -> bool {
        matches!(self, SmallVec::Inline { .. })
    }

    fn get(&self, index: usize) -> Option<&T> {
        match self {
            SmallVec::Inline { data, len } => {
                if index < *len {
                    data[index].as_ref()
                } else {
                    None
                }
            }
            SmallVec::Heap(vec) => vec.get(index),
        }
    }

    fn to_vec(&self) -> Vec<T> {
        match self {
            SmallVec::Inline { data, len } => data
                .iter()
                .take(*len)
                .filter_map(|x| x.as_ref().cloned())
                .collect(),
            SmallVec::Heap(vec) => vec.clone(),
        }
    }
}

fn basic_small_vec() {
    let mut sv: SmallVec<i32, 4> = SmallVec::new();
    sv.push(1);
    sv.push(2);
    sv.push(3);

    assert_eq!(sv.len(), 3);
    assert!(sv.is_inline()); // Still on stack
    assert_eq!(sv.to_vec(), vec![1, 2, 3]);

    sv.push(4); // At capacity, still inline
    assert!(sv.is_inline());

    sv.push(5); // Spills to heap
    assert!(!sv.is_inline());
    assert_eq!(sv.to_vec(), vec![1, 2, 3, 4, 5]);
}

fn indexed_access() {
    let mut sv: SmallVec<&str, 3> = SmallVec::new();
    sv.push("hello");
    sv.push("world");

    assert_eq!(sv.get(0), Some(&"hello"));
    assert_eq!(sv.get(1), Some(&"world"));
    assert_eq!(sv.get(2), None);
}

fn spill_behavior() {
    let mut sv: SmallVec<i32, 2> = SmallVec::new();
    sv.push(10);
    sv.push(20);
    assert!(sv.is_inline());

    sv.push(30); // Spills
    assert!(!sv.is_inline());
    assert_eq!(sv.len(), 3);
    assert_eq!(sv.get(2), Some(&30));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        basic_small_vec();
    }

    #[test]
    fn test_indexed() {
        indexed_access();
    }

    #[test]
    fn test_spill() {
        spill_behavior();
    }

    #[test]
    fn test_empty() {
        let sv: SmallVec<i32, 4> = SmallVec::new();
        assert_eq!(sv.len(), 0);
        assert!(sv.is_inline());
        assert_eq!(sv.get(0), None);
    }

    #[test]
    fn test_large_n() {
        let mut sv: SmallVec<i32, 16> = SmallVec::new();
        for i in 0..16 {
            sv.push(i);
        }
        assert!(sv.is_inline());
        sv.push(16);
        assert!(!sv.is_inline());
    }
}
