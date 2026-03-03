/// Difference List — O(1) Append
///
/// Ownership insight: A difference list is a function Vec<T> -> Vec<T>.
/// In Rust, we use Box<dyn FnOnce> since each function is consumed on use.

/// A difference list wraps a function that prepends elements to a tail
pub struct DList<T> {
    f: Box<dyn FnOnce(Vec<T>) -> Vec<T>>,
}

impl<T: 'static> DList<T> {
    pub fn empty() -> Self {
        DList { f: Box::new(|v| v) }
    }

    pub fn singleton(x: T) -> Self {
        DList {
            f: Box::new(move |mut v| { v.insert(0, x); v }),
        }
    }

    pub fn from_vec(mut items: Vec<T>) -> Self {
        DList {
            f: Box::new(move |mut v| { items.append(&mut v); items }),
        }
    }

    /// O(1) append — just composes two functions
    pub fn append(self, other: DList<T>) -> Self {
        DList {
            f: Box::new(move |v| (self.f)((other.f)(v))),
        }
    }

    /// Materialize the difference list into a Vec
    pub fn to_vec(self) -> Vec<T> {
        (self.f)(Vec::new())
    }
}

/// Version 2: Simple vec-based builder (practical alternative)
pub struct VecBuilder<T> {
    chunks: Vec<Vec<T>>,
}

impl<T> VecBuilder<T> {
    pub fn new() -> Self { VecBuilder { chunks: vec![] } }

    pub fn push_vec(&mut self, v: Vec<T>) { self.chunks.push(v); }

    pub fn build(self) -> Vec<T> {
        let total: usize = self.chunks.iter().map(|c| c.len()).sum();
        let mut result = Vec::with_capacity(total);
        for mut chunk in self.chunks { result.append(&mut chunk); }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let dl: DList<i32> = DList::empty();
        assert_eq!(dl.to_vec(), Vec::<i32>::new());
    }

    #[test]
    fn test_singleton() {
        assert_eq!(DList::singleton(42).to_vec(), vec![42]);
    }

    #[test]
    fn test_append() {
        let a = DList::from_vec(vec![1, 2, 3]);
        let b = DList::from_vec(vec![4, 5, 6]);
        let c = DList::singleton(7);
        assert_eq!(a.append(b).append(c).to_vec(), vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_vec_builder() {
        let mut b = VecBuilder::new();
        b.push_vec(vec![1, 2]);
        b.push_vec(vec![3, 4]);
        assert_eq!(b.build(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_many_appends() {
        let mut dl = DList::empty();
        for i in 0..100 {
            dl = dl.append(DList::singleton(i));
        }
        let v = dl.to_vec();
        assert_eq!(v.len(), 100);
        assert_eq!(v[0], 0);
        assert_eq!(v[99], 99);
    }
}

fn main() {
    println!("{:?}", dl.to_vec(), Vec::<i32>::new());
    println!("{:?}", DList::singleton(42).to_vec(), vec![42]);
    println!("{:?}", a.append(b).append(c).to_vec(), vec![1, 2, 3, 4, 5, 6, 7]);
}
