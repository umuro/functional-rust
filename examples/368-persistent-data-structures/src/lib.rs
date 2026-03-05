//! Persistent Data Structures
//!
//! Immutable data structures with structural sharing via Rc.

use std::rc::Rc;

// === Approach 1: Persistent Linked List ===

/// A persistent (immutable) linked list
#[derive(Clone, Debug)]
pub enum PList<T> {
    Nil,
    Cons(T, Rc<PList<T>>),
}

impl<T: Clone> PList<T> {
    /// Create an empty list
    pub fn nil() -> Rc<Self> {
        Rc::new(Self::Nil)
    }

    /// Prepend an element (O(1))
    pub fn cons(head: T, tail: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Cons(head, tail))
    }

    /// Get the head element
    pub fn head(list: &Rc<Self>) -> Option<&T> {
        match list.as_ref() {
            Self::Nil => None,
            Self::Cons(h, _) => Some(h),
        }
    }

    /// Get the tail (O(1) - just clone the Rc)
    pub fn tail(list: &Rc<Self>) -> Rc<Self> {
        match list.as_ref() {
            Self::Nil => Self::nil(),
            Self::Cons(_, t) => Rc::clone(t),
        }
    }

    /// Check if empty
    pub fn is_empty(list: &Rc<Self>) -> bool {
        matches!(list.as_ref(), Self::Nil)
    }

    /// Get length
    pub fn len(list: &Rc<Self>) -> usize {
        let mut count = 0;
        let mut cur = Rc::clone(list);
        loop {
            match cur.as_ref() {
                Self::Nil => break,
                Self::Cons(_, t) => {
                    count += 1;
                    cur = Rc::clone(t);
                }
            }
        }
        count
    }

    /// Convert to Vec
    pub fn to_vec(list: &Rc<Self>) -> Vec<T> {
        let mut v = Vec::new();
        let mut cur = Rc::clone(list);
        loop {
            match cur.as_ref() {
                Self::Nil => break,
                Self::Cons(x, next) => {
                    v.push(x.clone());
                    cur = Rc::clone(next);
                }
            }
        }
        v
    }

    /// Create from iterator
    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Rc<Self> {
        let items: Vec<T> = iter.into_iter().collect();
        let mut list = Self::nil();
        for item in items.into_iter().rev() {
            list = Self::cons(item, list);
        }
        list
    }
}

// === Approach 2: Persistent Vector (simple path-copying) ===

/// A persistent vector using copy-on-write
#[derive(Clone, Debug)]
pub struct PVec<T: Clone> {
    data: Rc<Vec<T>>,
}

impl<T: Clone> PVec<T> {
    /// Create an empty vector
    pub fn new() -> Self {
        Self {
            data: Rc::new(Vec::new()),
        }
    }

    /// Push an element (returns new vector)
    pub fn push(&self, val: T) -> Self {
        let mut new_data = (*self.data).clone();
        new_data.push(val);
        Self {
            data: Rc::new(new_data),
        }
    }

    /// Set element at index (returns new vector)
    pub fn set(&self, i: usize, val: T) -> Self {
        let mut new_data = (*self.data).clone();
        new_data[i] = val;
        Self {
            data: Rc::new(new_data),
        }
    }

    /// Get element at index
    pub fn get(&self, i: usize) -> Option<&T> {
        self.data.get(i)
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Convert to Vec
    pub fn to_vec(&self) -> Vec<T> {
        (*self.data).clone()
    }

    /// Pop last element (returns new vector and popped value)
    pub fn pop(&self) -> (Self, Option<T>) {
        if self.is_empty() {
            return (self.clone(), None);
        }
        let mut new_data = (*self.data).clone();
        let val = new_data.pop();
        (
            Self {
                data: Rc::new(new_data),
            },
            val,
        )
    }
}

impl<T: Clone> Default for PVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

// === Approach 3: Persistent Map (simple version) ===

/// A persistent map using sorted vector
#[derive(Clone, Debug)]
pub struct PMap<K: Ord + Clone, V: Clone> {
    data: Rc<Vec<(K, V)>>,
}

impl<K: Ord + Clone, V: Clone> PMap<K, V> {
    /// Create empty map
    pub fn new() -> Self {
        Self {
            data: Rc::new(Vec::new()),
        }
    }

    /// Insert key-value (returns new map)
    pub fn insert(&self, key: K, value: V) -> Self {
        let mut new_data = (*self.data).clone();
        match new_data.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(i) => new_data[i] = (key, value),
            Err(i) => new_data.insert(i, (key, value)),
        }
        Self {
            data: Rc::new(new_data),
        }
    }

    /// Get value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.data.binary_search_by(|(k, _)| k.cmp(key)) {
            Ok(i) => Some(&self.data[i].1),
            Err(_) => None,
        }
    }

    /// Remove key (returns new map)
    pub fn remove(&self, key: &K) -> Self {
        let mut new_data = (*self.data).clone();
        if let Ok(i) = new_data.binary_search_by(|(k, _)| k.cmp(key)) {
            new_data.remove(i);
        }
        Self {
            data: Rc::new(new_data),
        }
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<K: Ord + Clone, V: Clone> Default for PMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plist_basic() {
        let l1 = PList::cons(2, PList::cons(1, PList::nil()));
        let l2 = PList::cons(3, Rc::clone(&l1));

        assert_eq!(PList::to_vec(&l1), vec![2, 1]);
        assert_eq!(PList::to_vec(&l2), vec![3, 2, 1]);
        // l1 is unchanged
        assert_eq!(PList::to_vec(&l1), vec![2, 1]);
    }

    #[test]
    fn test_plist_head_tail() {
        let l = PList::cons(1, PList::cons(2, PList::nil()));
        assert_eq!(PList::head(&l), Some(&1));
        assert_eq!(PList::to_vec(&PList::tail(&l)), vec![2]);
    }

    #[test]
    fn test_plist_from_iter() {
        let l = PList::from_iter(vec![1, 2, 3]);
        assert_eq!(PList::to_vec(&l), vec![1, 2, 3]);
    }

    #[test]
    fn test_pvec_basic() {
        let v0: PVec<i32> = PVec::new();
        let v1 = v0.push(1);
        let v2 = v1.push(2);
        let v3 = v2.set(0, 99);

        assert_eq!(v1.to_vec(), vec![1]);
        assert_eq!(v2.to_vec(), vec![1, 2]);
        assert_eq!(v3.to_vec(), vec![99, 2]);
        // v1 unchanged
        assert_eq!(v1.to_vec(), vec![1]);
    }

    #[test]
    fn test_pvec_pop() {
        let v = PVec::new().push(1).push(2).push(3);
        let (v2, val) = v.pop();
        assert_eq!(val, Some(3));
        assert_eq!(v2.to_vec(), vec![1, 2]);
    }

    #[test]
    fn test_pmap_basic() {
        let m0: PMap<&str, i32> = PMap::new();
        let m1 = m0.insert("a", 1);
        let m2 = m1.insert("b", 2);
        let m3 = m2.insert("a", 10); // update

        assert_eq!(m1.get(&"a"), Some(&1));
        assert_eq!(m2.get(&"b"), Some(&2));
        assert_eq!(m3.get(&"a"), Some(&10));
        // m1 unchanged
        assert_eq!(m1.get(&"a"), Some(&1));
    }

    #[test]
    fn test_pmap_remove() {
        let m = PMap::new().insert("a", 1).insert("b", 2);
        let m2 = m.remove(&"a");
        assert_eq!(m2.get(&"a"), None);
        assert_eq!(m2.get(&"b"), Some(&2));
    }

    #[test]
    fn test_structural_sharing() {
        let l1 = PList::cons(1, PList::nil());
        let l2 = PList::cons(2, Rc::clone(&l1));
        let l3 = PList::cons(3, Rc::clone(&l1));
        // l2 and l3 share the tail l1
        assert_eq!(PList::len(&l1), 1);
        assert_eq!(PList::len(&l2), 2);
        assert_eq!(PList::len(&l3), 2);
    }
}
