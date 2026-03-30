/// Huffman coding tree — functional implementation.
/// 
/// This is a direct translation of the OCaml Huffman tree example.
/// Shows functional tree construction and prefix code generation.

#[derive(Debug, Clone)]
enum HTree {
    Leaf(char, i32),
    Node(Box<HTree>, Box<HTree>, i32),
}

impl HTree {
    /// Extract frequency from a tree node.
    fn freq(&self) -> i32 {
        match self {
            HTree::Leaf(_, f) => *f,
            HTree::Node(_, _, f) => *f,
        }
    }
}

/// Builds a Huffman tree from character frequencies.
///
/// # Arguments
/// * `freqs` - Slice of character-frequency pairs.
///
/// # Returns
/// The constructed Huffman tree.
///
/// # Panics
/// Panics if the input slice is empty.
fn build_tree(freqs: &[(char, i32)]) -> HTree {
    // Convert frequencies to leaf nodes
    let mut trees: Vec<HTree> = freqs
        .iter()
        .map(|&(c, f)| HTree::Leaf(c, f))
        .collect();
    
    // Sort by frequency (ascending)
    trees.sort_by_key(|t| t.freq());
    
    // Build tree by repeatedly merging two smallest nodes
    while trees.len() > 1 {
        // Take two smallest trees
        let a = trees.remove(0);
        let b = trees.remove(0);
        let merged = HTree::Node(Box::new(a), Box::new(b), a.freq() + b.freq());
        
        trees.push(merged);
        trees.sort_by_key(|t| t.freq());
    }
    
    // Should have exactly one tree left
    trees.pop().expect("empty frequency list")
}

/// Generates prefix codes for all leaves in the Huffman tree.
///
/// # Arguments
/// * `prefix` - The binary prefix built so far (e.g., "0", "10", etc.).
/// * `tree` - The Huffman tree.
///
/// # Returns
/// Vector of character-code pairs.
fn codes(prefix: &str, tree: &HTree) -> Vec<(char, String)> {
    match tree {
        HTree::Leaf(c, _) => vec![(*c, prefix.to_string())],
        HTree::Node(left, right, _) => {
            let mut left_codes = codes(&format!("{}0", prefix), left);
            let right_codes = codes(&format!("{}1", prefix), right);
            left_codes.extend(right_codes);
            left_codes
        }
    }
}

/// Main function demonstrating Huffman coding.
fn main() {
    let freqs = vec![
        ('a', 5),
        ('b', 9),
        ('c', 12),
        ('d', 13),
        ('e', 16),
        ('f', 45),
    ];
    
    let tree = build_tree(&freqs);
    let huffman_codes = codes("", &tree);
    
    println!("Huffman codes:");
    for (c, code) in huffman_codes {
        println!("  {}: {}", c, code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq() {
        let leaf = HTree::Leaf('a', 5);
        assert_eq!(leaf.freq(), 5);
        
        let node = HTree::Node(
            Box::new(HTree::Leaf('a', 3)), 
            Box::new(HTree::Leaf('b', 4)), 
            7
        );
        assert_eq!(node.freq(), 7);
    }

    #[test]
    fn test_single_leaf() {
        let freqs = vec![('x', 100)];
        let tree = build_tree(&freqs);
        
        // Single leaf tree
        match tree {
            HTree::Leaf(c, f) => {
                assert_eq!(c, 'x');
                assert_eq!(f, 100);
            },
            HTree::Node(_, _, _) => panic!("Expected leaf, got node"),
        }
    }

    #[test]
    fn test_two_leaves() {
        let freqs = vec![('a', 1), ('b', 2)];
        let tree = build_tree(&freqs);
        
        // Should create a node with two leaves
        match tree {
            HTree::Node(left, right, f) => {
                assert_eq!(f, 3); // 1 + 2
                
                // Check left leaf
                match *left {
                    HTree::Leaf(c, freq) => {
                        // Sorted by frequency, 'a' (1) should be left
                        assert_eq!(c, 'a');
                        assert_eq!(freq, 1);
                    },
                    _ => panic!("Expected leaf"),
                }
                
                // Check right leaf  
                match *right {
                    HTree::Leaf(c, freq) => {
                        // 'b' (2) should be right
                        assert_eq!(c, 'b');
                        assert_eq!(freq, 2);
                    },
                    _ => panic!("Expected leaf"),
                }
            },
            HTree::Leaf(_, _) => panic!("Expected node, got leaf"),
        }
    }

    #[test]
    fn test_codes_single() {
        let tree = HTree::Leaf('x', 100);
        let result = codes("", &tree);
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], ('x', "".to_string()));
    }

    #[test]
    fn test_codes_two_level() {
        // Build a simple tree manually
        let tree = HTree::Node(
            Box::new(HTree::Leaf('a', 1)),
            Box::new(HTree::Leaf('b', 2)),
            3,
        );
        
        let result = codes("", &tree);
        assert_eq!(result.len(), 2);
        
        // Sort for stable comparison
        let mut result = result;
        result.sort_by_key(|(c, _)| *c);
        
        // Left child gets "0", right gets "1"
        assert_eq!(result[0], ('a', "0".to_string()));
        assert_eq!(result[1], ('b', "1".to_string()));
    }

    #[test]
    #[should_panic(expected = "empty frequency list")]
    fn test_empty_input() {
        let freqs: Vec<(char, i32)> = vec![];
        build_tree(&freqs);
    }

    #[test]
    fn test_book_example() {
        // The example from the OCaml code
        let freqs = vec![
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ];
        
        let tree = build_tree(&freqs);
        let result = codes("", &tree);
        
        // Should generate codes for all 6 characters
        assert_eq!(result.len(), 6);
        
        // All codes should be unique and have no prefix collisions
        let codes: Vec<String> = result.iter().map(|(_, c)| c.clone()).collect();
        for i in 0..codes.len() {
            for j in 0..codes.len() {
                if i != j {
                    // No code should be a prefix of another
                    assert!(!codes[i].starts_with(&codes[j]));
                }
            }
        }
    }
}