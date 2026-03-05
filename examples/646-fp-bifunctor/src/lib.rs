//! # Bifunctor
//!
//! A bifunctor is a functor over two type parameters.
//! It provides `bimap` to map over both simultaneously, and `first`/`second` individually.

/// Trait for bifunctors
pub trait Bifunctor<A, B> {
    type Output<C, D>;
    
    fn bimap<C, D, F, G>(self, f: F, g: G) -> Self::Output<C, D>
    where
        F: FnOnce(A) -> C,
        G: FnOnce(B) -> D;
    
    fn first<C, F>(self, f: F) -> Self::Output<C, B>
    where
        F: FnOnce(A) -> C,
        Self: Sized,
        B: Clone,
    {
        self.bimap(f, |b| b)
    }
    
    fn second<D, G>(self, g: G) -> Self::Output<A, D>
    where
        G: FnOnce(B) -> D,
        Self: Sized,
        A: Clone,
    {
        self.bimap(|a| a, g)
    }
}

// Approach 1: Tuple as Bifunctor
impl<A, B> Bifunctor<A, B> for (A, B) {
    type Output<C, D> = (C, D);
    
    fn bimap<C, D, F, G>(self, f: F, g: G) -> (C, D)
    where
        F: FnOnce(A) -> C,
        G: FnOnce(B) -> D,
    {
        (f(self.0), g(self.1))
    }
}

// Approach 2: Result as Bifunctor
impl<A, B> Bifunctor<A, B> for Result<B, A> {
    type Output<C, D> = Result<D, C>;
    
    fn bimap<C, D, F, G>(self, f: F, g: G) -> Result<D, C>
    where
        F: FnOnce(A) -> C,
        G: FnOnce(B) -> D,
    {
        match self {
            Ok(b) => Ok(g(b)),
            Err(a) => Err(f(a)),
        }
    }
}

// Approach 3: Either type (explicit bifunctor)
#[derive(Debug, Clone, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<A, B> Bifunctor<A, B> for Either<A, B> {
    type Output<C, D> = Either<C, D>;
    
    fn bimap<C, D, F, G>(self, f: F, g: G) -> Either<C, D>
    where
        F: FnOnce(A) -> C,
        G: FnOnce(B) -> D,
    {
        match self {
            Either::Left(a) => Either::Left(f(a)),
            Either::Right(b) => Either::Right(g(b)),
        }
    }
}

impl<L, R> Either<L, R> {
    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }
    
    pub fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }
    
    pub fn left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }
    
    pub fn right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }
}

/// Standalone bimap for tuples
pub fn bimap_tuple<A, B, C, D, F, G>(pair: (A, B), f: F, g: G) -> (C, D)
where
    F: FnOnce(A) -> C,
    G: FnOnce(B) -> D,
{
    (f(pair.0), g(pair.1))
}

/// Map first element of tuple
pub fn first<A, B, C, F>(pair: (A, B), f: F) -> (C, B)
where
    F: FnOnce(A) -> C,
{
    (f(pair.0), pair.1)
}

/// Map second element of tuple
pub fn second<A, B, D, G>(pair: (A, B), g: G) -> (A, D)
where
    G: FnOnce(B) -> D,
{
    (pair.0, g(pair.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_bimap() {
        let pair = (10, "hello");
        let result = pair.bimap(|x| x * 2, |s| s.len());
        assert_eq!(result, (20, 5));
    }

    #[test]
    fn test_tuple_first() {
        let pair = (10, "world");
        let result = first(pair, |x| x + 5);
        assert_eq!(result, (15, "world"));
    }

    #[test]
    fn test_tuple_second() {
        let pair = (10, "rust");
        let result = second(pair, |s: &str| s.to_uppercase());
        assert_eq!(result, (10, "RUST"));
    }

    #[test]
    fn test_result_bimap_ok() {
        let res: Result<i32, &str> = Ok(42);
        let mapped = res.bimap(|e| e.to_uppercase(), |x| x * 2);
        assert_eq!(mapped, Ok(84));
    }

    #[test]
    fn test_result_bimap_err() {
        let res: Result<i32, &str> = Err("error");
        let mapped = res.bimap(|e| e.len(), |x| x * 2);
        assert_eq!(mapped, Err(5));
    }

    #[test]
    fn test_either_bimap_left() {
        let e: Either<i32, &str> = Either::Left(10);
        let result = e.bimap(|x| x * 2, |s| s.len());
        assert_eq!(result, Either::Left(20));
    }

    #[test]
    fn test_either_bimap_right() {
        let e: Either<i32, &str> = Either::Right("hello");
        let result = e.bimap(|x| x * 2, |s| s.len());
        assert_eq!(result, Either::Right(5));
    }

    #[test]
    fn test_either_is_left_right() {
        let left: Either<i32, &str> = Either::Left(42);
        let right: Either<i32, &str> = Either::Right("hi");
        
        assert!(left.is_left());
        assert!(!left.is_right());
        assert!(!right.is_left());
        assert!(right.is_right());
    }

    #[test]
    fn test_either_extract() {
        let left: Either<i32, &str> = Either::Left(42);
        assert_eq!(left.clone().left(), Some(42));
        assert_eq!(left.right(), None);
    }

    #[test]
    fn test_bimap_laws_identity() {
        // bimap id id = id
        let pair = (1, 2);
        let result = pair.bimap(|x| x, |y| y);
        assert_eq!(result, (1, 2));
    }

    #[test]
    fn test_bimap_laws_composition() {
        // bimap (f . g) (h . i) = bimap f h . bimap g i
        let pair = (2, 3);
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        let h = |x: i32| x.to_string();
        let i = |x: i32| x - 1;
        
        let left = pair.bimap(|x| f(g(x)), |y| h(i(y)));
        let right = pair.bimap(g, i).bimap(f, h);
        
        assert_eq!(left, right);
    }
}
