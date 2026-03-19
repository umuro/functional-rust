//! # Functor Laws
//!
//! Functors must satisfy identity and composition laws.

/// Functor trait - map a function over a wrapped value.
pub trait Functor {
    type Inner;
    type Mapped<B>;
    fn fmap<B>(self, f: impl Fn(Self::Inner) -> B) -> Self::Mapped<B>;
}

impl<A> Functor for Option<A> {
    type Inner = A;
    type Mapped<B> = Option<B>;
    fn fmap<B>(self, f: impl Fn(A) -> B) -> Option<B> {
        self.map(f)
    }
}

impl<A> Functor for Vec<A> {
    type Inner = A;
    type Mapped<B> = Vec<B>;
    fn fmap<B>(self, f: impl Fn(A) -> B) -> Vec<B> {
        self.into_iter().map(f).collect()
    }
}

/// Identity law: fmap id == id
pub fn check_identity<F: Functor<Inner = A> + Clone + PartialEq, A: Clone>(fa: F) -> bool
where
    F::Mapped<A>: PartialEq<F>,
{
    let mapped = fa.clone().fmap(|x| x);
    mapped == fa
}

/// Composition law: fmap (g . f) == fmap g . fmap f
pub fn check_composition<A: Clone, B: Clone, C: PartialEq>(
    fa: Option<A>,
    f: impl Fn(A) -> B + Clone,
    g: impl Fn(B) -> C + Clone,
) -> bool {
    let left = fa.clone().map(|x| g(f(x)));
    let right = fa.map(f).map(g);
    left == right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_identity() {
        let opt = Some(42);
        let mapped = opt.clone().fmap(|x| x);
        assert_eq!(mapped, opt);
    }

    #[test]
    fn test_option_composition() {
        let opt = Some(5);
        let f = |x: i32| x * 2;
        let g = |x: i32| x + 1;
        assert!(check_composition(opt, f, g));
    }

    #[test]
    fn test_vec_functor() {
        let v = vec![1, 2, 3];
        let mapped = v.fmap(|x| x * 2);
        assert_eq!(mapped, vec![2, 4, 6]);
    }
}
