// 083: Display Trait
// Implement Display for custom types

use std::fmt;

// Approach 1: Simple Display
#[derive(Debug)]
enum Color { Red, Green, Blue }

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
        }
    }
}

#[derive(Debug)]
struct Point { x: f64, y: f64 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

// Approach 2: Complex formatting
#[derive(Debug)]
struct Person { name: String, age: u32, email: String }

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (age {}, {})", self.name, self.age, self.email)
    }
}

// Approach 3: Recursive Display
#[derive(Debug)]
enum Tree<T> { Leaf, Node(Box<Tree<T>>, T, Box<Tree<T>>) }

impl<T: fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tree::Leaf => write!(f, "."),
            Tree::Node(l, v, r) => write!(f, "({} {} {})", l, v, r),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_display() {
        assert_eq!(format!("{}", Color::Red), "Red");
        assert_eq!(format!("{}", Color::Green), "Green");
    }

    #[test]
    fn test_point_display() {
        assert_eq!(format!("{}", Point { x: 3.0, y: 4.0 }), "(3.0, 4.0)");
    }

    #[test]
    fn test_person_display() {
        let p = Person { name: "Alice".into(), age: 30, email: "alice@ex.com".into() };
        assert_eq!(format!("{}", p), "Alice (age 30, alice@ex.com)");
    }

    #[test]
    fn test_tree_display() {
        let tree = Tree::Node(
            Box::new(Tree::Node(Box::new(Tree::Leaf), 1, Box::new(Tree::Leaf))),
            2,
            Box::new(Tree::Node(Box::new(Tree::Leaf), 3, Box::new(Tree::Leaf))),
        );
        assert_eq!(format!("{}", tree), "((. 1 .) 2 (. 3 .))");
    }
}
