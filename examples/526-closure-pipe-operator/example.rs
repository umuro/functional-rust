//! # 526. Pipe Operator Simulation
//! Simulating OCaml's |> operator with a Pipe extension trait.

/// Extension trait to simulate the |> pipe operator
trait Pipe: Sized {
    /// Apply f to self, left-to-right: self.pipe(f) == f(self)
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

fn double(x: i32) -> i32 { x * 2 }
fn add1(x: i32) -> i32 { x + 1 }
fn square(x: i32) -> i32 { x * x }

fn main() {
    // Basic pipe: 3 |> double |> add1 |> square
    let result = 3i32
        .pipe(double)
        .pipe(add1)
        .pipe(square);
    println!("3 |> double |> add1 |> square = {}", result); // (3*2+1)^2 = 49

    // Pipe with closures
    let offset = 10;
    let result2 = 5i32
        .pipe(|x| x + offset)
        .pipe(|x| x * 3)
        .pipe(|x| x.to_string());
    println!("5 + 10 * 3 as string = {:?}", result2);

    // Pipe with Vec transformations
    let sum_evens_tripled = vec![1, 2, 3, 4, 5, 6]
        .pipe(|v| v.into_iter().filter(|x| x % 2 == 0).collect::<Vec<_>>())
        .pipe(|v| v.into_iter().map(|x| x * 3).collect::<Vec<_>>())
        .pipe(|v| v.into_iter().sum::<i32>());
    println!("sum of tripled evens: {}", sum_evens_tripled);

    // pipe_ref for non-consuming inspection
    let data = vec![5, 3, 8, 1, 9, 2];
    let max = data
        .pipe_ref(|v| v.iter().max().copied());
    println!("max: {:?}", max);
    println!("data still owned: {:?}", data); // data not consumed

    // Multi-type pipe chain
    let word_lengths: Vec<usize> = "hello world rust programming"
        .pipe(|s| s.split_whitespace().collect::<Vec<_>>())
        .pipe(|words| words.iter().map(|w| w.len()).collect());
    println!("word lengths: {:?}", word_lengths);

    // Simulate OCaml pipeline
    let result3 = 42i32
        .pipe(|x| x.to_string())
        .pipe(|s| format!("The answer is: {}", s))
        .pipe(|s| s.to_uppercase());
    println!("{}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_pipe() {
        let result = 3i32.pipe(double).pipe(add1).pipe(square);
        assert_eq!(result, 49);
    }

    #[test]
    fn test_pipe_type_change() {
        let result = 42i32.pipe(|x| x.to_string());
        assert_eq!(result, "42");
    }

    #[test]
    fn test_pipe_ref_no_consume() {
        let v = vec![1, 2, 3];
        let sum = v.pipe_ref(|v| v.iter().sum::<i32>());
        assert_eq!(sum, 6);
        assert_eq!(v, [1, 2, 3]); // not consumed
    }

    #[test]
    fn test_pipe_closure_capture() {
        let n = 100;
        let result = 5i32.pipe(move |x| x + n);
        assert_eq!(result, 105);
    }
}
