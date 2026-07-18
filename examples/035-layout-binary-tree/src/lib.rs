#![allow(clippy::all)]
// Assign (x, y) coordinates for tree visualization (OCaml 99 Problems #35):
// x = in-order position (1-based), y = depth (1-based).
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

pub fn layout<T: Clone>(tree: &Tree<T>) -> Tree<(T, (usize, usize))> {
    fn go<T: Clone>(tree: &Tree<T>, x: &mut usize, depth: usize) -> Tree<(T, (usize, usize))> {
        match tree {
            Tree::Leaf => Tree::Leaf,
            Tree::Node(v, l, r) => {
                let left = go(l, x, depth + 1);
                *x += 1;
                let pos = (*x, depth);
                let right = go(r, x, depth + 1);
                Tree::node((v.clone(), pos), left, right)
            }
        }
    }
    let mut x = 0;
    go(tree, &mut x, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_simple_tree() {
        let t = Tree::node('a', Tree::node('b', Tree::leaf(), Tree::leaf()), Tree::node('c', Tree::leaf(), Tree::leaf()));
        let laid_out = layout(&t);
        assert_eq!(
            laid_out,
            Tree::node(
                ('a', (2, 1)),
                Tree::node(('b', (1, 2)), Tree::leaf(), Tree::leaf()),
                Tree::node(('c', (3, 2)), Tree::leaf(), Tree::leaf()),
            )
        );
    }

    #[test]
    fn test_layout_empty_tree() {
        assert_eq!(layout::<char>(&Tree::leaf()), Tree::leaf());
    }

    #[test]
    fn test_layout_single_node() {
        let t = Tree::node('a', Tree::leaf(), Tree::leaf());
        assert_eq!(layout(&t), Tree::node(('a', (1, 1)), Tree::leaf(), Tree::leaf()));
    }
}
