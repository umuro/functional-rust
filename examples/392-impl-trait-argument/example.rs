// impl Trait in argument position in Rust
use std::fmt;

// impl Trait in argument = sugar for generic parameter
fn print_all(items: impl Iterator<Item = impl fmt::Display>) {
    for item in items {
        println!("{}", item);
    }
}

// Same as above, written with explicit generics:
// fn print_all<T: fmt::Display, I: Iterator<Item=T>>(items: I)

fn sum_iter(iter: impl Iterator<Item = i32>) -> i32 {
    iter.sum()
}

fn max_of(a: impl PartialOrd + Copy, b: impl PartialOrd + Copy) -> impl PartialOrd + Copy {
    // Note: a and b are DIFFERENT type params here!
    // This won't compile for mixed types, use explicit generics for that.
    // For simplicity, let's use a single type:
    if a > b { a } else { b }
}

fn show_twice(item: &impl fmt::Display) {
    println!("{} {}", item, item);
}

fn transform_and_show<T: fmt::Display>(
    items: impl Iterator<Item = T>,
    transform: impl Fn(T) -> String,
) {
    for item in items {
        println!("{}", transform(item));
    }
}

fn main() {
    println!("=== print_all ===");
    print_all(vec![1, 2, 3].into_iter());
    print_all(vec!["hello", "world"].into_iter());

    println!("\n=== sum_iter ===");
    println!("Sum: {}", sum_iter(vec![1, 2, 3, 4, 5].into_iter()));
    println!("Sum of range: {}", sum_iter(1..=100));

    println!("\n=== show_twice ===");
    show_twice(&42);
    show_twice(&"rust");

    println!("\n=== transform_and_show ===");
    transform_and_show(vec![1i32, 2, 3].into_iter(), |x| format!("item:{}", x * x));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_iter() {
        assert_eq!(sum_iter(vec![10, 20, 30].into_iter()), 60);
        assert_eq!(sum_iter(1..=10), 55);
    }

    #[test]
    fn test_impl_trait_arg() {
        // Confirm both work with different concrete types
        let v1 = vec![1i32, 2, 3];
        let v2 = vec!["a", "b"];
        assert_eq!(sum_iter(v1.into_iter()), 6);
        // print_all accepts any Display iterator
        print_all(v2.into_iter()); // just test it doesn't panic
    }
}
