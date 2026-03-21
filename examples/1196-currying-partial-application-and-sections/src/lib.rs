// Currying, Partial Application, and Sections
//
// OCaml is curried by default: every multi-arg function is really a chain of
// single-arg functions. Rust functions are NOT curried, but closures let us
// emulate all the same patterns explicitly.

// ---------------------------------------------------------------------------
// 1. Curried add via closure (OCaml: let add x y = x + y)
// ---------------------------------------------------------------------------

/// Returns a closure that adds `x` to its argument.
/// This is the Rust equivalent of OCaml's partially applied `add 5`.
pub fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// `add5` is a value — a partially applied function, like `let add5 = add 5`.
pub fn add5() -> impl Fn(i32) -> i32 {
    add(5)
}

// ---------------------------------------------------------------------------
// 2. Tupled add (OCaml: let add_tup (x, y) = x + y)
// ---------------------------------------------------------------------------

/// Takes a tuple — the non-default OCaml style; natural in Rust.
pub fn add_tup((x, y): (i32, i32)) -> i32 {
    x + y
}

// ---------------------------------------------------------------------------
// 3. curry / uncurry converters
// ---------------------------------------------------------------------------

/// Converts a tupled function into a curried (two-closure) form.
/// OCaml: `let curry f x y = f (x, y)`
pub fn curry<A, B, C>(f: impl Fn((A, B)) -> C) -> impl Fn(A) -> impl Fn(B) -> C
where
    A: Copy,
{
    move |x| move |y| f((x, y))
}

/// Converts a curried two-step function into a tupled form.
/// OCaml: `let uncurry f (x, y) = f x y`
pub fn uncurry<A, B, C>(f: impl Fn(A) -> impl Fn(B) -> C) -> impl Fn((A, B)) -> C {
    move |(x, y)| f(x)(y)
}

// ---------------------------------------------------------------------------
// 4. Operator sections via partial application
// OCaml: let double = ( * ) 2 / let increment = ( + ) 1
// ---------------------------------------------------------------------------

pub fn double() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

pub fn increment() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

/// OCaml: `Fun.flip ( / ) 2` — divides its argument by 2.
/// `flip` swaps the operand order so `flip (/) 2` means `\x -> x / 2`.
pub fn halve() -> impl Fn(i32) -> i32 {
    |x| x / 2
}

// ---------------------------------------------------------------------------
// 5. Named-argument partial application
// OCaml: let scale_and_shift ~scale ~shift x = x * scale + shift
//        let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)
// ---------------------------------------------------------------------------

/// Returns a closure that scales then shifts.
/// Rust has no labeled arguments, so we build the partial application manually.
pub fn scale_and_shift(scale: i32, shift: i32) -> impl Fn(i32) -> i32 {
    move |x| x * scale + shift
}

pub fn celsius_of_fahrenheit() -> impl Fn(i32) -> i32 {
    scale_and_shift(5, -160)
}

// ---------------------------------------------------------------------------
// 6. Function pipeline via fold (OCaml: List.fold_left)
// ---------------------------------------------------------------------------

/// Applies a list of `i32 -> i32` functions in sequence using fold.
/// Mirrors OCaml's `List.fold_left (fun acc f -> f acc) init pipeline`.
pub fn apply_pipeline(init: i32, fns: &[&dyn Fn(i32) -> i32]) -> i32 {
    fns.iter().fold(init, |acc, f| f(acc))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add5() {
        let f = add5();
        assert_eq!(f(10), 15);
        assert_eq!(f(0), 5);
    }

    #[test]
    fn test_add_tup() {
        assert_eq!(add_tup((3, 4)), 7);
        assert_eq!(add_tup((0, 0)), 0);
    }

    #[test]
    fn test_curry_uncurry_roundtrip() {
        // curry(add_tup) should behave like our curried add
        let curried = curry(add_tup);
        assert_eq!(curried(3)(4), 7);

        // uncurry(add) should behave like add_tup
        let uncc = uncurry(add);
        assert_eq!(uncc((3, 4)), 7);
    }

    #[test]
    fn test_operator_sections() {
        let d = double();
        let inc = increment();
        let h = halve();

        assert_eq!(d(7), 14);
        assert_eq!(inc(41), 42);
        assert_eq!(h(20), 10);
    }

    #[test]
    fn test_scale_and_shift_and_celsius() {
        // celsius = (F * 5 - 160) / ... but OCaml uses integer math:
        // scale_and_shift ~scale:5 ~shift:(-160) 212 = 212*5 + (-160) = 1060 - 160 = 900
        // Note: the OCaml example prints "≈ 100" but that would need /9; here we match OCaml output.
        let convert = celsius_of_fahrenheit();
        assert_eq!(convert(212), 900); // matches OCaml integer result: 212*5 - 160 = 900

        let f = scale_and_shift(2, 10);
        assert_eq!(f(5), 20); // 5*2 + 10 = 20
        assert_eq!(f(0), 10);
    }

    #[test]
    fn test_pipeline() {
        let d = double();
        let inc = increment();
        let h = halve();
        // 6 |> *2 = 12 |> +1 = 13 |> /2 = 6
        let result = apply_pipeline(6, &[&d, &inc, &h]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_add_partial_application_creates_independent_closures() {
        let add3 = add(3);
        let add7 = add(7);
        assert_eq!(add3(10), 13);
        assert_eq!(add7(10), 17);
    }
}
