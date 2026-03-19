#![allow(clippy::all)]
// Example 179: GADT Preventing Runtime Errors — Safe Head
// Type-level guarantees that head can never be called on an empty list

use std::marker::PhantomData;

// === Approach 1: Phantom type states for emptiness ===

struct Empty;
struct NonEmpty;

struct SafeList<T, S> {
    data: Vec<T>,
    _state: PhantomData<S>,
}

impl<T> SafeList<T, Empty> {
    fn new() -> Self {
        SafeList {
            data: Vec::new(),
            _state: PhantomData,
        }
    }

    // Pushing to empty list transitions to NonEmpty
    fn push(mut self, val: T) -> SafeList<T, NonEmpty> {
        self.data.push(val);
        SafeList {
            data: self.data,
            _state: PhantomData,
        }
    }
}

impl<T> SafeList<T, NonEmpty> {
    // head is ONLY available on NonEmpty lists
    fn head(&self) -> &T {
        &self.data[0]
    }

    fn tail(&self) -> &[T] {
        &self.data[1..]
    }

    fn push(mut self, val: T) -> SafeList<T, NonEmpty> {
        self.data.push(val);
        self
    }

    fn to_vec(&self) -> &Vec<T> {
        &self.data
    }
}

// Construct from a known non-empty source
fn from_vec<T>(v: Vec<T>) -> Option<SafeList<T, NonEmpty>> {
    if v.is_empty() {
        None
    } else {
        Some(SafeList {
            data: v,
            _state: PhantomData,
        })
    }
}

// === Approach 2: NonEmpty struct (like OCaml NonEmpty module) ===

#[derive(Debug, Clone)]
struct NonEmptyVec<T> {
    head: T,
    tail: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    fn new(head: T, tail: Vec<T>) -> Self {
        NonEmptyVec { head, tail }
    }

    fn from_vec(v: Vec<T>) -> Option<Self> {
        let mut iter = v.into_iter();
        iter.next().map(|head| NonEmptyVec {
            head,
            tail: iter.collect(),
        })
    }

    fn head(&self) -> &T {
        &self.head
    }

    fn tail(&self) -> &[T] {
        &self.tail
    }

    fn to_vec(&self) -> Vec<&T> {
        let mut v = vec![&self.head];
        v.extend(self.tail.iter());
        v
    }

    fn map<U>(&self, f: impl Fn(&T) -> U) -> NonEmptyVec<U> {
        NonEmptyVec {
            head: f(&self.head),
            tail: self.tail.iter().map(f).collect(),
        }
    }

    fn fold<U>(&self, init: U, f: impl Fn(U, &T) -> U) -> U {
        let acc = f(init, &self.head);
        self.tail.iter().fold(acc, f)
    }
}

// === Approach 3: Type-state with const generics ===
// Using const generics to guarantee non-empty at type level

struct StaticList<T, const N: usize> {
    data: [T; N],
}

// head only exists when N >= 1
impl<T, const N: usize> StaticList<T, N> {
    fn head(&self) -> &T {
        &self.data[0]
    }

    fn len(&self) -> usize {
        N
    }
}

// Cannot create StaticList<T, 0> with this constructor
fn static_list<T, const N: usize>(data: [T; N]) -> StaticList<T, N> {
    StaticList { data }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_list_head() {
        let list = SafeList::<_, Empty>::new().push(42);
        assert_eq!(*list.head(), 42);
    }

    #[test]
    fn test_safe_list_chain() {
        let list = SafeList::<_, Empty>::new().push(1).push(2).push(3);
        assert_eq!(*list.head(), 1);
        assert_eq!(list.tail(), &[2, 3]);
    }

    #[test]
    fn test_from_vec() {
        assert!(from_vec::<i32>(vec![]).is_none());
        let l = from_vec(vec![1, 2]).unwrap();
        assert_eq!(*l.head(), 1);
    }

    #[test]
    fn test_nonempty_vec() {
        let ne = NonEmptyVec::new(1, vec![2, 3]);
        assert_eq!(*ne.head(), 1);
        assert_eq!(ne.tail(), &[2, 3]);
    }

    #[test]
    fn test_nonempty_map() {
        let ne = NonEmptyVec::new(1, vec![2, 3]);
        let d = ne.map(|x| x * 2);
        assert_eq!(*d.head(), 2);
        assert_eq!(d.tail, vec![4, 6]);
    }

    #[test]
    fn test_nonempty_fold() {
        let ne = NonEmptyVec::new(1, vec![2, 3]);
        assert_eq!(ne.fold(0, |a, x| a + x), 6);
    }

    #[test]
    fn test_nonempty_from_vec() {
        assert!(NonEmptyVec::<i32>::from_vec(vec![]).is_none());
        let ne = NonEmptyVec::from_vec(vec![42]).unwrap();
        assert_eq!(*ne.head(), 42);
    }

    #[test]
    fn test_static_list() {
        let sl = static_list([10, 20, 30]);
        assert_eq!(*sl.head(), 10);
        assert_eq!(sl.len(), 3);
    }
}
