//! Currying Pattern
//!
//! Explicit currying via nested closures returning closures.

/// Curried add: add(x)(y) = x + y
pub fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// Curried multiply: mul(x)(y) = x * y
pub fn mul(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}

/// Three-argument curried clamp: clamp(lo)(hi)(x)
pub fn clamp(lo: i32) -> Box<dyn Fn(i32) -> Box<dyn Fn(i32) -> i32>> {
    Box::new(move |hi| Box::new(move |x| x.max(lo).min(hi)))
}

/// Convert a 2-arg uncurried function to curried form.
pub fn curry<A: Copy + 'static, B: Copy + 'static, C: 'static, F>(
    f: F,
) -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> C>>
where
    F: Fn(A, B) -> C + Copy + 'static,
{
    Box::new(move |a| Box::new(move |b| f(a, b)))
}

/// Convert a curried function back to uncurried.
pub fn uncurry<A, B, C, F, G>(f: F) -> impl Fn(A, B) -> C
where
    F: Fn(A) -> G,
    G: Fn(B) -> C,
{
    move |a, b| f(a)(b)
}

/// Flip argument order for a curried function.
pub fn flip<A: 'static, B: Copy + 'static, C: 'static, F, G>(
    f: F,
) -> Box<dyn Fn(B) -> Box<dyn Fn(A) -> C>>
where
    F: Fn(A) -> G + Clone + 'static,
    G: Fn(B) -> C + 'static,
{
    Box::new(move |b| {
        let f = f.clone();
        Box::new(move |a| f(a)(b))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curried_add() {
        assert_eq!(add(5)(3), 8);
        let add10 = add(10);
        assert_eq!(add10(0), 10);
        assert_eq!(add10(5), 15);
    }

    #[test]
    fn test_curried_mul() {
        assert_eq!(mul(6)(7), 42);
        let times3 = mul(3);
        assert_eq!(times3(4), 12);
    }

    #[test]
    fn test_curried_clamp() {
        let clamp_5_10 = clamp(5)(10);
        assert_eq!(clamp_5_10(7), 7);
        assert_eq!(clamp_5_10(2), 5);
        assert_eq!(clamp_5_10(15), 10);
    }

    #[test]
    fn test_curry() {
        let f = curry(|x: i32, y: i32| x * y);
        assert_eq!(f(6)(7), 42);
    }

    #[test]
    fn test_uncurry() {
        let g = uncurry(add);
        assert_eq!(g(3, 4), 7);
    }

    #[test]
    fn test_partial_application() {
        let add5 = add(5);
        let result: Vec<i32> = [1, 2, 3, 4, 5].iter().map(|&x| add5(x)).collect();
        assert_eq!(result, vec![6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_curried_chain() {
        // Chain curried calls
        assert_eq!(add(add(1)(2))(3), 6); // (1+2)+3
    }
}
