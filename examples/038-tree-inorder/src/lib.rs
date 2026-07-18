#![allow(clippy::all)]
// Inorder traversal: left, root, right (OCaml 99 Problems, ext. of #29-40).
// For a BST, inorder produces a sorted sequence.
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

impl<T> Tree<T> {
    pub fn leaf() -> Self {
        Tree::Leaf
    }

    pub fn node(val: T, left: Tree<T>, right: Tree<T>) -> Self {
        Tree::Node(val, Box::new(left), Box::new(right))
    }
}

pub fn inorder<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],
        Tree::Node(v, l, r) => {
            let mut result = inorder(l);
            result.push(v.clone());
            result.extend(inorder(r));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inorder_general_tree() {
        let t = Tree::node('a', Tree::node('b', Tree::leaf(), Tree::leaf()), Tree::node('c', Tree::leaf(), Tree::leaf()));
        assert_eq!(inorder(&t), vec!['b', 'a', 'c']);
    }

    #[test]
    fn test_inorder_of_bst_is_sorted() {
        // BST:      5
        //          / \
        //         3   8
        //        / \
        //       1   4
        let bst = Tree::node(
            5,
            Tree::node(3, Tree::node(1, Tree::leaf(), Tree::leaf()), Tree::node(4, Tree::leaf(), Tree::leaf())),
            Tree::node(8, Tree::leaf(), Tree::leaf()),
        );
        assert_eq!(inorder(&bst), vec![1, 3, 4, 5, 8]);
    }

    #[test]
    fn test_inorder_empty_tree() {
        let empty: Vec<i32> = vec![];
        assert_eq!(inorder::<i32>(&Tree::leaf()), empty);
    }
}
