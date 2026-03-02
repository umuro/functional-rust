// Currying and Partial Application in Rust

// Manual currying (not default in Rust)
fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn multiply(x: i32) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
    move |y| Box::new(move |z| x * y * z)
}

// Closure-based partial application
fn add_closure(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply_closure(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}

// Trait-based approach for better ergonomics
trait Curry2<A, B, R> {
    fn curry(self) -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> R>>;
}

impl<F, A: 'static, B: 'static, R: 'static> Curry2<A, B, R> for F
where
    F: Fn(A, B) -> R + 'static,
{
    fn curry(self) -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> R>> {
        Box::new(move |a| Box::new(move |b| self(a, b)))
    }
}

fn main() {
    // Manual currying
    let add5 = add(5);
    println!("add5(10) = {}", add5(10));
    
    let double = multiply(2);
    println!("double(3)(4) = {}", double(3)(4));
    
    // Closure-based partial application
    let numbers = vec![1, 2, 3, 4, 5];
    let plus10: Vec<_> = numbers.iter().map(|x| add_closure(10, *x)).collect();
    println!("Add 10: {:?}", plus10);
    
    let filtered: Vec<_> = numbers.iter().filter(|x| **x > 3).collect();
    println!("Greater than 3: {:?}", filtered);
    
    // Method chaining (Rust's answer to pipelines)
    let result = vec![5]
        .into_iter()
        .map(|x| add_closure(3, x))
        .map(|x| multiply_closure(2, 1, x))
        .next()
        .unwrap();
    println!("Pipeline: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currying() {
        let add5 = add(5);
        assert_eq!(add5(10), 15);
    }

    #[test]
    fn test_partial() {
        let double = multiply(2);
        assert_eq!(double(3)(4), 24);
    }
}
