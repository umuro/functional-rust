// Foldable in Rust

trait Foldable {
    type Item;
    fn fold_left<B, F>(self, init: B, f: F) -> B
    where F: FnMut(B, Self::Item) -> B;
}

impl<T> Foldable for Vec<T> {
    type Item = T;
    fn fold_left<B, F>(self, init: B, f: F) -> B
    where F: FnMut(B, T) -> B {
        self.into_iter().fold(init, f)
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let sum = nums.fold_left(0, |acc, x| acc + x);
    println!("Sum: {}", sum);
    
    let words = vec!["hello", "world"];
    let concat = words.fold_left(String::new(), |acc, x| acc + x + " ");
    println!("Concat: {}", concat.trim());
}
