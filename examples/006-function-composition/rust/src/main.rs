// Function Composition in Rust

// Composition trait
trait Compose<A, B, C> {
    fn compose<F, G>(self, g: G) -> Box<dyn Fn(A) -> C>
    where
        F: Fn(A) -> B + 'static,
        G: Fn(B) -> C + 'static,
        Self: Fn(A) -> B + 'static;
}

// Manual composition functions
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

// Example functions
fn double(x: i32) -> i32 {
    x * 2
}

fn add3(x: i32) -> i32 {
    x + 3
}

fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    // Manual composition
    let double_then_add3 = |x| add3(double(x));
    let add3_then_double = |x| double(add3(x));
    let complex = |x| add3(double(square(x)));
    
    println!("double_then_add3(5) = {}", double_then_add3(5));
    println!("add3_then_double(5) = {}", add3_then_double(5));
    println!("complex(4) = {}", complex(4));
    
    // Using compose function
    let composed = compose(add3, double);
    println!("composed(5) = {}", composed(5));
    
    // Iterator-based composition (idiomatic Rust)
    let numbers = vec![1, 2, 3, 4];
    let processed: Vec<_> = numbers
        .iter()
        .map(|&x| square(x))
        .map(|x| double(x))
        .collect();
    println!("Processed: {:?}", processed);
    
    // Chained (point-free style)
    let result: Vec<_> = vec![1, 2, 3]
        .into_iter()
        .map(square)
        .map(double)
        .map(add3)
        .collect();
    println!("Chained: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composition() {
        let f = compose(add3, double);
        assert_eq!(f(5), 13); // 5 * 2 + 3
    }

    #[test]
    fn test_complex() {
        let f = |x| add3(double(square(x)));
        assert_eq!(f(4), 35); // 4^2 * 2 + 3
    }
}
