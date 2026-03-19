#![allow(clippy::all)]
// Example 117: Recursive Types with Box
//
// Recursive types need Box in Rust because the compiler must know
// the size of each type at compile time. Box<T> is pointer-sized,
// breaking the infinite-size recursion.

// ── Approach 1: Binary search tree ──────────────────────────────────────────
//
// OCaml: type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree
// Rust:  the Node children must be boxed so the compiler sees a fixed size.

#[derive(Debug)]
pub enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Tree::Leaf
    }

    /// Insert a value, returning the updated tree (consumes self).
    pub fn insert(self, x: T) -> Self {
        match self {
            Tree::Leaf => Tree::Node(Box::new(Tree::Leaf), x, Box::new(Tree::Leaf)),
            Tree::Node(l, v, r) => {
                if x < v {
                    Tree::Node(Box::new(l.insert(x)), v, r)
                } else if x > v {
                    Tree::Node(l, v, Box::new(r.insert(x)))
                } else {
                    Tree::Node(l, v, r)
                }
            }
        }
    }

    /// In-order traversal yields elements in sorted order.
    pub fn to_sorted_vec(&self) -> Vec<&T> {
        match self {
            Tree::Leaf => vec![],
            Tree::Node(l, v, r) => {
                let mut result = l.to_sorted_vec();
                result.push(v);
                result.extend(r.to_sorted_vec());
                result
            }
        }
    }

    pub fn contains(&self, x: &T) -> bool {
        match self {
            Tree::Leaf => false,
            Tree::Node(l, v, r) => {
                if x < v {
                    l.contains(x)
                } else if x > v {
                    r.contains(x)
                } else {
                    true
                }
            }
        }
    }
}

impl<T: Ord> Default for Tree<T> {
    fn default() -> Self {
        Tree::new()
    }
}

// ── Approach 2: Singly-linked list ──────────────────────────────────────────
//
// OCaml: type 'a mylist = Nil | Cons of 'a * 'a mylist
// Rust:  tail must be Box'd — same reason as the tree above.

#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    pub fn nil() -> Self {
        List::Nil
    }

    /// Prepend an element (O(1)).
    pub fn cons(head: T, tail: Self) -> Self {
        List::Cons(head, Box::new(tail))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }

    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }

    pub fn to_vec(&self) -> Vec<&T> {
        let mut result = vec![];
        let mut current = self;
        loop {
            match current {
                List::Nil => break,
                List::Cons(h, tail) => {
                    result.push(h);
                    current = tail;
                }
            }
        }
        result
    }
}

// ── Approach 3: Expression AST ───────────────────────────────────────────────
//
// A classic use-case: recursive expression trees for interpreters / compilers.

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(e) => -eval(e),
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Tree tests

    #[test]
    fn tree_insert_and_sorted_order() {
        let tree = [5, 3, 7, 1, 4, 6, 8]
            .into_iter()
            .fold(Tree::new(), Tree::insert);
        let sorted: Vec<i32> = tree.to_sorted_vec().into_iter().copied().collect();
        assert_eq!(sorted, [1, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn tree_contains() {
        let tree = [10, 5, 15].into_iter().fold(Tree::new(), Tree::insert);
        assert!(tree.contains(&5));
        assert!(tree.contains(&15));
        assert!(!tree.contains(&99));
    }

    #[test]
    fn tree_empty_is_leaf() {
        let tree: Tree<i32> = Tree::new();
        assert!(tree.to_sorted_vec().is_empty());
        assert!(!tree.contains(&0));
    }

    #[test]
    fn tree_no_duplicates() {
        let tree = [3, 3, 3].into_iter().fold(Tree::new(), Tree::insert);
        assert_eq!(tree.to_sorted_vec(), [&3]);
    }

    // List tests

    #[test]
    fn list_len_and_to_vec() {
        let list = List::cons(1, List::cons(2, List::cons(3, List::nil())));
        assert_eq!(list.len(), 3);
        assert_eq!(list.to_vec(), [&1, &2, &3]);
    }

    #[test]
    fn list_nil_is_empty() {
        let list: List<i32> = List::nil();
        assert_eq!(list.len(), 0);
        assert!(list.to_vec().is_empty());
    }

    // Expr / AST tests

    #[test]
    fn expr_eval_arithmetic() {
        // (2 + 3) * -(4)  == -20
        let expr = Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Num(2.0)),
                Box::new(Expr::Num(3.0)),
            )),
            Box::new(Expr::Neg(Box::new(Expr::Num(4.0)))),
        );
        assert!((eval(&expr) - (-20.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn expr_eval_leaf() {
        assert!((eval(&Expr::Num(42.0)) - 42.0).abs() < f64::EPSILON);
    }
}
