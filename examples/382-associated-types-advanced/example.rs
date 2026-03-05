// Associated types vs type parameters in Rust
use std::fmt::Display;

// Associated type: one impl per type — cleaner call sites
trait Container {
    type Item;  // associated type
    fn empty() -> Self;
    fn add(self, item: Self::Item) -> Self;
    fn to_vec(&self) -> Vec<Self::Item> where Self::Item: Clone;
}

// Type parameter: allows multiple impls for same type
trait ConvertTo<T> {
    fn convert(&self) -> T;
}

struct Stack<T>(Vec<T>);

impl<T: Clone> Container for Stack<T> {
    type Item = T;
    fn empty() -> Self { Stack(vec![]) }
    fn add(mut self, item: T) -> Self { self.0.push(item); self }
    fn to_vec(&self) -> Vec<T> { self.0.clone() }
}

// Multiple ConvertTo impls for the same type
struct Wrapper(i32);

impl ConvertTo<String> for Wrapper {
    fn convert(&self) -> String { self.0.to_string() }
}

impl ConvertTo<f64> for Wrapper {
    fn convert(&self) -> f64 { self.0 as f64 }
}

fn print_all<C: Container>(c: &C) where C::Item: Display + Clone {
    for item in c.to_vec() {
        print!("{} ", item);
    }
    println!();
}

fn main() {
    let s = Stack::<i32>::empty().add(1).add(2).add(3);
    print_all(&s);

    let w = Wrapper(42);
    let as_string: String = w.convert();
    let as_float: f64 = w.convert();
    println!("As string: {}, as float: {}", as_string, as_float);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container() {
        let s = Stack::<i32>::empty().add(10).add(20);
        assert_eq!(s.to_vec(), vec![10, 20]);
    }

    #[test]
    fn test_convert_to() {
        let w = Wrapper(7);
        assert_eq!(ConvertTo::<String>::convert(&w), "7");
        assert_eq!((ConvertTo::<f64>::convert(&w) - 7.0).abs() < 1e-9, true);
    }
}
