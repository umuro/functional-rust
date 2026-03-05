//! # Coproduct Types (Sum Types)
//!
//! Combine multiple types into a single sum type.

/// Either type - canonical two-way coproduct.
#[derive(Debug, Clone, PartialEq)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    pub fn is_left(&self) -> bool { matches!(self, Either::Left(_)) }
    pub fn is_right(&self) -> bool { matches!(self, Either::Right(_)) }
    
    pub fn map_left<C>(self, f: impl FnOnce(A) -> C) -> Either<C, B> {
        match self { Either::Left(a) => Either::Left(f(a)), Either::Right(b) => Either::Right(b) }
    }
    
    pub fn map_right<C>(self, f: impl FnOnce(B) -> C) -> Either<A, C> {
        match self { Either::Left(a) => Either::Left(a), Either::Right(b) => Either::Right(f(b)) }
    }
}

/// Three-way coproduct.
#[derive(Debug, Clone, PartialEq)]
pub enum Either3<A, B, C> {
    First(A), Second(B), Third(C),
}

/// Inject into coproduct.
pub fn left<A, B>(a: A) -> Either<A, B> { Either::Left(a) }
pub fn right<A, B>(b: B) -> Either<A, B> { Either::Right(b) }

/// Eliminate coproduct.
pub fn either<A, B, C>(e: Either<A, B>, f: impl FnOnce(A) -> C, g: impl FnOnce(B) -> C) -> C {
    match e { Either::Left(a) => f(a), Either::Right(b) => g(b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_right() {
        let l: Either<i32, &str> = left(42);
        let r: Either<i32, &str> = right("hello");
        assert!(l.is_left());
        assert!(r.is_right());
    }

    #[test]
    fn test_either() {
        let l: Either<i32, &str> = left(42);
        let result = either(l, |n| n * 2, |s| s.len() as i32);
        assert_eq!(result, 84);
    }

    #[test]
    fn test_map() {
        let l: Either<i32, i32> = left(5);
        let mapped = l.map_left(|n| n * 2);
        assert_eq!(mapped, Either::Left(10));
    }
}
