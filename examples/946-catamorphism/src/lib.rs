/// # Catamorphism — Generalized Fold on ADTs
///
/// A catamorphism replaces each constructor of an algebraic data type
/// with a function. It's the universal way to consume a recursive structure.

/// Binary tree
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T> Tree<T> {
    pub fn node(left: Tree<T>, value: T, right: Tree<T>) -> Self {
        Tree::Node(Box::new(left), value, Box::new(right))
    }
}

/// The catamorphism: replace Leaf with `leaf` value, Node with `node` function.
/// This is the most general way to fold over a tree.
pub fn cata<T, R>(tree: &Tree<T>, leaf: R, node: &dyn Fn(R, &T, R) -> R) -> R
where
    R: Clone,
{
    match tree {
        Tree::Leaf => leaf,
        Tree::Node(l, v, r) => {
            let left_result = cata(l, leaf.clone(), node);
            let right_result = cata(r, leaf, node);
            node(left_result, v, right_result)
        }
    }
}

/// Size: count all nodes
pub fn size<T>(tree: &Tree<T>) -> usize {
    cata(tree, 0, &|l, _, r| 1 + l + r)
}

/// Sum: add all values
pub fn sum(tree: &Tree<i64>) -> i64 {
    cata(tree, 0, &|l, v, r| l + v + r)
}

/// Height: longest path from root to leaf
pub fn height<T>(tree: &Tree<T>) -> usize {
    cata(tree, 0, &|l, _, r| 1 + l.max(r))
}

/// Mirror: swap left and right subtrees
pub fn mirror<T: Clone>(tree: &Tree<T>) -> Tree<T> {
    match tree {
        Tree::Leaf => Tree::Leaf,
        Tree::Node(l, v, r) => Tree::node(mirror(r), v.clone(), mirror(l)),
    }
}

/// In-order traversal to list
pub fn to_list<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    cata(tree, vec![], &|mut l, v, r| {
        l.push(v.clone());
        l.extend(r);
        l
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Tree<i64> {
        Tree::node(
            Tree::node(Tree::Leaf, 1, Tree::Leaf),
            2,
            Tree::node(Tree::Leaf, 3, Tree::Leaf),
        )
    }

    #[test]
    fn test_size() {
        assert_eq!(size(&sample_tree()), 3);
        assert_eq!(size::<i64>(&Tree::Leaf), 0);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&sample_tree()), 6);
    }

    #[test]
    fn test_height() {
        assert_eq!(height(&sample_tree()), 2);
        assert_eq!(height::<i64>(&Tree::Leaf), 0);
    }

    #[test]
    fn test_mirror() {
        let t = sample_tree();
        let m = mirror(&t);
        assert_eq!(to_list(&m), vec![3, 2, 1]);
    }

    #[test]
    fn test_to_list() {
        assert_eq!(to_list(&sample_tree()), vec![1, 2, 3]);
    }

    #[test]
    fn test_catamorphism_is_general() {
        // Any tree computation can be expressed as a cata
        let product = cata(&sample_tree(), 1i64, &|l, v, r| l * v * r);
        assert_eq!(product, 6); // 1 * 1 * 2 * 1 * 3 * 1 = 6... wait
                                // Actually: Node(Node(Leaf,1,Leaf), 2, Node(Leaf,3,Leaf))
                                // = node(node(1, 1, 1), 2, node(1, 3, 1)) = node(1, 2, 3) = 1*2*3 = 6
        assert_eq!(product, 6);
    }
}
