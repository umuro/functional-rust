//! Pipe Operator Simulation
//!
//! Simulating OCaml's |> operator with a Pipe extension trait.

/// Extension trait to simulate the |> pipe operator.
pub trait Pipe: Sized {
    /// Apply f to self: self.pipe(f) == f(self)
    fn pipe<B, F: FnOnce(Self) -> B>(self, f: F) -> B {
        f(self)
    }

    /// pipe_ref: apply f to &self (doesn't consume)
    fn pipe_ref<B, F: FnOnce(&Self) -> B>(&self, f: F) -> B {
        f(self)
    }

    /// pipe_mut: apply f to &mut self
    fn pipe_mut<B, F: FnOnce(&mut Self) -> B>(&mut self, f: F) -> B {
        f(self)
    }
}

impl<T> Pipe for T {}

/// Some functions to use with pipe.
pub fn double(x: i32) -> i32 {
    x * 2
}

pub fn add1(x: i32) -> i32 {
    x + 1
}

pub fn square(x: i32) -> i32 {
    x * x
}

pub fn to_string(x: i32) -> String {
    x.to_string()
}

pub fn prefix(s: String) -> String {
    format!("Result: {}", s)
}

/// Compose two functions into one.
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |a| g(f(a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_single() {
        let result = 5.pipe(double);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_pipe_chain() {
        // 5 -> double -> add1 -> square
        // 5 -> 10 -> 11 -> 121
        let result = 5.pipe(double).pipe(add1).pipe(square);
        assert_eq!(result, 121);
    }

    #[test]
    fn test_pipe_type_change() {
        // 42 -> to_string -> prefix
        let result = 42.pipe(to_string).pipe(prefix);
        assert_eq!(result, "Result: 42");
    }

    #[test]
    fn test_pipe_with_closure() {
        let offset = 100;
        let result = 5.pipe(|x| x + offset).pipe(|x| x * 2);
        assert_eq!(result, 210); // (5 + 100) * 2
    }

    #[test]
    fn test_pipe_ref() {
        let v = vec![1, 2, 3, 4, 5];
        let sum = v.pipe_ref(|v| v.iter().sum::<i32>());
        assert_eq!(sum, 15);
        // v is still usable
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn test_pipe_mut() {
        let mut v = vec![1, 2, 3];
        v.pipe_mut(|v| v.push(4));
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_pipe_with_methods() {
        let result = "  hello  "
            .pipe(|s| s.trim())
            .pipe(|s| s.to_uppercase());
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_compose_basic() {
        let double_then_add1 = compose(double, add1);
        assert_eq!(double_then_add1(5), 11); // 5*2 + 1
    }

    #[test]
    fn test_compose_chain() {
        let pipeline = compose(compose(double, add1), square);
        assert_eq!(pipeline(5), 121); // ((5*2)+1)^2
    }

    #[test]
    fn test_pipe_vs_method_chain() {
        // Traditional method chain
        let v1: Vec<i32> = vec![1, 2, 3, 4, 5]
            .into_iter()
            .map(|x| x * 2)
            .filter(|x| *x > 4)
            .collect();

        // With pipe (collecting intermediate)
        let v2 = vec![1, 2, 3, 4, 5]
            .pipe(|v| v.into_iter().map(|x| x * 2).collect::<Vec<_>>())
            .pipe(|v| v.into_iter().filter(|x| *x > 4).collect::<Vec<_>>());

        assert_eq!(v1, v2);
    }
}
