//! Rope data structure for efficient string operations
//!
//! A tree-based string representation enabling O(1) concatenation and O(log n) splits.

/// A rope is either a leaf containing a string, or an internal node
/// with left and right children and a cached total length.
#[derive(Debug, Clone)]
pub enum Rope {
    Leaf(String),
    Node {
        left: Box<Rope>,
        right: Box<Rope>,
        length: usize,
    },
}

impl Rope {
    // === Approach 1: Basic constructor-based API ===

    /// Create a leaf rope from a string
    pub fn leaf(s: impl Into<String>) -> Self {
        Self::Leaf(s.into())
    }

    /// Get the total length of the rope
    pub fn length(&self) -> usize {
        match self {
            Self::Leaf(s) => s.len(),
            Self::Node { length, .. } => *length,
        }
    }

    /// Concatenate two ropes in O(1) time
    pub fn concat(left: Rope, right: Rope) -> Rope {
        if left.length() == 0 {
            return right;
        }
        if right.length() == 0 {
            return left;
        }
        let length = left.length() + right.length();
        Rope::Node {
            left: Box::new(left),
            right: Box::new(right),
            length,
        }
    }

    /// Convert rope to a standard String
    pub fn to_string(&self) -> String {
        match self {
            Self::Leaf(s) => s.clone(),
            Self::Node { left, right, .. } => left.to_string() + &right.to_string(),
        }
    }

    // === Approach 2: Index-based access ===

    /// Get byte at a specific index
    pub fn byte_at(&self, idx: usize) -> Option<u8> {
        match self {
            Self::Leaf(s) => s.as_bytes().get(idx).copied(),
            Self::Node { left, right, .. } => {
                let ll = left.length();
                if idx < ll {
                    left.byte_at(idx)
                } else {
                    right.byte_at(idx - ll)
                }
            }
        }
    }

    /// Get character at a specific index (assuming ASCII)
    pub fn char_at(&self, idx: usize) -> Option<char> {
        self.byte_at(idx).map(|b| b as char)
    }

    // === Approach 3: Split operation ===

    /// Split rope at index, returning (left, right) parts
    pub fn split_at(self, idx: usize) -> (Rope, Rope) {
        match self {
            Rope::Leaf(s) => {
                let split_idx = idx.min(s.len());
                let (a, b) = s.split_at(split_idx);
                (Rope::leaf(a), Rope::leaf(b))
            }
            Rope::Node { left, right, .. } => {
                let ll = left.length();
                if idx <= ll {
                    let (la, lb) = left.split_at(idx);
                    (la, Rope::concat(lb, *right))
                } else {
                    let (ra, rb) = right.split_at(idx - ll);
                    (Rope::concat(*left, ra), rb)
                }
            }
        }
    }

    /// Insert a string at a given index
    pub fn insert(self, idx: usize, s: &str) -> Rope {
        let (left, right) = self.split_at(idx);
        Rope::concat(Rope::concat(left, Rope::leaf(s)), right)
    }

    /// Delete a range [start, end) from the rope
    pub fn delete(self, start: usize, end: usize) -> Rope {
        let (left, rest) = self.split_at(start);
        let (_, right) = rest.split_at(end - start);
        Rope::concat(left, right)
    }

    /// Check if rope is empty
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Get a substring as a new rope
    pub fn substring(&self, start: usize, end: usize) -> Rope {
        let cloned = self.clone();
        let (_, rest) = cloned.split_at(start);
        let (result, _) = rest.split_at(end - start);
        result
    }
}

impl Default for Rope {
    fn default() -> Self {
        Rope::leaf("")
    }
}

impl From<&str> for Rope {
    fn from(s: &str) -> Self {
        Rope::leaf(s)
    }
}

impl From<String> for Rope {
    fn from(s: String) -> Self {
        Rope::leaf(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_creation_and_length() {
        let r = Rope::leaf("hello");
        assert_eq!(r.length(), 5);
        assert_eq!(r.to_string(), "hello");
    }

    #[test]
    fn test_concat_two_leaves() {
        let left = Rope::leaf("Hello, ");
        let right = Rope::leaf("World!");
        let r = Rope::concat(left, right);
        assert_eq!(r.to_string(), "Hello, World!");
        assert_eq!(r.length(), 13);
    }

    #[test]
    fn test_concat_empty() {
        let empty = Rope::leaf("");
        let nonempty = Rope::leaf("test");

        let r1 = Rope::concat(empty.clone(), nonempty.clone());
        assert_eq!(r1.to_string(), "test");

        let r2 = Rope::concat(nonempty, empty);
        assert_eq!(r2.to_string(), "test");
    }

    #[test]
    fn test_char_at() {
        let r = Rope::concat(Rope::leaf("abc"), Rope::leaf("def"));
        assert_eq!(r.char_at(0), Some('a'));
        assert_eq!(r.char_at(2), Some('c'));
        assert_eq!(r.char_at(3), Some('d'));
        assert_eq!(r.char_at(5), Some('f'));
        assert_eq!(r.char_at(6), None);
    }

    #[test]
    fn test_split_at_leaf() {
        let r = Rope::leaf("hello");
        let (left, right) = r.split_at(2);
        assert_eq!(left.to_string(), "he");
        assert_eq!(right.to_string(), "llo");
    }

    #[test]
    fn test_split_at_node() {
        let r = Rope::concat(Rope::leaf("hello"), Rope::leaf(" world"));
        let (left, right) = r.split_at(5);
        assert_eq!(left.to_string(), "hello");
        assert_eq!(right.to_string(), " world");
    }

    #[test]
    fn test_insert() {
        let r = Rope::leaf("helloworld");
        let r2 = r.insert(5, " ");
        assert_eq!(r2.to_string(), "hello world");
    }

    #[test]
    fn test_delete() {
        let r = Rope::leaf("hello world");
        let r2 = r.delete(5, 6);
        assert_eq!(r2.to_string(), "helloworld");
    }

    #[test]
    fn test_substring() {
        let r = Rope::concat(Rope::leaf("Hello, "), Rope::leaf("World!"));
        let sub = r.substring(7, 12);
        assert_eq!(sub.to_string(), "World");
    }

    #[test]
    fn test_nested_concatenation() {
        let r = Rope::concat(
            Rope::concat(Rope::leaf("a"), Rope::leaf("b")),
            Rope::concat(Rope::leaf("c"), Rope::leaf("d")),
        );
        assert_eq!(r.to_string(), "abcd");
        assert_eq!(r.length(), 4);
    }

    #[test]
    fn test_from_conversions() {
        let r1: Rope = "test".into();
        let r2: Rope = String::from("test").into();
        assert_eq!(r1.to_string(), "test");
        assert_eq!(r2.to_string(), "test");
    }
}
