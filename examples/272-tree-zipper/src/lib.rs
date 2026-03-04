/// A binary tree — either a Leaf (empty) or a Node with left subtree, value, right subtree.
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T> Tree<T> {
    pub fn node(left: Tree<T>, val: T, right: Tree<T>) -> Self {
        Tree::Node(Box::new(left), val, Box::new(right))
    }
}

/// A crumb records which direction we descended and what we left behind.
/// Left(v, r)  — we went left; parent had value v and right subtree r
/// Right(l, v) — we went right; parent had left subtree l and value v
#[derive(Debug, Clone)]
pub enum Crumb<T> {
    Left(T, Tree<T>),
    Right(Tree<T>, T),
}

/// A zipper: a focused subtree plus the breadcrumb trail back to the root.
/// Navigation is O(1); rebuilding the whole tree is O(depth).
#[derive(Debug, Clone)]
pub struct Zipper<T> {
    pub focus: Tree<T>,
    pub trail: Vec<Crumb<T>>,
}

/// Wrap a tree in a zipper focused on the root.
pub fn of_tree<T>(tree: Tree<T>) -> Zipper<T> {
    Zipper {
        focus: tree,
        trail: Vec::new(),
    }
}

/// Move focus to the left child. Returns None if focused on a Leaf.
pub fn go_left<T>(mut z: Zipper<T>) -> Option<Zipper<T>> {
    match z.focus {
        Tree::Leaf => None,
        Tree::Node(l, v, r) => {
            z.trail.push(Crumb::Left(v, *r));
            Some(Zipper {
                focus: *l,
                trail: z.trail,
            })
        }
    }
}

/// Move focus to the right child. Returns None if focused on a Leaf.
pub fn go_right<T>(mut z: Zipper<T>) -> Option<Zipper<T>> {
    match z.focus {
        Tree::Leaf => None,
        Tree::Node(l, v, r) => {
            z.trail.push(Crumb::Right(*l, v));
            Some(Zipper {
                focus: *r,
                trail: z.trail,
            })
        }
    }
}

/// Move focus to the parent. Returns None if already at the root.
pub fn go_up<T>(mut z: Zipper<T>) -> Option<Zipper<T>> {
    match z.trail.pop() {
        None => None,
        Some(Crumb::Left(v, r)) => Some(Zipper {
            focus: Tree::node(z.focus, v, r),
            trail: z.trail,
        }),
        Some(Crumb::Right(l, v)) => Some(Zipper {
            focus: Tree::node(l, v, z.focus),
            trail: z.trail,
        }),
    }
}

/// Replace the value at the current focus (no-op if focused on a Leaf).
pub fn set_value<T>(x: T, z: Zipper<T>) -> Zipper<T> {
    match z.focus {
        Tree::Leaf => z,
        Tree::Node(l, _, r) => Zipper {
            focus: Tree::node(*l, x, *r),
            trail: z.trail,
        },
    }
}

/// Climb back to the root and return the reconstructed tree (idiomatic iterative).
///
/// Note: in OCaml `to_tree` can be written as a one-liner because the language
/// lets you read `z.focus` after passing `z` to `go_up` in the same match.
/// In Rust, `go_up` consumes `z` by value, so we must save the focus *before*
/// calling `go_up`, which is clearest expressed as a loop.
pub fn to_tree<T>(mut z: Zipper<T>) -> Tree<T> {
    loop {
        if z.trail.is_empty() {
            return z.focus;
        }
        z = go_up(z).expect("trail was non-empty");
    }
}

/// Climb back to the root — explicit recursion mirrors the OCaml original.
pub fn to_tree_recursive<T>(z: Zipper<T>) -> Tree<T> {
    if z.trail.is_empty() {
        return z.focus;
    }
    to_tree_recursive(go_up(z).expect("trail was non-empty"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Tree<i32> {
        // Node(Node(Leaf,1,Leaf), 2, Node(Leaf,3,Leaf))
        Tree::node(
            Tree::node(Tree::Leaf, 1, Tree::Leaf),
            2,
            Tree::node(Tree::Leaf, 3, Tree::Leaf),
        )
    }

    #[test]
    fn test_of_tree_is_root() {
        let z = of_tree(sample_tree());
        assert!(z.trail.is_empty());
    }

    #[test]
    fn test_go_left_moves_focus() {
        let z = of_tree(sample_tree());
        let z = go_left(z).expect("should have left child");
        assert_eq!(z.focus, Tree::node(Tree::Leaf, 1, Tree::Leaf));
        assert_eq!(z.trail.len(), 1);
    }

    #[test]
    fn test_go_right_moves_focus() {
        let z = of_tree(sample_tree());
        let z = go_right(z).expect("should have right child");
        assert_eq!(z.focus, Tree::node(Tree::Leaf, 3, Tree::Leaf));
        assert_eq!(z.trail.len(), 1);
    }

    #[test]
    fn test_go_left_then_up_returns_root() {
        let z = of_tree(sample_tree());
        let z = go_left(z).expect("left");
        let z = go_up(z).expect("up");
        assert_eq!(z.focus, sample_tree());
        assert!(z.trail.is_empty());
    }

    #[test]
    fn test_go_right_then_up_returns_root() {
        let z = of_tree(sample_tree());
        let z = go_right(z).expect("right");
        let z = go_up(z).expect("up");
        assert_eq!(z.focus, sample_tree());
        assert!(z.trail.is_empty());
    }

    #[test]
    fn test_set_value_updates_focused_node() {
        let z = of_tree(sample_tree());
        let z = go_left(z).expect("left");
        let z = set_value(10, z);
        assert_eq!(z.focus, Tree::node(Tree::Leaf, 10, Tree::Leaf));
    }

    #[test]
    fn test_set_value_on_leaf_is_noop() {
        let z: Zipper<i32> = of_tree(Tree::Leaf);
        let z2 = set_value(42, z);
        assert_eq!(z2.focus, Tree::Leaf);
    }

    #[test]
    fn test_to_tree_rebuilds_after_edit() {
        let z = of_tree(sample_tree());
        let z = go_left(z).expect("left");
        let z = set_value(10, z);
        let result = to_tree(z);
        let expected = Tree::node(
            Tree::node(Tree::Leaf, 10, Tree::Leaf),
            2,
            Tree::node(Tree::Leaf, 3, Tree::Leaf),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_tree_recursive_matches_iter() {
        let z = of_tree(sample_tree());
        let z = go_right(z).expect("right");
        let z = set_value(30, z);
        let result = to_tree_recursive(z);
        let expected = Tree::node(
            Tree::node(Tree::Leaf, 1, Tree::Leaf),
            2,
            Tree::node(Tree::Leaf, 30, Tree::Leaf),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_go_left_on_leaf_returns_none() {
        let z: Zipper<i32> = of_tree(Tree::Leaf);
        assert!(go_left(z).is_none());
    }

    #[test]
    fn test_go_up_at_root_returns_none() {
        let z = of_tree(sample_tree());
        assert!(go_up(z).is_none());
    }

    #[test]
    fn test_deep_navigation_and_edit() {
        // Build a 3-level tree: root=5, left subtree root=3, its left child=1
        let tree = Tree::node(
            Tree::node(Tree::node(Tree::Leaf, 1, Tree::Leaf), 3, Tree::Leaf),
            5,
            Tree::Leaf,
        );
        let z = of_tree(tree);
        let z = go_left(z).expect("left");
        let z = go_left(z).expect("left-left");
        let z = set_value(99, z);
        let result = to_tree(z);
        let expected = Tree::node(
            Tree::node(Tree::node(Tree::Leaf, 99, Tree::Leaf), 3, Tree::Leaf),
            5,
            Tree::Leaf,
        );
        assert_eq!(result, expected);
    }
}
