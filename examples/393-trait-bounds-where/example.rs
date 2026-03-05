// Complex where clause bounds in Rust
use std::fmt;
use std::ops::Add;

// Complex where clause: multiple bounds on associated types
fn print_sum<I>(iter: I) -> I::Item
where
    I: Iterator,
    I::Item: Add<Output = I::Item> + Default + fmt::Display + Copy,
{
    let sum = iter.fold(I::Item::default(), |acc, x| acc + x);
    println!("Sum: {}", sum);
    sum
}

// Where clause with lifetime bounds
fn longest_display<'a, T>(x: &'a T, y: &'a T) -> &'a T
where
    T: fmt::Display + PartialOrd,
{
    if x >= y {
        println!("Choosing: {}", x);
        x
    } else {
        println!("Choosing: {}", y);
        y
    }
}

// Multiple associated type bounds
trait Transformer {
    type Input: fmt::Debug;
    type Output: fmt::Display + Clone;
    fn transform(&self, input: Self::Input) -> Self::Output;
}

struct Doubler;
impl Transformer for Doubler {
    type Input = i32;
    type Output = i32;
    fn transform(&self, input: i32) -> i32 { input * 2 }
}

struct Stringifier;
impl Transformer for Stringifier {
    type Input = i32;
    type Output = String;
    fn transform(&self, input: i32) -> String { format!("#{}", input) }
}

fn apply_and_print<T>(transformer: &T, input: T::Input)
where
    T: Transformer,
    T::Input: fmt::Debug + Clone,
    T::Output: fmt::Display + Clone,
{
    let output = transformer.transform(input.clone());
    println!("transform({:?}) = {}", input, output);
}

fn main() {
    println!("=== print_sum ===");
    print_sum(vec![1i32, 2, 3, 4, 5].into_iter());
    print_sum(vec![1.0f64, 2.5, 3.5].into_iter());

    println!("\n=== longest_display ===");
    longest_display(&"hello", &"world");
    longest_display(&42i32, &17);

    println!("\n=== transformers ===");
    apply_and_print(&Doubler, 21);
    apply_and_print(&Stringifier, 42);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_sum() {
        let s = print_sum(vec![10i32, 20, 30].into_iter());
        assert_eq!(s, 60);
    }

    #[test]
    fn test_longest() {
        let result = longest_display(&5i32, &3);
        assert_eq!(*result, 5);
    }

    #[test]
    fn test_transformer() {
        let d = Doubler;
        assert_eq!(d.transform(7), 14);
        let s = Stringifier;
        assert_eq!(s.transform(5), "#5");
    }
}
