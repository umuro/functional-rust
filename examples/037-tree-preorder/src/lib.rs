#![allow(clippy::all)]
// Preorder dot-string encoding of a tree: "." for Leaf, self-delimiting (OCaml 99 Problems, ext. of #29-40).
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

pub fn preorder(tree: &Tree<char>) -> String {
    match tree {
        Tree::Leaf => ".".to_string(),
        Tree::Node(c, l, r) => format!("{}{}{}", c, preorder(l), preorder(r)),
    }
}

pub fn from_preorder(s: &str) -> Tree<char> {
    let chars: Vec<char> = s.chars().collect();
    let mut pos = 0;
    parse(&chars, &mut pos)
}

fn parse(chars: &[char], pos: &mut usize) -> Tree<char> {
    let c = chars[*pos];
    *pos += 1;
    if c == '.' {
        Tree::Leaf
    } else {
        let left = parse(chars, pos);
        let right = parse(chars, pos);
        Tree::node(c, left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Tree<char> {
        Tree::node('a', Tree::node('b', Tree::leaf(), Tree::leaf()), Tree::node('c', Tree::leaf(), Tree::leaf()))
    }

    #[test]
    fn test_preorder_encoding() {
        assert_eq!(preorder(&sample()), "ab..c..");
    }

    #[test]
    fn test_from_preorder() {
        assert_eq!(from_preorder("ab..c.."), sample());
    }

    #[test]
    fn test_round_trip() {
        assert_eq!(from_preorder(&preorder(&sample())), sample());
    }

    #[test]
    fn test_leaf_tree() {
        assert_eq!(preorder(&Tree::Leaf), ".");
        assert_eq!(from_preorder("."), Tree::Leaf);
    }
}
