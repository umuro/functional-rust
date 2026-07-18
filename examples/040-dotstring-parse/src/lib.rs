#![allow(clippy::all)]
// Parse a dotstring back to a tree (OCaml 99 Problems, complement to #39) via recursive descent.
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

pub fn parse_dotstring(chars: &[char], pos: &mut usize) -> Result<Tree<char>, String> {
    if *pos >= chars.len() {
        return Err("unexpected end of input".to_string());
    }
    let c = chars[*pos];
    *pos += 1;
    if c == '.' {
        Ok(Tree::Leaf)
    } else {
        let left = parse_dotstring(chars, pos)?;
        let right = parse_dotstring(chars, pos)?;
        Ok(Tree::node(c, left, right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_dotstring() {
        let chars: Vec<char> = "ab..c..".chars().collect();
        let mut pos = 0;
        let tree = parse_dotstring(&chars, &mut pos).unwrap();
        assert_eq!(
            tree,
            Tree::node('a', Tree::node('b', Tree::leaf(), Tree::leaf()), Tree::node('c', Tree::leaf(), Tree::leaf()))
        );
        assert_eq!(pos, chars.len());
    }

    #[test]
    fn test_parse_leaf_only() {
        let chars: Vec<char> = ".".chars().collect();
        let mut pos = 0;
        assert_eq!(parse_dotstring(&chars, &mut pos).unwrap(), Tree::Leaf);
    }

    #[test]
    fn test_parse_truncated_input_errors() {
        let chars: Vec<char> = "ab.".chars().collect(); // missing right subtree of 'b'
        let mut pos = 0;
        assert!(parse_dotstring(&chars, &mut pos).is_err());
    }

    #[test]
    fn test_parse_empty_input_errors() {
        let chars: Vec<char> = vec![];
        let mut pos = 0;
        assert!(parse_dotstring(&chars, &mut pos).is_err());
    }
}
