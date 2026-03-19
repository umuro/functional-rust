//! # LinkedList in Rust
//! Doubly-linked list (rarely needed, Vec is usually better).

use std::collections::LinkedList;

pub fn build_list<T>(items: Vec<T>) -> LinkedList<T> {
    items.into_iter().collect()
}

pub fn concat<T>(mut a: LinkedList<T>, mut b: LinkedList<T>) -> LinkedList<T> {
    a.append(&mut b);
    a
}

pub fn split_at<T>(mut list: LinkedList<T>, at: usize) -> (LinkedList<T>, LinkedList<T>) {
    let second = list.split_off(at.min(list.len()));
    (list, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_and_iterate() {
        let list = build_list(vec![1, 2, 3]);
        let v: Vec<_> = list.iter().cloned().collect();
        assert_eq!(v, vec![1, 2, 3]);
    }
    #[test]
    fn concat_lists() {
        let a = build_list(vec![1, 2]);
        let b = build_list(vec![3, 4]);
        let c = concat(a, b);
        let v: Vec<_> = c.iter().cloned().collect();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
    #[test]
    fn split_list() {
        let list = build_list(vec![1, 2, 3, 4, 5]);
        let (a, b) = split_at(list, 2);
        assert_eq!(a.iter().cloned().collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(b.iter().cloned().collect::<Vec<_>>(), vec![3, 4, 5]);
    }
}
