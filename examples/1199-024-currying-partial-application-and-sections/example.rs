// example-1199-024: Currying, Partial Application, and Operator Sections
// Demonstrates how OCaml's automatic currying maps to explicit Rust closures.

fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn add_tupled((x, y): (i32, i32)) -> i32 {
    x + y
}

fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    F: Fn((A, B)) -> C + Copy + 'static,
    A: Copy + 'static,
    B: 'static,
    C: 'static,
{
    move |x: A| Box::new(move |y: B| f((x, y)))
}

fn uncurry<A, B, C>(
    f: impl Fn(A) -> Box<dyn Fn(B) -> C> + 'static,
) -> impl Fn((A, B)) -> C {
    move |(x, y)| f(x)(y)
}

fn multiply(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

fn divide_by(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x / n
}

fn scale_and_shift(scale: i32, shift: i32) -> impl Fn(i32) -> i32 {
    move |x| x * scale + shift
}

fn celsius_of_fahrenheit() -> impl Fn(i32) -> i32 {
    scale_and_shift(5, -160)
}

fn pipeline(initial: i32, transforms: &[&dyn Fn(i32) -> i32]) -> i32 {
    transforms.iter().fold(initial, |acc, f| f(acc))
}

fn main() {
    // Partial application
    let add5 = add(5);
    println!("add5 10   = {}", add5(10));

    // Operator sections via partial application
    let double = multiply(2);
    let increment = add(1);
    let halve = divide_by(2);

    println!("double 7  = {}", double(7));
    println!("halve 20  = {}", halve(20));

    // Pipeline: 6 → *2 → +1 → /2
    let result = pipeline(6, &[&double, &increment, &halve]);
    println!("6 |> *2 |> +1 |> /2 = {}", result);

    // Labeled-argument partial application (Rust: ordered curry)
    let f_to_c = celsius_of_fahrenheit();
    println!("212F in Celsius ≈ {}", f_to_c(212));

    // curry / uncurry round-trip
    let curried = curry(add_tupled);
    println!("curry(add_tupled)(3)(4) = {}", curried(3)(4));

    let tupled = uncurry(curry(add_tupled));
    println!("uncurry(curry(add_tupled))((5,6)) = {}", tupled((5, 6)));
}

/* Output:
   add5 10   = 15
   double 7  = 14
   halve 20  = 10
   6 |> *2 |> +1 |> /2 = 6
   212F in Celsius ≈ 900
   curry(add_tupled)(3)(4) = 7
   uncurry(curry(add_tupled))((5,6)) = 11
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_application_add5() {
        let add5 = add(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(-5), 0);
    }

    #[test]
    fn test_add_tupled() {
        assert_eq!(add_tupled((3, 4)), 7);
        assert_eq!(add_tupled((0, 0)), 0);
        assert_eq!(add_tupled((-1, 1)), 0);
        assert_eq!(add_tupled((-3, -4)), -7);
    }

    #[test]
    fn test_curry_converts_tupled_to_curried() {
        let curried = curry(add_tupled);
        assert_eq!(curried(3)(4), 7);
        assert_eq!(curried(0)(0), 0);
        assert_eq!(curried(-1)(1), 0);
    }

    #[test]
    fn test_uncurry_converts_back_to_tupled() {
        let tupled = uncurry(curry(add_tupled));
        assert_eq!(tupled((3, 4)), 7);
        assert_eq!(tupled((0, 0)), 0);
        assert_eq!(tupled((-5, 5)), 0);
        assert_eq!(tupled((10, 20)), 30);
    }

    #[test]
    fn test_operator_sections_double_increment_halve() {
        let double = multiply(2);
        let increment = add(1);
        let halve = divide_by(2);

        assert_eq!(double(7), 14);
        assert_eq!(increment(9), 10);
        assert_eq!(halve(20), 10);
        assert_eq!(halve(21), 10);
    }

    #[test]
    fn test_pipeline_fold() {
        let double = multiply(2);
        let increment = add(1);
        let halve = divide_by(2);

        let result = pipeline(6, &[&double, &increment, &halve]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_pipeline_empty() {
        assert_eq!(pipeline(42, &[]), 42);
    }

    #[test]
    fn test_celsius_of_fahrenheit_fixed_points() {
        let to_c = celsius_of_fahrenheit();
        assert_eq!(to_c(32), 0);
        assert_eq!(to_c(212), 900);
    }

    #[test]
    fn test_scale_and_shift_generic() {
        let id = scale_and_shift(1, 0);
        assert_eq!(id(42), 42);

        let f = scale_and_shift(2, 3);
        assert_eq!(f(5), 13);
        assert_eq!(f(0), 3);
    }
}
