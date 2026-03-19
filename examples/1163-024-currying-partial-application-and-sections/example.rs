// Currying, Partial Application, and Operator Sections
// OCaml → Rust translation example

// ---------------------------------------------------------------------------
// Partial application via closures
// ---------------------------------------------------------------------------

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

pub fn add_tup((x, y): (i32, i32)) -> i32 {
    x + y
}

// ---------------------------------------------------------------------------
// curry / uncurry converters
// ---------------------------------------------------------------------------

pub fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    F: Fn((A, B)) -> C + Clone + 'static,
    A: Clone + 'static,
    B: 'static,
    C: 'static,
{
    move |x: A| {
        let f = f.clone();
        let x = x.clone();
        Box::new(move |y: B| f((x.clone(), y)))
    }
}

pub fn uncurry<A, B, C, G, F>(f: F) -> impl Fn((A, B)) -> C
where
    F: Fn(A) -> G,
    G: Fn(B) -> C,
{
    move |(x, y)| f(x)(y)
}

// ---------------------------------------------------------------------------
// flip — swap argument order
// ---------------------------------------------------------------------------

pub fn flip<A, B, C, F>(f: F) -> impl Fn(B) -> Box<dyn Fn(A) -> C>
where
    F: Fn(A, B) -> C + Clone + 'static,
    A: 'static,
    B: Clone + 'static,
    C: 'static,
{
    move |b: B| {
        let f = f.clone();
        let b = b.clone();
        Box::new(move |a: A| f(a, b.clone()))
    }
}

// ---------------------------------------------------------------------------
// Operator sections
// ---------------------------------------------------------------------------

pub fn double(x: i32) -> i32 {
    x * 2
}

pub fn increment(x: i32) -> i32 {
    x + 1
}

pub fn halve(x: i32) -> i32 {
    x / 2
}

// ---------------------------------------------------------------------------
// scale_and_shift / celsius_of_fahrenheit
// ---------------------------------------------------------------------------

pub fn scale_and_shift(scale: i32, shift: i32, x: i32) -> i32 {
    x * scale + shift
}

pub fn celsius_of_fahrenheit(fahrenheit: i32) -> i32 {
    scale_and_shift(5, -160, fahrenheit)
}

// ---------------------------------------------------------------------------
// Function pipeline
// ---------------------------------------------------------------------------

pub fn apply_pipeline(fns: &[fn(i32) -> i32], start: i32) -> i32 {
    fns.iter().fold(start, |acc, f| f(acc))
}

// ---------------------------------------------------------------------------

fn main() {
    // Partial application: make_adder(5) returns a closure
    let add5 = make_adder(5);
    println!("add5(10)  = {}", add5(10)); // 15

    // curry: tupled add_tup → sequential
    let curried = curry(add_tup);
    println!("curry(add_tup)(3)(4) = {}", curried(3)(4)); // 7

    // Operator sections
    println!("double(7)    = {}", double(7)); // 14
    println!("increment(41) = {}", increment(41)); // 42
    println!("halve(20)    = {}", halve(20)); // 10

    // flip: Fun.flip ( / ) 2  →  |x| x / 2
    let halve_fn = flip(|a: i32, b: i32| a / b)(2);
    println!("halve_fn(20) = {}", halve_fn(20)); // 10

    // Function pipeline via fold: 6 → *2 → +1 → /2
    let pipeline: &[fn(i32) -> i32] = &[double, increment, halve];
    let result = apply_pipeline(pipeline, 6);
    println!("6 |> *2 |> +1 |> /2 = {}", result); // 6

    // Celsius: partial application of scale_and_shift
    println!(
        "212F in Celsius ≈ {} (divide by 9 for actual °C)",
        celsius_of_fahrenheit(212) // 900; 900/9 = 100
    );
}

/* Output:
   add5(10)  = 15
   curry(add_tup)(3)(4) = 7
   double(7)    = 14
   increment(41) = 42
   halve(20)    = 10
   halve_fn(20) = 10
   6 |> *2 |> +1 |> /2 = 6
   212F in Celsius ≈ 900 (divide by 9 for actual °C)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_application_via_closure() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(-3), 2);
        assert_eq!(make_adder(0)(42), 42);
    }

    #[test]
    fn test_add_and_add_tup_agree() {
        assert_eq!(add(3, 4), add_tup((3, 4)));
        assert_eq!(add(0, 7), add_tup((0, 7)));
        assert_eq!(add(-5, 5), add_tup((-5, 5)));
    }

    #[test]
    fn test_curry_converts_tupled_to_sequential() {
        let curried = curry(add_tup);
        assert_eq!(curried(3)(4), 7);
        assert_eq!(curried(5)(0), 5);
        let add10 = curried(10);
        assert_eq!(add10(1), 11);
        assert_eq!(add10(90), 100);
    }

    #[test]
    fn test_uncurry_converts_sequential_to_tupled() {
        let tupled = uncurry(|x: i32| move |y: i32| x + y);
        assert_eq!(tupled((3, 4)), 7);
        assert_eq!(tupled((10, 0)), 10);
        assert_eq!(tupled((-1, 1)), 0);
    }

    #[test]
    fn test_flip_swaps_argument_order() {
        let flipped_sub = flip(|a: i32, b: i32| a - b);
        assert_eq!(flipped_sub(3)(10), 7);
        let halve_fn = flip(|a: i32, b: i32| a / b)(2);
        assert_eq!(halve_fn(20), 10);
        assert_eq!(halve_fn(7), 3);
    }

    #[test]
    fn test_operator_sections() {
        assert_eq!(double(7), 14);
        assert_eq!(double(0), 0);
        assert_eq!(increment(41), 42);
        assert_eq!(increment(-1), 0);
        assert_eq!(halve(20), 10);
        assert_eq!(halve(7), 3);
    }

    #[test]
    fn test_pipeline_fold() {
        let pipeline: &[fn(i32) -> i32] = &[double, increment, halve];
        assert_eq!(apply_pipeline(pipeline, 6), 6);
        assert_eq!(apply_pipeline(pipeline, 10), 10);
        assert_eq!(apply_pipeline(&[], 42), 42);
    }

    #[test]
    fn test_celsius_formula_boundary_values() {
        assert_eq!(celsius_of_fahrenheit(32), 0);
        assert_eq!(celsius_of_fahrenheit(212), 900);
        assert_eq!(scale_and_shift(2, 3, 5), 13);
    }
}
