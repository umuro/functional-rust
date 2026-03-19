#![allow(clippy::all)]
//! Immutable Binary Search Tree
//!
//! OCaml uses `type 'a bst = Leaf | Node of 'a bst * 'a * 'a bst`.
//! Rust uses `enum Bst<T>` with `Box` for heap allocation of recursive variants.
//! Both languages naturally express immutable, persistent data structures.

//! A persistent (immutable) binary search tree.
//! Each insert creates a new tree sharing unchanged subtrees with the original.
#[derive(Debug, Clone, PartialEq)]
pub enum Bst<T> {
    Leaf,
    Node(Box<Bst<T>>, T, Box<Bst<T>>),
}

impl<T: Ord + Clone> Bst<T> {
    /// Creates an empty tree.
    pub fn new() -> Self {
        Bst::Leaf
    }

    /// Inserts a value, returning a new tree.
    /// Duplicates are ignored (set semantics).
    ///
    /// OCaml: `let rec insert x = function | Leaf -> Node(Leaf, x, Leaf) | ...`
    /// Rust must use Box for recursive heap allocation.
    pub fn insert(&self, x: T) -> Self {
        match self {
            Bst::Leaf => Bst::Node(Box::new(Bst::Leaf), x, Box::new(Bst::Leaf)),
            Bst::Node(left, val, right) => match x.cmp(val) {
                std::cmp::Ordering::Less => {
                    Bst::Node(Box::new(left.insert(x)), val.clone(), right.clone())
                }
                std::cmp::Ordering::Greater => {
                    Bst::Node(left.clone(), val.clone(), Box::new(right.insert(x)))
                }
                std::cmp::Ordering::Equal => self.clone(),
            },
        }
    }

    /// Checks membership in the tree.
    ///
    /// OCaml: `let rec mem x = function | Leaf -> false | Node(l,v,r) -> ...`
    /// Rust borrows `&self` — no allocation needed for lookup.
    pub fn mem(&self, x: &T) -> bool {
        match self {
            Bst::Leaf => false,
            Bst::Node(left, val, right) => match x.cmp(val) {
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Less => left.mem(x),
                std::cmp::Ordering::Greater => right.mem(x),
            },
        }
    }

    /// Returns elements in sorted order via in-order traversal.
    ///
    /// OCaml: `let rec inorder = function | Leaf -> [] | Node(l,v,r) -> inorder l @ [v] @ inorder r`
    /// Rust collects into a Vec, which owns the values.
    pub fn inorder(&self) -> Vec<T> {
        match self {
            Bst::Leaf => vec![],
            Bst::Node(left, val, right) => {
                let mut result = left.inorder();
                result.push(val.clone());
                result.extend(right.inorder());
                result
            }
        }
    }

    /// Functional construction: builds a tree from an iterator.
    /// Mirrors OCaml's `List.fold_left (fun t x -> insert x t) Leaf items`.
    pub fn build(items: impl IntoIterator<Item = T>) -> Self {
        items.into_iter().fold(Bst::new(), |tree, x| tree.insert(x))
    }
}

impl<T: Ord + Clone> Default for Bst<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: Bst<i32> = Bst::new();
        assert_eq!(tree.inorder(), Vec::<i32>::new());
        assert!(!tree.mem(&1));
    }

    #[test]
    fn test_single_element() {
        let tree = Bst::new().insert(42);
        assert_eq!(tree.inorder(), vec![42]);
        assert!(tree.mem(&42));
        assert!(!tree.mem(&0));
    }

    #[test]
    fn test_multiple_elements_sorted() {
        let tree = Bst::build([5, 3, 7, 1, 4, 6, 8]);
        assert_eq!(tree.inorder(), vec![1, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_membership() {
        let tree = Bst::build([5, 3, 7, 1, 4, 6, 8]);
        assert!(tree.mem(&4));
        assert!(tree.mem(&5));
        assert!(tree.mem(&8));
        assert!(!tree.mem(&9));
        assert!(!tree.mem(&0));
        assert!(!tree.mem(&2));
    }

    #[test]
    fn test_duplicate_insert() {
        let tree = Bst::build([3, 1, 3, 2, 1]);
        assert_eq!(tree.inorder(), vec![1, 2, 3]);
    }

    #[test]
    fn test_persistence() {
        // Inserting into a tree doesn't modify the original
        let tree1 = Bst::build([5, 3, 7]);
        let tree2 = tree1.insert(1);
        assert!(!tree1.mem(&1));
        assert!(tree2.mem(&1));
    }
}
