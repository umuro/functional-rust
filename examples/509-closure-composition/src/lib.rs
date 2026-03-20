#![allow(clippy::all)]
//! Function Composition
//!
//! Building complex transformations from simple composed pieces.

/// Compose two functions: apply g first, then f.
/// compose(f, g)(x) == f(g(x))
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

/// Pipe: apply f first, then g (left-to-right composition).
pub fn pipe<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

/// Build a pipeline from a Vec of boxed transformations.
pub fn make_pipeline<T>(transforms: Vec<Box<dyn Fn(T) -> T>>) -> impl Fn(T) -> T {
    move |x| transforms.iter().fold(x, |acc, f| f(acc))
}

/// A builder that accumulates transformations.
pub struct Pipeline<T> {
    steps: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T: 'static> Pipeline<T> {
    pub fn new() -> Self {
        Pipeline { steps: Vec::new() }
    }

    pub fn then(mut self, f: impl Fn(T) -> T + 'static) -> Self {
        self.steps.push(Box::new(f));
        self
    }

    pub fn run(self) -> impl Fn(T) -> T {
        make_pipeline(self.steps)
    }
}

impl<T: 'static> Default for Pipeline<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_basic() {
        let f = compose(|x: i32| x + 1, |x| x * 2);
        assert_eq!(f(5), 11); // (5*2)+1 = 11
    }

    #[test]
    fn test_pipe_basic() {
        let f = pipe(|x: i32| x * 2, |x| x + 1);
        assert_eq!(f(5), 11); // (5*2)+1 = 11
    }

    #[test]
    fn test_compose_vs_pipe_order() {
        let double = |x: i32| x * 2;
        let inc = |x: i32| x + 1;

        // compose: right-to-left (inc after double)
        let c = compose(inc, double);
        // pipe: left-to-right (double then inc)
        let p = pipe(double, inc);

        assert_eq!(c(3), p(3)); // both: (3*2)+1 = 7
    }

    #[test]
    fn test_pipeline_builder() {
        let p = Pipeline::new().then(|x: i32| x + 1).then(|x| x * 3).run();
        assert_eq!(p(4), 15); // (4+1)*3 = 15
    }

    #[test]
    fn test_identity_compose() {
        let f = compose(|x: i32| x, |x| x);
        assert_eq!(f(42), 42);
    }

    #[test]
    fn test_compose_type_change() {
        let to_string = compose(|s: String| s.len(), |x: i32| x.to_string());
        assert_eq!(to_string(12345), 5);
    }

    #[test]
    fn test_triple_compose() {
        let double = |x: i32| x * 2;
        let inc = |x: i32| x + 1;
        let square = |x: i32| x * x;

        let f = compose(square, compose(inc, double));
        assert_eq!(f(3), 49); // ((3*2)+1)^2 = 49
    }

    #[test]
    fn test_make_pipeline() {
        let transforms: Vec<Box<dyn Fn(i32) -> i32>> = vec![
            Box::new(|x| x * 2),
            Box::new(|x| x + 1),
            Box::new(|x| x * x),
        ];
        let pipeline = make_pipeline(transforms);
        assert_eq!(pipeline(2), 25); // ((2*2)+1)^2 = 25
    }
}
