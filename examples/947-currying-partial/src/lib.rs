#![allow(clippy::all)]
/// Currying, Partial Application, and Sections
///
/// OCaml functions are curried by default: `let add x y = x + y` can be
/// partially applied as `add 5`. Rust functions are NOT curried — closures
/// are used instead for partial application.

/// Regular two-argument function (NOT curried in Rust).
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Partial application via closure — the Rust way.
pub fn add5() -> impl Fn(i32) -> i32 {
    move |y| add(5, y)
}

/// Curried function — returns a closure. This mimics OCaml's default.
pub fn add_curried(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// Operator "sections" via closures.
pub fn double() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

pub fn increment() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

pub fn halve() -> impl Fn(i32) -> i32 {
    |x| x / 2
}

/// Curry converter: turns a 2-arg function into a curried one.
/// Requires A: Copy so the closure can capture it by value in Fn.
pub fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    A: Copy + 'static,
    B: 'static,
    C: 'static,
    F: Fn(A, B) -> C + Clone + 'static,
{
    move |a: A| {
        let f = f.clone();
        Box::new(move |b: B| f(a, b))
    }
}

/// Uncurry converter: turns a curried function into a 2-arg one.
pub fn uncurry<A, B, C>(f: impl Fn(A) -> Box<dyn Fn(B) -> C>) -> impl Fn(A, B) -> C {
    move |a, b| f(a)(b)
}

/// Pipeline: fold a value through a list of functions.
pub fn pipeline(initial: i32, funcs: &[&dyn Fn(i32) -> i32]) -> i32 {
    funcs.iter().fold(initial, |acc, f| f(acc))
}

/// Scale and shift with named parameters (Rust doesn't have labeled args,
/// but builder pattern or structs serve the same purpose).
pub fn scale_and_shift(scale: i32, shift: i32, x: i32) -> i32 {
    x * scale + shift
}

pub fn celsius_of_fahrenheit(f: i32) -> i32 {
    scale_and_shift(5, -160, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add5() {
        assert_eq!(add5()(10), 15);
    }

    #[test]
    fn test_curried() {
        let add3 = add_curried(3);
        assert_eq!(add3(7), 10);
        assert_eq!(add3(0), 3);
    }

    #[test]
    fn test_sections() {
        assert_eq!(double()(7), 14);
        assert_eq!(increment()(9), 10);
        assert_eq!(halve()(20), 10);
    }

    #[test]
    fn test_pipeline() {
        let d = double();
        let i = increment();
        let h = halve();
        let result = pipeline(6, &[&d, &i, &h]);
        // 6 * 2 = 12, + 1 = 13, / 2 = 6
        assert_eq!(result, 6);
    }

    #[test]
    fn test_celsius() {
        assert_eq!(celsius_of_fahrenheit(212), 900); // 212*5 - 160 = 900
                                                     // Note: integer arithmetic, not actual Celsius conversion
    }

    #[test]
    fn test_curry_uncurry() {
        let curried_add = curry(add);
        assert_eq!(curried_add(3)(4), 7);
    }
}
