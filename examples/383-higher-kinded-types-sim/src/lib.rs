//! Simulating Higher-Kinded Types with GATs

pub trait Functor {
    type Unwrapped;
    type Mapped<B>;
    fn fmap<B, F: Fn(Self::Unwrapped) -> B>(self, f: F) -> Self::Mapped<B>;
}

impl<A> Functor for Option<A> {
    type Unwrapped = A;
    type Mapped<B> = Option<B>;
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Option<B> { self.map(f) }
}

impl<A> Functor for Vec<A> {
    type Unwrapped = A;
    type Mapped<B> = Vec<B>;
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Vec<B> { self.into_iter().map(f).collect() }
}

impl<A, E> Functor for Result<A, E> {
    type Unwrapped = A;
    type Mapped<B> = Result<B, E>;
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Result<B, E> { self.map(f) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_functor() {
        let opt = Some(21);
        let doubled = opt.fmap(|x| x * 2);
        assert_eq!(doubled, Some(42));
    }

    #[test]
    fn test_vec_functor() {
        let v = vec![1, 2, 3];
        let tripled = v.fmap(|x| x * 3);
        assert_eq!(tripled, vec![3, 6, 9]);
    }

    #[test]
    fn test_result_functor() {
        let r: Result<i32, &str> = Ok(10);
        let s = r.fmap(|x| x.to_string());
        assert_eq!(s, Ok("10".to_string()));
    }

    #[test]
    fn test_none_functor() {
        let opt: Option<i32> = None;
        let result = opt.fmap(|x| x * 2);
        assert_eq!(result, None);
    }
}
