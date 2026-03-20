#![allow(clippy::all)]
// Example 182: Existential Types
// Hide the concrete type while retaining ability to use it

use std::fmt;

// === Approach 1: Box<dyn Trait> — Rust's native existential ===

fn make_showables() -> Vec<Box<dyn fmt::Display>> {
    vec![
        Box::new(42),
        Box::new("hello"),
        Box::new(3.14),
        Box::new(true),
    ]
}

// === Approach 2: Custom existential with closure (like OCaml GADT) ===

struct Showable {
    show_fn: Box<dyn Fn() -> String>,
}

impl Showable {
    fn new<T: 'static>(value: T, to_string: fn(&T) -> String) -> Self {
        Showable {
            show_fn: Box::new(move || to_string(&value)),
        }
    }

    fn show(&self) -> String {
        (self.show_fn)()
    }
}

// === Approach 3: Existential with comparison ===

struct Comparable {
    compare_fn: Box<dyn Fn() -> std::cmp::Ordering>,
    describe: Box<dyn Fn() -> String>,
}

impl Comparable {
    fn new<T: Ord + fmt::Debug + 'static>(a: T, b: T) -> Self {
        let a2 = format!("{:?}", a);
        let b2 = format!("{:?}", b);
        Comparable {
            compare_fn: Box::new(move || a.cmp(&b)),
            describe: Box::new(move || format!("{} vs {}", a2, b2)),
        }
    }

    fn result(&self) -> &'static str {
        match (self.compare_fn)() {
            std::cmp::Ordering::Less => "less",
            std::cmp::Ordering::Equal => "equal",
            std::cmp::Ordering::Greater => "greater",
        }
    }
}

// Multi-trait existential using a custom trait
trait Printable: fmt::Display + fmt::Debug {}
impl<T: fmt::Display + fmt::Debug> Printable for T {}

fn print_all(items: &[Box<dyn Printable>]) -> Vec<String> {
    items.iter().map(|x| format!("{}", x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_dyn_display() {
        let items = make_showables();
        assert_eq!(format!("{}", items[0]), "42");
        assert_eq!(format!("{}", items[1]), "hello");
    }

    #[test]
    fn test_custom_showable() {
        let s = Showable::new(42, |x| x.to_string());
        assert_eq!(s.show(), "42");
        let s2 = Showable::new(String::from("world"), |x| x.clone());
        assert_eq!(s2.show(), "world");
    }

    #[test]
    fn test_comparable() {
        assert_eq!(Comparable::new(1, 2).result(), "less");
        assert_eq!(Comparable::new(5, 5).result(), "equal");
        assert_eq!(Comparable::new("z", "a").result(), "greater");
    }

    #[test]
    fn test_multi_trait() {
        let items: Vec<Box<dyn Printable>> = vec![Box::new(42), Box::new("hi")];
        let strs = print_all(&items);
        assert_eq!(strs, vec!["42", "hi"]);
    }
}
