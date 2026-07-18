#![allow(clippy::all)]
// Construct a complete binary tree of n nodes, left-justified (OCaml 99 Problems #34).
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

pub fn count_nodes<T>(tree: &Tree<T>) -> usize {
    match tree {
        Tree::Leaf => 0,
        Tree::Node(_, l, r) => 1 + count_nodes(l) + count_nodes(r),
    }
}

pub fn complete_binary_tree(n: usize) -> Tree<()> {
    if n == 0 {
        return Tree::Leaf;
    }
    let remaining = n - 1;
    let left_count = remaining / 2 + remaining % 2;
    let right_count = remaining - left_count;
    Tree::node((), complete_binary_tree(left_count), complete_binary_tree(right_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_count_matches_n() {
        for n in 0..20 {
            assert_eq!(count_nodes(&complete_binary_tree(n)), n);
        }
    }

    #[test]
    fn test_zero_nodes_is_leaf() {
        assert_eq!(complete_binary_tree(0), Tree::Leaf);
    }

    #[test]
    fn test_left_is_at_least_as_large_as_right() {
        fn min_depth<T>(tree: &Tree<T>) -> usize {
            match tree {
                Tree::Leaf => 0,
                Tree::Node(_, l, r) => 1 + min_depth(l).min(min_depth(r)),
            }
        }
        fn max_depth<T>(tree: &Tree<T>) -> usize {
            match tree {
                Tree::Leaf => 0,
                Tree::Node(_, l, r) => 1 + max_depth(l).max(max_depth(r)),
            }
        }
        // A complete tree's min and max leaf depth differ by at most 1.
        let t = complete_binary_tree(10);
        assert!(max_depth(&t) - min_depth(&t) <= 1);
    }
}
