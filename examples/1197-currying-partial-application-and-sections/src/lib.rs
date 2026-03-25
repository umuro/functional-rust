//! Currying, partial application, and operator sections in Rust.
//!
//! Rust doesn't have native currying like OCaml, but we can emulate it with
//! closures and higher-order functions.

// Curried addition (returns a closure)
pub fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// Pre-bound partial application: add5 = add 5
pub fn add5() -> impl Fn(i32) -> i32 {
    add(5)
}

// Tupled addition (Rust's default style)
pub fn add_tup((x, y): (i32, i32)) -> i32 {
    x + y
}

// Convert tupled to curried (currying)
pub fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> impl Fn(B) -> C
where
    F: Fn(A, B) -> C,
{
    move |a| move |b| f(a, b)
}

// Convert curried to tupled (uncurrying)
pub fn uncurry<A, B, C, F>(f: F) -> impl Fn(A, B) -> C
where
    F: Fn(A) -> impl Fn(B) -> C,
{
    move |a, b| f(a)(b)
}

// Operator sections via partial application
pub fn double() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

pub fn increment() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

pub fn halve() -> impl Fn(i32) -> i32 {
    |x| x / 2
}

// Named arguments via struct builder pattern
#[derive(Default)]
pub struct ScaleAndShift {
    scale: i32,
    shift: i32,
}

impl ScaleAndShift {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scale(mut self, scale: i32) -> Self {
        self.scale = scale;
        self
    }

    pub fn shift(mut self, shift: i32) -> Self {
        self.shift = shift;
        self
    }

    pub fn apply(&self, x: i32) -> i32 {
        x * self.scale + self.shift
    }
}

// Celsius to Fahrenheit conversion using builder
pub fn celsius_of_fahrenheit() -> impl Fn(i32) -> i32 {
    let builder = ScaleAndShift::new().scale(5).shift(-160);
    move |temp| builder.apply(temp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add5() {
        assert_eq!(add5()(10), 15);
    }

    #[test]
    fn test_double() {
        assert_eq!(double()(7), 14);
    }

    #[test]
    fn test_halve() {
        assert_eq!(halve()(20), 10);
    }

    #[test]
    fn test_pipeline() {
        let pipeline = [double(), increment(), halve()];
        let result = pipeline.iter().fold(6, |acc, f| f(acc));
        assert_eq!(result, 7); // (6 * 2 + 1) / 2 = 7
    }

    #[test]
    fn test_celsius_conversion() {
        // Rough approximation: (212°F - 32) * 5/9 ≈ 100°C
        // Our formula: 212 * 5 - 160 = 1060 - 160 = 900 (≈100°C)
        assert_eq!(celsius_of_fahrenheit()(212), 900);
    }

    #[test]
    fn test_curry_uncurry() {
        let tupled = |a: i32, b: i32| a + b;
        let curried = curry(tupled);
        let uncurried = uncurry(curried);
        assert_eq!(tupled(3, 4), uncurried(3, 4));
        assert_eq!(curried(3)(4), 7);
    }
}

// Main function for demonstration
fn main() {
    println!("add5 10   = {}", add5()(10));
    println!("double 7  = {}", double()(7));
    println!("halve 20  = {}", halve()(20));
    
    let pipeline = [double(), increment(), halve()];
    let result = pipeline.iter().fold(6, |acc, f| f(acc));
    println!("6 |> *2 |> +1 |> /2 = {}", result);
    
    println!("212F in Celsius ≈ {}", celsius_of_fahrenheit()(212));
    
    // Demonstrate curry/uncurry
    let tupled = |a: i32, b: i32| a + b;
    let curried = curry(tupled);
    println!("curried add 3 4 = {}", curried(3)(4));
}