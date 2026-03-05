//! Vec-like Collection Macros
//!
//! Creating collection literals with macros.

/// HashSet literal macro.
#[macro_export]
macro_rules! set {
    () => { ::std::collections::HashSet::new() };
    ($($elem:expr),+ $(,)?) => {{
        let mut s = ::std::collections::HashSet::new();
        $(s.insert($elem);)+
        s
    }};
}

/// BTreeSet literal macro.
#[macro_export]
macro_rules! btree_set {
    () => { ::std::collections::BTreeSet::new() };
    ($($elem:expr),+ $(,)?) => {{
        let mut s = ::std::collections::BTreeSet::new();
        $(s.insert($elem);)+
        s
    }};
}

/// HashMap literal macro.
#[macro_export]
macro_rules! map {
    () => { ::std::collections::HashMap::new() };
    ($($k:expr => $v:expr),+ $(,)?) => {{
        let mut m = ::std::collections::HashMap::new();
        $(m.insert($k, $v);)+
        m
    }};
}

/// VecDeque literal macro.
#[macro_export]
macro_rules! deque {
    () => { ::std::collections::VecDeque::new() };
    ($($elem:expr),+ $(,)?) => {{
        let mut d = ::std::collections::VecDeque::new();
        $(d.push_back($elem);)+
        d
    }};
}

/// LinkedList literal macro.
#[macro_export]
macro_rules! list {
    () => { ::std::collections::LinkedList::new() };
    ($($elem:expr),+ $(,)?) => {{
        let mut l = ::std::collections::LinkedList::new();
        $(l.push_back($elem);)+
        l
    }};
}

/// BinaryHeap (max-heap) literal macro.
#[macro_export]
macro_rules! heap {
    () => { ::std::collections::BinaryHeap::new() };
    ($($elem:expr),+ $(,)?) => {{
        let mut h = ::std::collections::BinaryHeap::new();
        $(h.push($elem);)+
        h
    }};
}

/// Vec with transformation.
#[macro_export]
macro_rules! vec_map {
    ($f:expr; $($elem:expr),* $(,)?) => {
        vec![$($f($elem)),*]
    };
}

/// Vec with filter.
#[macro_export]
macro_rules! vec_filter {
    ($pred:expr; $($elem:expr),* $(,)?) => {{
        let pred = $pred;
        let mut v = Vec::new();
        $(if pred(&$elem) { v.push($elem); })*
        v
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

    #[test]
    fn test_set_empty() {
        let s: HashSet<i32> = set!();
        assert!(s.is_empty());
    }

    #[test]
    fn test_set_elements() {
        let s = set![1, 2, 3, 2, 1];
        assert_eq!(s.len(), 3);
        assert!(s.contains(&1));
        assert!(s.contains(&2));
        assert!(s.contains(&3));
    }

    #[test]
    fn test_btree_set() {
        let s = btree_set![3, 1, 2];
        let v: Vec<_> = s.into_iter().collect();
        assert_eq!(v, vec![1, 2, 3]); // sorted
    }

    #[test]
    fn test_map_empty() {
        let m: HashMap<&str, i32> = map!();
        assert!(m.is_empty());
    }

    #[test]
    fn test_map_entries() {
        let m = map! {
            "a" => 1,
            "b" => 2,
        };
        assert_eq!(m["a"], 1);
        assert_eq!(m["b"], 2);
    }

    #[test]
    fn test_deque() {
        let d = deque![1, 2, 3];
        assert_eq!(d.len(), 3);
        assert_eq!(d.front(), Some(&1));
        assert_eq!(d.back(), Some(&3));
    }

    #[test]
    fn test_list() {
        let l = list![1, 2, 3];
        assert_eq!(l.len(), 3);
    }

    #[test]
    fn test_heap() {
        let mut h = heap![3, 1, 4, 1, 5];
        assert_eq!(h.pop(), Some(5)); // max first
        assert_eq!(h.pop(), Some(4));
    }

    #[test]
    fn test_vec_map() {
        let v = vec_map!(|x| x * 2; 1, 2, 3);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_vec_filter() {
        let v = vec_filter!(|x: &i32| *x % 2 == 0; 1, 2, 3, 4, 5);
        assert_eq!(v, vec![2, 4]);
    }
}
