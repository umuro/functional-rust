#![allow(clippy::all)]
// Example 108: Rc<T> — Shared Ownership
//
// Rc<T> = Reference Counted pointer. Multiple owners, single-threaded.
// OCaml shares all values implicitly via GC; Rust makes you opt in explicitly.

use std::rc::Rc;

// --- Approach 1: Shared tree nodes ----------------------------------------

// A binary tree where subtrees are shared via Rc.
// Cloning an Rc only bumps a counter — no heap allocation.
#[derive(Debug)]
pub enum Tree {
    Leaf,
    Node(Rc<Tree>, i32, Rc<Tree>),
}

pub fn tree_sum(t: &Tree) -> i32 {
    match t {
        Tree::Leaf => 0,
        Tree::Node(l, v, r) => tree_sum(l) + v + tree_sum(r),
    }
}

/// Build two trees that share a common subtree.
/// Returns (tree1_sum, tree2_sum, strong_count_of_shared_node).
pub fn shared_tree_demo() -> (i32, i32, usize) {
    // shared subtree: Node(Leaf, 42, Leaf)
    let shared = Rc::new(Tree::Node(Rc::new(Tree::Leaf), 42, Rc::new(Tree::Leaf)));

    // tree1 = Node(shared, 1, Leaf)  → sum = 42 + 1 = 43
    let tree1 = Tree::Node(Rc::clone(&shared), 1, Rc::new(Tree::Leaf));
    // tree2 = Node(Leaf, 2, shared)  → sum = 2 + 42 = 44
    let tree2 = Tree::Node(Rc::new(Tree::Leaf), 2, Rc::clone(&shared));

    let s1 = tree_sum(&tree1);
    let s2 = tree_sum(&tree2);
    // shared + tree1's Rc + tree2's Rc = 3 strong references
    let count = Rc::strong_count(&shared);
    (s1, s2, count)
}

// --- Approach 2: Reference-counted linked list (cons list) -----------------

// An immutable singly-linked list where tails are shared.
// Classic functional data structure: O(1) prepend, shared tails.
#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T: Copy> List<T> {
    pub fn nil() -> Rc<Self> {
        Rc::new(List::Nil)
    }

    /// Prepend `head` to `tail`, sharing `tail` without cloning its contents.
    pub fn cons(head: T, tail: Rc<Self>) -> Rc<Self> {
        Rc::new(List::Cons(head, tail))
    }

    pub fn to_vec(list: &Rc<Self>) -> Vec<T> {
        let mut acc = Vec::new();
        let mut cur = Rc::clone(list);
        loop {
            match cur.as_ref() {
                List::Nil => break,
                List::Cons(h, t) => {
                    acc.push(*h);
                    cur = Rc::clone(t);
                }
            }
        }
        acc
    }
}

/// Build a shared-tail cons list demo.
/// Returns (list_a, list_b) where both share the same tail [3, 2, 1].
pub fn shared_list_demo() -> (Vec<i32>, Vec<i32>, usize) {
    //  shared tail: [3, 2, 1]
    let tail = {
        let nil = List::nil();
        let t1 = List::cons(1, nil);
        let t2 = List::cons(2, t1);
        List::cons(3, t2)
    };

    // list_a = [10, 3, 2, 1]
    let list_a = List::cons(10, Rc::clone(&tail));
    // list_b = [20, 3, 2, 1]
    let list_b = List::cons(20, Rc::clone(&tail));

    let count = Rc::strong_count(&tail); // tail + list_a's Rc + list_b's Rc = 3
    (List::to_vec(&list_a), List::to_vec(&list_b), count)
}

// --- Approach 3: Rc drop semantics ----------------------------------------

/// Demonstrates that the value is dropped exactly when the last Rc is gone.
/// Returns strong_count at each stage: (after_clone, after_drop_one).
pub fn rc_drop_demo() -> (usize, usize) {
    let a = Rc::new(vec![1, 2, 3]);
    let b = Rc::clone(&a);
    let count_before = Rc::strong_count(&a); // 2

    drop(b); // decrement — not freed yet
    let count_after = Rc::strong_count(&a); // 1
                                            // `a` drops at end of scope — value is freed here
    (count_before, count_after)
}

// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_tree_sums() {
        let (s1, s2, _) = shared_tree_demo();
        assert_eq!(s1, 43); // shared(42) + 1
        assert_eq!(s2, 44); // 2 + shared(42)
    }

    #[test]
    fn test_shared_tree_ref_count() {
        let (_, _, count) = shared_tree_demo();
        // `shared` + tree1's Rc::clone + tree2's Rc::clone = 3
        assert_eq!(count, 3);
    }

    #[test]
    fn test_shared_list_contents() {
        let (a, b, _) = shared_list_demo();
        assert_eq!(a, vec![10, 3, 2, 1]);
        assert_eq!(b, vec![20, 3, 2, 1]);
    }

    #[test]
    fn test_shared_list_ref_count() {
        let (_, _, count) = shared_list_demo();
        // tail itself + list_a holds a clone + list_b holds a clone = 3
        assert_eq!(count, 3);
    }

    #[test]
    fn test_rc_drop_semantics() {
        let (before, after) = rc_drop_demo();
        assert_eq!(before, 2);
        assert_eq!(after, 1);
    }

    #[test]
    fn test_tree_leaf_sum_is_zero() {
        assert_eq!(tree_sum(&Tree::Leaf), 0);
    }

    #[test]
    fn test_single_node_tree() {
        let t = Tree::Node(Rc::new(Tree::Leaf), 7, Rc::new(Tree::Leaf));
        assert_eq!(tree_sum(&t), 7);
    }

    #[test]
    fn test_list_nil_is_empty() {
        let nil: Rc<List<i32>> = List::nil();
        assert_eq!(List::to_vec(&nil), Vec::<i32>::new());
    }
}
