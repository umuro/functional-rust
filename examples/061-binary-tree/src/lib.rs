/// Binary Tree — Size, Membership, Traversal
///
/// A recursive enum mirrors OCaml's algebraic data type for binary trees.
/// Rust's `enum` is the direct equivalent of OCaml's `type 'a tree = Leaf | Node of ...`.
/// Key difference: Rust requires `Box` for recursive types because it needs to know
/// the size at compile time — OCaml's GC handles this transparently.

/// A generic binary tree. `Box` is needed because recursive types
/// would otherwise have infinite size on the stack.
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

impl<T> Tree<T> {
    /// Helper to construct a node without writing Box::new everywhere.
    pub fn node(val: T, left: Tree<T>, right: Tree<T>) -> Self {
        Tree::Node(val, Box::new(left), Box::new(right))
    }

    /// Helper to construct a leaf.
    pub fn leaf() -> Self {
        Tree::Leaf
    }
}

/// Count the number of nodes (recursive, mirrors OCaml's `size`).
pub fn size<T>(tree: &Tree<T>) -> usize {
    match tree {
        Tree::Leaf => 0,
        Tree::Node(_, l, r) => 1 + size(l) + size(r),
    }
}

/// Compute the depth (height) of the tree.
pub fn depth<T>(tree: &Tree<T>) -> usize {
    match tree {
        Tree::Leaf => 0,
        Tree::Node(_, l, r) => 1 + depth(l).max(depth(r)),
    }
}

/// Check membership — requires `PartialEq` for comparison.
/// In OCaml, structural equality is built in; in Rust we use trait bounds.
pub fn mem<T: PartialEq>(x: &T, tree: &Tree<T>) -> bool {
    match tree {
        Tree::Leaf => false,
        Tree::Node(v, l, r) => v == x || mem(x, l) || mem(x, r),
    }
}

/// Preorder traversal using an accumulator (tail-recursive style).
/// Returns owned values — requires `Clone` since we borrow the tree.
pub fn preorder<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    fn go<T: Clone>(tree: &Tree<T>, acc: &mut Vec<T>) {
        match tree {
            Tree::Leaf => {}
            Tree::Node(v, l, r) => {
                acc.push(v.clone());
                go(l, acc);
                go(r, acc);
            }
        }
    }
    let mut result = Vec::new();
    go(tree, &mut result);
    result
}

/// Inorder traversal — iterative with explicit stack, zero cloning needed
/// if we only collect references.
pub fn inorder<T>(tree: &Tree<T>) -> Vec<&T> {
    let mut result = Vec::new();
    let mut stack: Vec<&Tree<T>> = Vec::new();
    let mut current = tree;
    loop {
        match current {
            Tree::Node(v, l, _r) => {
                stack.push(current);
                current = l;
            }
            Tree::Leaf => {
                if let Some(node) = stack.pop() {
                    if let Tree::Node(v, _, r) = node {
                        result.push(v);
                        current = r;
                    }
                } else {
                    break;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use Tree::*;

    fn sample_tree() -> Tree<i32> {
        //      4
        //     / \
        //    2   5
        //   / \
        //  1   3
        Tree::node(
            4,
            Tree::node(2, Tree::node(1, Leaf, Leaf), Tree::node(3, Leaf, Leaf)),
            Tree::node(5, Leaf, Leaf),
        )
    }

    #[test]
    fn test_size() {
        assert_eq!(size(&sample_tree()), 5);
        assert_eq!(size::<i32>(&Leaf), 0);
        assert_eq!(size(&Tree::node(1, Leaf, Leaf)), 1);
    }

    #[test]
    fn test_depth() {
        assert_eq!(depth(&sample_tree()), 3);
        assert_eq!(depth::<i32>(&Leaf), 0);
        assert_eq!(depth(&Tree::node(1, Leaf, Leaf)), 1);
    }

    #[test]
    fn test_mem() {
        let t = sample_tree();
        assert!(mem(&3, &t));
        assert!(mem(&4, &t));
        assert!(!mem(&99, &t));
        assert!(!mem::<i32>(&1, &Leaf));
    }

    #[test]
    fn test_preorder() {
        assert_eq!(preorder(&sample_tree()), vec![4, 2, 1, 3, 5]);
        assert_eq!(preorder::<i32>(&Leaf), vec![]);
    }

    #[test]
    fn test_inorder() {
        assert_eq!(inorder(&sample_tree()), vec![&1, &2, &3, &4, &5]);
        assert_eq!(inorder::<i32>(&Leaf), Vec::<&i32>::new());
    }

    #[test]
    fn test_single_node() {
        let t = Tree::node(42, Leaf, Leaf);
        assert_eq!(size(&t), 1);
        assert_eq!(depth(&t), 1);
        assert!(mem(&42, &t));
        assert_eq!(preorder(&t), vec![42]);
    }
}
