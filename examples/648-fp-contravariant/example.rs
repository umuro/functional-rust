// Contravariant Functor in Rust

use std::cmp::Ordering;

struct Comparator<A> {
    compare: Box<dyn Fn(&A, &A) -> Ordering>,
}

impl<A> Comparator<A> {
    fn new<F: Fn(&A, &A) -> Ordering + 'static>(f: F) -> Self {
        Comparator { compare: Box::new(f) }
    }
    
    fn contramap<B, F>(self, f: F) -> Comparator<B>
    where F: Fn(&B) -> A + 'static, A: 'static {
        let cmp = self.compare;
        Comparator::new(move |b1, b2| cmp(&f(b1), &f(b2)))
    }
}

fn main() {
    // Sort strings by length using contramap
    let int_cmp = Comparator::new(|a: &i32, b: &i32| a.cmp(b));
    let by_len = int_cmp.contramap(|s: &String| s.len() as i32);
    
    let mut words = vec!["hello", "hi", "greetings"];
    words.sort_by(|a, b| by_len.compare(&a.to_string(), &b.to_string()));
    println!("Sorted by length: {:?}", words);
}
