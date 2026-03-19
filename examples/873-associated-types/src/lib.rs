// Example 079: Associated Types
// OCaml module types → Rust associated types in traits

// === Approach 1: Trait with associated type ===
trait Container {
    type Item;

    fn empty() -> Self;
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Container for Stack<T> {
    type Item = T;

    fn empty() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

// === Approach 2: Associated type for output (like Add trait) ===
trait Combinable {
    type Output;
    fn combine(&self, other: &Self) -> Self::Output;
}

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Combinable for Point {
    type Output = f64; // distance between points
    fn combine(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Combinable for String {
    type Output = String;
    fn combine(&self, other: &Self) -> String {
        format!("{}{}", self, other)
    }
}

// === Approach 3: Multiple associated types ===
trait Transformer {
    type Input;
    type Output;
    fn transform(&self, input: Self::Input) -> Self::Output;
}

struct StringLength;

impl Transformer for StringLength {
    type Input = String;
    type Output = usize;
    fn transform(&self, input: String) -> usize {
        input.len()
    }
}

struct Doubler;

impl Transformer for Doubler {
    type Input = i32;
    type Output = i32;
    fn transform(&self, input: i32) -> i32 {
        input * 2
    }
}

// Generic function using associated types
fn apply_transform<T: Transformer>(t: &T, input: T::Input) -> T::Output {
    t.transform(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut s: Stack<i32> = Container::empty();
        assert!(s.is_empty());
        s.push(10);
        s.push(20);
        assert_eq!(s.size(), 2);
        assert_eq!(s.pop(), Some(20));
        assert_eq!(s.pop(), Some(10));
        assert_eq!(s.pop(), None);
        assert!(s.is_empty());
    }

    #[test]
    fn test_string_stack() {
        let mut s: Stack<String> = Container::empty();
        s.push("hello".into());
        s.push("world".into());
        assert_eq!(s.pop(), Some("world".to_string()));
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        assert!((p1.combine(&p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_string_combine() {
        let s1 = "foo".to_string();
        let s2 = "bar".to_string();
        assert_eq!(s1.combine(&s2), "foobar");
    }

    #[test]
    fn test_transformers() {
        assert_eq!(apply_transform(&StringLength, "test".into()), 4);
        assert_eq!(apply_transform(&Doubler, 5), 10);
    }
}
