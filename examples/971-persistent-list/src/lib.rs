// 971: Persistent/Immutable Linked List with Structural Sharing
// OCaml lists are GC-managed with implicit sharing
// Rust needs Rc<T> to share nodes between multiple list versions

use std::rc::Rc;

// Approach 1: Persistent stack using Rc for structural sharing
#[derive(Debug)]
pub enum PList<T> {
    Nil,
    Cons(T, Rc<PList<T>>),
}

impl<T: Clone + PartialEq + std::fmt::Debug> PList<T> {
    pub fn nil() -> Rc<PList<T>> {
        Rc::new(PList::Nil)
    }

    pub fn push(x: T, tail: &Rc<PList<T>>) -> Rc<PList<T>> {
        Rc::new(PList::Cons(x, Rc::clone(tail))) // O(1), shares tail
    }

    pub fn pop(list: &Rc<PList<T>>) -> Option<(&T, Rc<PList<T>>)> {
        match list.as_ref() {
            PList::Nil => None,
            PList::Cons(x, rest) => Some((x, Rc::clone(rest))),
        }
    }

    pub fn peek(list: &Rc<PList<T>>) -> Option<&T> {
        match list.as_ref() {
            PList::Nil => None,
            PList::Cons(x, _) => Some(x),
        }
    }

    pub fn length(list: &Rc<PList<T>>) -> usize {
        match list.as_ref() {
            PList::Nil => 0,
            PList::Cons(_, rest) => 1 + Self::length(rest),
        }
    }

    pub fn to_vec(list: &Rc<PList<T>>) -> Vec<T> {
        let mut result = vec![];
        let mut current = Rc::clone(list);
        loop {
            match current.as_ref() {
                PList::Nil => break,
                PList::Cons(x, rest) => {
                    result.push(x.clone());
                    current = Rc::clone(rest);
                }
            }
        }
        result
    }
}

// Approach 2: Using standard Rc<Option<(T, Rc<...>)>> pattern (type alias)
// Simpler variant: functional list as enum with Rc sharing
#[derive(Debug, Clone, PartialEq)]
pub enum FList<T> {
    Nil,
    Cons(T, Rc<FList<T>>),
}

impl<T: Clone + PartialEq> FList<T> {
    pub fn empty() -> Self { FList::Nil }

    pub fn cons(head: T, tail: FList<T>) -> Self {
        FList::Cons(head, Rc::new(tail))
    }

    pub fn head(&self) -> Option<&T> {
        match self {
            FList::Nil => None,
            FList::Cons(x, _) => Some(x),
        }
    }

    pub fn tail(&self) -> Option<FList<T>> {
        match self {
            FList::Nil => None,
            FList::Cons(_, rest) => Some((**rest).clone()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            FList::Nil => 0,
            FList::Cons(_, rest) => 1 + rest.len(),
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut v = vec![];
        let mut curr = self.clone();
        while let FList::Cons(x, rest) = curr {
            v.push(x);
            curr = (*rest).clone();
        }
        v
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistent_push() {
        let s0 = PList::<i32>::nil();
        let s1 = PList::push(1, &s0);
        let s2 = PList::push(2, &s1);
        let s3 = PList::push(3, &s2);

        assert_eq!(PList::length(&s0), 0);
        assert_eq!(PList::length(&s1), 1);
        assert_eq!(PList::length(&s2), 2);
        assert_eq!(PList::length(&s3), 3);
    }

    #[test]
    fn test_old_versions_unchanged() {
        let s0 = PList::<i32>::nil();
        let s1 = PList::push(1, &s0);
        let s2 = PList::push(2, &s1);
        let _s3 = PList::push(3, &s2);

        // s2 is unchanged after creating s3
        assert_eq!(PList::peek(&s2), Some(&2));
        assert_eq!(PList::length(&s2), 2);
    }

    #[test]
    fn test_pop_returns_old_tail() {
        let s0 = PList::<i32>::nil();
        let s1 = PList::push(1, &s0);
        let s2 = PList::push(2, &s1);
        let s3 = PList::push(3, &s2);

        let (v, s2_prime) = PList::pop(&s3).unwrap();
        assert_eq!(*v, 3);
        assert_eq!(PList::peek(&s2_prime), Some(&2));
        assert_eq!(PList::length(&s2_prime), 2);
    }

    #[test]
    fn test_to_vec() {
        let s0 = PList::<i32>::nil();
        let s1 = PList::push(1, &s0);
        let s2 = PList::push(2, &s1);
        let s3 = PList::push(3, &s2);
        assert_eq!(PList::to_vec(&s3), vec![3, 2, 1]);
    }

    #[test]
    fn test_flist_persistence() {
        let l1 = FList::cons(1, FList::empty());
        let l2 = FList::cons(2, l1.clone());
        let l3 = FList::cons(3, l2.clone());
        assert_eq!(l3.to_vec(), vec![3, 2, 1]);
        assert_eq!(l2.to_vec(), vec![2, 1]); // unchanged
        assert_eq!(l1.to_vec(), vec![1]); // unchanged
    }
}
