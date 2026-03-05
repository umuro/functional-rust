// Deref and deref coercions in Rust
use std::ops::{Deref, DerefMut};
use std::fmt;

// Custom smart pointer implementing Deref
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.0 }
}

impl<T: fmt::Display> fmt::Display for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "MyBox({})", self.0) }
}

// Functions accepting coerced types
fn use_str(s: &str) { println!("&str: '{}' (len={})", s, s.len()); }
fn use_slice(s: &[i32]) { println!("&[i32]: {:?} (len={})", s, s.len()); }
fn use_i32_ref(n: &i32) { println!("&i32: {}", n); }

fn main() {
    // Deref coercion: &String -> &str (automatic)
    let s = String::from("Hello, deref!");
    use_str(&s);              // &String coerces to &str

    // &Vec<T> -> &[T]
    let v = vec![1, 2, 3, 4, 5];
    use_slice(&v);            // &Vec<i32> coerces to &[i32]

    // &Box<T> -> &T
    let boxed = Box::new(42i32);
    use_i32_ref(&boxed);      // &Box<i32> coerces to &i32

    // Our custom MyBox
    let my_box = MyBox::new(String::from("custom"));
    use_str(&my_box);         // &MyBox<String> -> &String -> &str (two coercions!)

    // Manual deref
    let n = MyBox::new(5i32);
    assert_eq!(*n, 5);
    println!("Dereferenced: {}", *n);

    // Deref coercion chain: MyBox<String> -> String -> str
    let mb = MyBox::new(String::from("chain"));
    println!("Upper: {}", mb.to_uppercase()); // calls str method via coercion chain

    // Mutable deref
    let mut mb_mut = MyBox::new(vec![1, 2, 3]);
    mb_mut.push(4);  // deref_mut allows calling Vec methods
    println!("After push: {:?}", *mb_mut);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mybox_deref() {
        let b = MyBox::new(10i32);
        assert_eq!(*b, 10);
    }

    #[test]
    fn test_coercion_string() {
        let s = String::from("test");
        fn takes_str(x: &str) -> usize { x.len() }
        assert_eq!(takes_str(&s), 4); // coercion happens here
    }

    #[test]
    fn test_coercion_vec() {
        let v = vec![1i32, 2, 3];
        fn takes_slice(x: &[i32]) -> i32 { x.iter().sum() }
        assert_eq!(takes_slice(&v), 6);
    }
}
