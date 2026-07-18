#![allow(clippy::all)]
// Dotstring encoding of a tree (OCaml 99 Problems #39): preorder with "." for Leaf, self-delimiting.
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

pub fn to_dotstring(tree: &Tree<char>) -> String {
    match tree {
        Tree::Leaf => ".".to_string(),
        Tree::Node(c, l, r) => format!("{}{}{}", c, to_dotstring(l), to_dotstring(r)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_dotstring() {
        let t = Tree::node('a', Tree::node('b', Tree::leaf(), Tree::leaf()), Tree::node('c', Tree::leaf(), Tree::leaf()));
        assert_eq!(to_dotstring(&t), "ab..c..");
    }

    #[test]
    fn test_leaf_dotstring() {
        assert_eq!(to_dotstring(&Tree::Leaf), ".");
    }

    #[test]
    fn test_deeper_tree() {
        let t = Tree::node('a', Tree::node('b', Tree::node('d', Tree::leaf(), Tree::leaf()), Tree::leaf()), Tree::leaf());
        assert_eq!(to_dotstring(&t), "abd....");
    }
}
