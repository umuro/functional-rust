// Example 226: Category Basics
// A category has objects, morphisms, composition, and identity.
// In Rust: types = objects, functions = morphisms.

// === Approach 1: Simple compose and identity functions ===

fn identity<A>(a: A) -> A {
    a
}

fn compose<A, B, C>(f: impl Fn(B) -> C, g: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |a| f(g(a))
}

// === Approach 2: Category trait (explicit abstraction) ===

trait Category {
    type Obj;
    // We represent morphisms as boxed closures for flexibility
    fn id() -> Box<dyn Fn(Self::Obj) -> Self::Obj>;
    fn compose(
        f: Box<dyn Fn(Self::Obj) -> Self::Obj>,
        g: Box<dyn Fn(Self::Obj) -> Self::Obj>,
    ) -> Box<dyn Fn(Self::Obj) -> Self::Obj>;
}

struct FnCategoryI32;

impl Category for FnCategoryI32 {
    type Obj = i32;

    fn id() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x)
    }

    fn compose(f: Box<dyn Fn(i32) -> i32>, g: Box<dyn Fn(i32) -> i32>) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| f(g(x)))
    }
}

// === Approach 3: Kleisli category (a -> Option<b>) ===

fn kleisli_id<A>(a: A) -> Option<A> {
    Some(a)
}

fn kleisli_compose<A, B, C>(
    f: impl Fn(B) -> Option<C> + 'static,
    g: impl Fn(A) -> Option<B> + 'static,
) -> impl Fn(A) -> Option<C> {
    move |a| g(a).and_then(|b| f(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_basic() {
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        assert_eq!(compose(f, g)(5), 11);
    }

    #[test]
    fn test_identity_laws() {
        let f = |x: i32| x + 1;
        assert_eq!(compose(identity, f)(10), f(10));
        assert_eq!(compose(f, identity)(10), f(10));
    }

    #[test]
    fn test_associativity() {
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        let h = |x: i32| x - 3;
        assert_eq!(compose(compose(f, g), h)(7), compose(f, compose(g, h))(7));
    }

    #[test]
    fn test_kleisli_compose() {
        let safe_div = |x: i32| -> Option<i32> {
            if x == 0 {
                None
            } else {
                Some(100 / x)
            }
        };
        let safe_succ = |x: i32| -> Option<i32> { Some(x + 1) };
        let k = kleisli_compose(safe_succ, safe_div);
        assert_eq!(k(5), Some(21));
        assert_eq!(k(0), None);
    }
}
