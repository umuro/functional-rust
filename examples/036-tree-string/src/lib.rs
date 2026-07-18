#![allow(clippy::all)]
// Serialize a Tree<char> to "x(left,right)" format and parse it back (OCaml 99 Problems #36).
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

pub fn to_string(tree: &Tree<char>) -> String {
    match tree {
        Tree::Leaf => String::new(),
        Tree::Node(c, l, r) => {
            let ls = to_string(l);
            let rs = to_string(r);
            if ls.is_empty() && rs.is_empty() {
                c.to_string()
            } else {
                format!("{}({},{})", c, ls, rs)
            }
        }
    }
}

pub fn from_str(s: &str) -> Tree<char> {
    let chars: Vec<char> = s.chars().collect();
    let mut pos = 0;
    parse(&chars, &mut pos)
}

fn parse(chars: &[char], pos: &mut usize) -> Tree<char> {
    if *pos >= chars.len() || chars[*pos] == ',' || chars[*pos] == ')' {
        return Tree::Leaf;
    }
    let c = chars[*pos];
    *pos += 1;
    if *pos < chars.len() && chars[*pos] == '(' {
        *pos += 1; // consume '('
        let left = parse(chars, pos);
        *pos += 1; // consume ','
        let right = parse(chars, pos);
        *pos += 1; // consume ')'
        Tree::node(c, left, right)
    } else {
        Tree::node(c, Tree::Leaf, Tree::Leaf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Tree<char> {
        Tree::node(
            'a',
            Tree::node('b', Tree::node('d', Tree::leaf(), Tree::leaf()), Tree::node('e', Tree::leaf(), Tree::leaf())),
            Tree::node('c', Tree::leaf(), Tree::node('f', Tree::leaf(), Tree::leaf())),
        )
    }

    #[test]
    fn test_to_string() {
        assert_eq!(to_string(&sample()), "a(b(d,e),c(,f))");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(from_str("a(b(d,e),c(,f))"), sample());
    }

    #[test]
    fn test_round_trip() {
        assert_eq!(from_str(&to_string(&sample())), sample());
    }

    #[test]
    fn test_single_leaf_node() {
        let t = Tree::node('x', Tree::leaf(), Tree::leaf());
        assert_eq!(to_string(&t), "x");
        assert_eq!(from_str("x"), t);
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(to_string(&Tree::Leaf), "");
        assert_eq!(from_str(""), Tree::Leaf);
    }
}
