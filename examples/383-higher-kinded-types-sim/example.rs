// Simulating Higher-Kinded Types with GATs in Rust
// GATs stabilized in Rust 1.65

trait Functor {
    type Unwrapped;
    type Mapped<B>; // GAT: the "higher-kinded" part

    fn map<B, F: Fn(Self::Unwrapped) -> B>(self, f: F) -> Self::Mapped<B>;
}

impl<A> Functor for Option<A> {
    type Unwrapped = A;
    type Mapped<B> = Option<B>;

    fn map<B, F: Fn(A) -> B>(self, f: F) -> Option<B> {
        self.map(f) // delegate to Option::map
    }
}

impl<A> Functor for Vec<A> {
    type Unwrapped = A;
    type Mapped<B> = Vec<B>;

    fn map<B, F: Fn(A) -> B>(self, f: F) -> Vec<B> {
        self.into_iter().map(f).collect()
    }
}

impl<A, E> Functor for Result<A, E> {
    type Unwrapped = A;
    type Mapped<B> = Result<B, E>;

    fn map<B, F: Fn(A) -> B>(self, f: F) -> Result<B, E> {
        self.map(f)
    }
}

fn main() {
    let opt: Option<i32> = Some(21);
    let doubled = Functor::map(opt, |x| x * 2);
    println!("Option map: {:?}", doubled);

    let v: Vec<i32> = vec![1, 2, 3, 4];
    let tripled = Functor::map(v, |x| x * 3);
    println!("Vec map: {:?}", tripled);

    let r: Result<i32, &str> = Ok(10);
    let stringified = Functor::map(r, |x| x.to_string());
    println!("Result map: {:?}", stringified);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_functor() {
        let x: Option<i32> = Some(5);
        assert_eq!(Functor::map(x, |n| n * 2), Some(10));
        let none: Option<i32> = None;
        assert_eq!(Functor::map(none, |n: i32| n * 2), None);
    }

    #[test]
    fn test_vec_functor() {
        let v = vec![1i32, 2, 3];
        assert_eq!(Functor::map(v, |x| x + 1), vec![2, 3, 4]);
    }
}
