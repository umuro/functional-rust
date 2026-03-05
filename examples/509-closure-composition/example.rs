//! # 509. Composing Functions and Closures
//! Building complex transformations from simple composed pieces.

/// Compose two functions: apply g first, then f
/// compose(f, g)(x) == f(g(x))
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

/// Pipe: apply f first, then g (left-to-right composition)
fn pipe<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

/// Build a pipeline from a Vec of boxed transformations
fn make_pipeline<T>(transforms: Vec<Box<dyn Fn(T) -> T>>) -> impl Fn(T) -> T {
    move |x| transforms.iter().fold(x, |acc, f| f(acc))
}

/// Trait extension for composable transforms
trait Composable<B>: Sized {
    fn then_apply<C, F: Fn(B) -> C>(self, other: F) -> impl Fn(i32) -> C
    where
        Self: Fn(i32) -> B;
}

/// A builder that accumulates transformations
struct Pipeline<T> {
    steps: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T: 'static> Pipeline<T> {
    fn new() -> Self { Pipeline { steps: Vec::new() } }

    fn then(mut self, f: impl Fn(T) -> T + 'static) -> Self {
        self.steps.push(Box::new(f));
        self
    }

    fn run(self) -> impl Fn(T) -> T {
        make_pipeline(self.steps)
    }
}

fn main() {
    let double = |x: i32| x * 2;
    let inc    = |x: i32| x + 1;
    let square = |x: i32| x * x;

    // compose: right-to-left (mathematical notation)
    let double_then_inc = compose(inc, double); // inc(double(x))
    println!("double_then_inc(5) = {}", double_then_inc(5)); // 11

    // pipe: left-to-right (data flow notation)
    let process = pipe(pipe(double, inc), square); // ((x*2)+1)^2
    println!("double|inc|square(3) = {}", process(3)); // 49

    // Manual chain
    let chained = compose(square, compose(inc, double));
    println!("chained(3) = {}", chained(3)); // 49

    // Multi-step pipeline builder
    let pipeline = Pipeline::new()
        .then(|x: i32| x * 2)
        .then(|x| x + 1)
        .then(|x| x * x)
        .then(|x| x - 1)
        .run();
    println!("pipeline(3) = {}", pipeline(3)); // ((3*2)+1)^2 - 1 = 48

    // Compose with different types
    let to_string_len = compose(|s: String| s.len(), |x: i32| x.to_string());
    println!("digits in 12345: {}", to_string_len(12345));

    // Point-free style: compose a chain
    let transforms: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(double),
        Box::new(inc),
        Box::new(square),
    ];
    let pipeline2 = make_pipeline(transforms);
    println!("pipeline2(2) = {}", pipeline2(2)); // ((2*2)+1)^2 = 25
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose() {
        let f = compose(|x: i32| x + 1, |x| x * 2);
        assert_eq!(f(5), 11); // (5*2)+1
    }

    #[test]
    fn test_pipe() {
        let f = pipe(|x: i32| x * 2, |x| x + 1);
        assert_eq!(f(5), 11); // (5*2)+1
    }

    #[test]
    fn test_pipeline_builder() {
        let p = Pipeline::new()
            .then(|x: i32| x + 1)
            .then(|x| x * 3)
            .run();
        assert_eq!(p(4), 15); // (4+1)*3
    }

    #[test]
    fn test_identity_compose() {
        let f = compose(|x: i32| x, |x| x);
        assert_eq!(f(42), 42);
    }
}
