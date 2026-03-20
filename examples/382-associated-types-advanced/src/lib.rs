#![allow(clippy::all)]
//! Associated Types vs Type Parameters
//!
//! When to use each for cleaner APIs.

/// Container trait with associated type
pub trait Container {
    type Item;
    fn empty() -> Self;
    fn add(&mut self, item: Self::Item);
    fn to_vec(&self) -> Vec<Self::Item>
    where
        Self::Item: Clone;
}

/// Type parameter trait allows multiple impls
pub trait ConvertTo<T> {
    fn convert(&self) -> T;
}

pub struct Stack<T>(Vec<T>);

impl<T: Clone> Container for Stack<T> {
    type Item = T;
    fn empty() -> Self {
        Stack(vec![])
    }
    fn add(&mut self, item: T) {
        self.0.push(item);
    }
    fn to_vec(&self) -> Vec<T> {
        self.0.clone()
    }
}

pub struct Wrapper(pub i32);

impl ConvertTo<String> for Wrapper {
    fn convert(&self) -> String {
        self.0.to_string()
    }
}

impl ConvertTo<f64> for Wrapper {
    fn convert(&self) -> f64 {
        self.0 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container() {
        let mut s = Stack::<i32>::empty();
        s.add(1);
        s.add(2);
        s.add(3);
        assert_eq!(s.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_convert_to_string() {
        let w = Wrapper(42);
        let s: String = w.convert();
        assert_eq!(s, "42");
    }

    #[test]
    fn test_convert_to_f64() {
        let w = Wrapper(42);
        let f: f64 = w.convert();
        assert_eq!(f, 42.0);
    }

    #[test]
    fn test_multiple_impls() {
        let w = Wrapper(10);
        assert_eq!(ConvertTo::<String>::convert(&w), "10");
        assert_eq!(ConvertTo::<f64>::convert(&w), 10.0);
    }
}
