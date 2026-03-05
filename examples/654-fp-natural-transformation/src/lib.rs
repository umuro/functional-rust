//! # Natural Transformations
//!
//! A natural transformation η: F => G is a family of morphisms η_A: F(A) -> G(A)
//! that commutes with functor operations: η_B ∘ F(f) = G(f) ∘ η_A

/// Natural transformation from Option to Vec
pub fn option_to_vec<A>(opt: Option<A>) -> Vec<A> {
    match opt {
        Some(a) => vec![a],
        None => vec![],
    }
}

/// Natural transformation from Vec to Option (head)
pub fn vec_to_option<A>(vec: Vec<A>) -> Option<A> {
    vec.into_iter().next()
}

/// Natural transformation from Result to Option
pub fn result_to_option<A, E>(res: Result<A, E>) -> Option<A> {
    res.ok()
}

/// Natural transformation from Option to Result
pub fn option_to_result<A, E>(opt: Option<A>, err: E) -> Result<A, E> {
    opt.ok_or(err)
}

// Approach 2: Iterator transformations
/// Natural transformation: Iterator -> Vec
pub fn iter_to_vec<I: Iterator>(iter: I) -> Vec<I::Item> {
    iter.collect()
}

/// Natural transformation: Vec -> Iterator  
pub fn vec_to_iter<A>(vec: Vec<A>) -> impl Iterator<Item = A> {
    vec.into_iter()
}

// Approach 3: Functor wrapper with transformation
pub struct Identity<A>(pub A);

impl<A> Identity<A> {
    pub fn run(self) -> A { self.0 }
}

/// Id => Option (natural transformation)
pub fn id_to_option<A>(id: Identity<A>) -> Option<A> {
    Some(id.0)
}

/// Option => Id (partial, requires default)
pub fn option_to_id<A>(opt: Option<A>, default: A) -> Identity<A> {
    Identity(opt.unwrap_or(default))
}

// Practical example: safe head as natural transformation
pub trait SafeHead {
    type Item;
    fn safe_head(self) -> Option<Self::Item>;
}

impl<A> SafeHead for Vec<A> {
    type Item = A;
    fn safe_head(mut self) -> Option<A> {
        if self.is_empty() { None } else { Some(self.remove(0)) }
    }
}

impl<A: Clone, const N: usize> SafeHead for [A; N] {
    type Item = A;
    fn safe_head(self) -> Option<A> {
        if N == 0 { None } else { Some(self[0].clone()) }
    }
}

/// Verify naturality: η_B ∘ F(f) = G(f) ∘ η_A
pub fn verify_naturality_option_vec<A: Clone + PartialEq, B: PartialEq>(
    a: A,
    f: impl Fn(A) -> B + Clone,
) -> bool {
    let opt = Some(a.clone());
    
    // Route 1: F(f) then η: map then convert
    let route1: Vec<B> = option_to_vec(opt.clone().map(f.clone()));
    
    // Route 2: η then G(f): convert then map
    let route2: Vec<B> = option_to_vec(opt).into_iter().map(f).collect();
    
    route1 == route2
}

/// Compose natural transformations
pub fn compose_nat<A, F, G>(
    first: impl Fn(A) -> F,
    second: impl Fn(F) -> G,
) -> impl Fn(A) -> G {
    move |a| second(first(a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_to_vec_some() {
        assert_eq!(option_to_vec(Some(42)), vec![42]);
    }

    #[test]
    fn test_option_to_vec_none() {
        assert_eq!(option_to_vec::<i32>(None), vec![]);
    }

    #[test]
    fn test_vec_to_option_nonempty() {
        assert_eq!(vec_to_option(vec![1, 2, 3]), Some(1));
    }

    #[test]
    fn test_vec_to_option_empty() {
        assert_eq!(vec_to_option::<i32>(vec![]), None);
    }

    #[test]
    fn test_result_to_option() {
        let ok: Result<i32, &str> = Ok(42);
        let err: Result<i32, &str> = Err("error");
        
        assert_eq!(result_to_option(ok), Some(42));
        assert_eq!(result_to_option(err), None);
    }

    #[test]
    fn test_option_to_result() {
        assert_eq!(option_to_result(Some(42), "err"), Ok(42));
        assert_eq!(option_to_result(None::<i32>, "err"), Err("err"));
    }

    #[test]
    fn test_naturality() {
        assert!(verify_naturality_option_vec(10, |x| x * 2));
        assert!(verify_naturality_option_vec("hello", |s: &str| s.len()));
    }

    #[test]
    fn test_safe_head_vec() {
        assert_eq!(vec![1, 2, 3].safe_head(), Some(1));
        assert_eq!(Vec::<i32>::new().safe_head(), None);
    }

    #[test]
    fn test_identity() {
        let id = Identity(42);
        assert_eq!(id.run(), 42);
    }

    #[test]
    fn test_compose_nat() {
        let composed = compose_nat(
            |x: i32| Some(x),
            |opt: Option<i32>| option_to_vec(opt)
        );
        assert_eq!(composed(42), vec![42]);
    }

    #[test]
    fn test_id_to_option() {
        assert_eq!(id_to_option(Identity(42)), Some(42));
    }
}
