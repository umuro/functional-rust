//! # Functor Composition
//!
//! Functors compose: if F and G are functors, then F ∘ G is also a functor.
//! map_{F∘G} = map_F ∘ map_G

use std::marker::PhantomData;

/// Composed functor F<G<A>>
#[derive(Debug, Clone, PartialEq)]
pub struct Compose<F, G, A> {
    pub value: F,
    _phantom: PhantomData<(G, A)>,
}

impl<F, G, A> Compose<F, G, A> {
    pub fn new(value: F) -> Self {
        Compose { value, _phantom: PhantomData }
    }
}

// Approach 1: Option<Vec<A>>
pub type OptionVec<A> = Option<Vec<A>>;

pub fn map_option_vec<A, B, F>(ov: OptionVec<A>, f: F) -> OptionVec<B>
where
    F: Fn(A) -> B,
{
    ov.map(|v| v.into_iter().map(f).collect())
}

// Approach 2: Vec<Option<A>>
pub type VecOption<A> = Vec<Option<A>>;

pub fn map_vec_option<A, B, F>(vo: VecOption<A>, f: F) -> VecOption<B>
where
    F: Fn(A) -> B,
{
    vo.into_iter().map(|opt| opt.map(&f)).collect()
}

// Approach 3: Result<Vec<A>, E>
pub fn map_result_vec<A, B, E, F>(rv: Result<Vec<A>, E>, f: F) -> Result<Vec<B>, E>
where
    F: Fn(A) -> B,
{
    rv.map(|v| v.into_iter().map(f).collect())
}

// Nested option
pub type OptionOption<A> = Option<Option<A>>;

pub fn map_option_option<A, B, F>(oo: OptionOption<A>, f: F) -> OptionOption<B>
where
    F: Fn(A) -> B,
{
    oo.map(|inner| inner.map(f))
}

/// Generic composed functor mapping
pub fn compose_map<F, G, A, B, MapF, MapG, FA, GB>(
    outer_map: MapF,
    inner_map: MapG,
    composed: FA,
    f: impl Fn(A) -> B,
) -> GB
where
    MapF: FnOnce(FA, impl FnOnce(G) -> G) -> GB,
    MapG: Fn(G, &dyn Fn(A) -> B) -> G,
{
    // Simplified - real implementation needs higher-kinded types
    unimplemented!()
}

// Practical composed functors

/// Async result: Future<Result<A, E>> pattern
pub struct AsyncResult<A, E> {
    // In practice, would wrap a Future
    result: Result<A, E>,
}

impl<A, E> AsyncResult<A, E> {
    pub fn ok(a: A) -> Self { AsyncResult { result: Ok(a) } }
    pub fn err(e: E) -> Self { AsyncResult { result: Err(e) } }
    
    pub fn map<B, F: FnOnce(A) -> B>(self, f: F) -> AsyncResult<B, E> {
        AsyncResult { result: self.result.map(f) }
    }
    
    pub fn into_result(self) -> Result<A, E> {
        self.result
    }
}

/// Validation accumulator: Vec<Result<A, E>>
pub fn validate_all<A: Clone, E: Clone>(items: Vec<Result<A, E>>) -> Result<Vec<A>, Vec<E>> {
    let (oks, errs): (Vec<_>, Vec<_>) = items.into_iter()
        .map(|r| match r {
            Ok(a) => (Some(a), None),
            Err(e) => (None, Some(e)),
        })
        .unzip();
    
    let errors: Vec<E> = errs.into_iter().flatten().collect();
    if errors.is_empty() {
        Ok(oks.into_iter().flatten().collect())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_option_vec_some() {
        let ov = Some(vec![1, 2, 3]);
        let result = map_option_vec(ov, |x| x * 2);
        assert_eq!(result, Some(vec![2, 4, 6]));
    }

    #[test]
    fn test_map_option_vec_none() {
        let ov: OptionVec<i32> = None;
        let result = map_option_vec(ov, |x| x * 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_map_vec_option() {
        let vo = vec![Some(1), None, Some(3)];
        let result = map_vec_option(vo, |x| x * 2);
        assert_eq!(result, vec![Some(2), None, Some(6)]);
    }

    #[test]
    fn test_map_result_vec() {
        let rv: Result<Vec<i32>, &str> = Ok(vec![1, 2, 3]);
        let result = map_result_vec(rv, |x| x.to_string());
        assert_eq!(result, Ok(vec!["1".to_string(), "2".to_string(), "3".to_string()]));
    }

    #[test]
    fn test_map_option_option() {
        let oo = Some(Some(42));
        let result = map_option_option(oo, |x| x * 2);
        assert_eq!(result, Some(Some(84)));
    }

    #[test]
    fn test_async_result_map() {
        let ar = AsyncResult::ok(42);
        let result = ar.map(|x| x * 2).into_result();
        assert_eq!(result, Ok(84));
    }

    #[test]
    fn test_validate_all_success() {
        let items: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
        assert_eq!(validate_all(items), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_validate_all_errors() {
        let items: Vec<Result<i32, &str>> = vec![Ok(1), Err("e1"), Err("e2")];
        assert_eq!(validate_all(items), Err(vec!["e1", "e2"]));
    }

    #[test]
    fn test_functor_composition_law() {
        // map (f . g) = map f . map g
        let ov = Some(vec![1, 2, 3]);
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        
        // Composed: map (g . f)
        let r1 = map_option_vec(ov.clone(), |x| g(f(x)));
        
        // Sequential: map g . map f
        let r2 = map_option_vec(map_option_vec(ov, f), g);
        
        assert_eq!(r1, r2);
    }
}
