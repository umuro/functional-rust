// 970: Rope String
// Tree-based string for O(log n) concat/split, O(log n) indexing
// OCaml: recursive type; Rust: enum with Box for recursion

#[derive(Debug, Clone)]
pub enum Rope {
    Leaf(String),
    Node(Box<Rope>, Box<Rope>, usize), // left, right, total_len
}

impl Rope {
    pub fn from_str(s: &str) -> Self {
        Rope::Leaf(s.to_string())
    }

    pub fn len(&self) -> usize {
        match self {
            Rope::Leaf(s) => s.len(),
            Rope::Node(_, _, n) => *n,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn concat(left: Rope, right: Rope) -> Rope {
        let total = left.len() + right.len();
        Rope::Node(Box::new(left), Box::new(right), total)
    }

    /// Collect rope into a String (O(n))
    pub fn to_string_val(&self) -> String {
        match self {
            Rope::Leaf(s) => s.clone(),
            Rope::Node(l, r, _) => {
                let mut out = l.to_string_val();
                out.push_str(&r.to_string_val());
                out
            }
        }
    }

    /// Get character at index i (O(log n))
    pub fn index(&self, i: usize) -> Option<char> {
        match self {
            Rope::Leaf(s) => s.chars().nth(i),
            Rope::Node(l, r, _) => {
                let ln = l.len();
                if i < ln {
                    l.index(i)
                } else {
                    r.index(i - ln)
                }
            }
        }
    }

    /// Split at position i: returns (left[0..i], right[i..])
    pub fn split(self, i: usize) -> (Rope, Rope) {
        match self {
            Rope::Leaf(s) => {
                let i = i.min(s.len());
                let left = Rope::Leaf(s[..i].to_string());
                let right = Rope::Leaf(s[i..].to_string());
                (left, right)
            }
            Rope::Node(l, r, _) => {
                let ln = l.len();
                if i <= ln {
                    let (ll, lr) = l.split(i);
                    (ll, Rope::concat(lr, *r))
                } else {
                    let (rl, rr) = r.split(i - ln);
                    (Rope::concat(*l, rl), rr)
                }
            }
        }
    }

    /// Extract substring [start, start+len)
    pub fn sub(&self, start: usize, len: usize) -> String {
        let (_, right) = self.clone().split(start);
        let (mid, _) = right.split(len);
        mid.to_string_val()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_rope() -> Rope {
        let r1 = Rope::from_str("Hello");
        let r2 = Rope::from_str(", ");
        let r3 = Rope::from_str("World");
        let r4 = Rope::from_str("!");
        Rope::concat(Rope::concat(r1, r2), Rope::concat(r3, r4))
    }

    #[test]
    fn test_length_and_to_string() {
        let rope = make_rope();
        assert_eq!(rope.len(), 13);
        assert_eq!(rope.to_string_val(), "Hello, World!");
    }

    #[test]
    fn test_index() {
        let rope = make_rope();
        assert_eq!(rope.index(0), Some('H'));
        assert_eq!(rope.index(7), Some('W'));
        assert_eq!(rope.index(12), Some('!'));
        assert_eq!(rope.index(13), None);
    }

    #[test]
    fn test_sub() {
        let rope = make_rope();
        assert_eq!(rope.sub(7, 5), "World");
        assert_eq!(rope.sub(0, 5), "Hello");
        assert_eq!(rope.sub(5, 2), ", ");
    }

    #[test]
    fn test_split() {
        let rope = make_rope();
        let (left, right) = rope.split(7);
        assert_eq!(left.to_string_val(), "Hello, ");
        assert_eq!(right.to_string_val(), "World!");
    }

    #[test]
    fn test_empty_rope() {
        let r = Rope::from_str("");
        assert!(r.is_empty());
        assert_eq!(r.to_string_val(), "");
    }
}
