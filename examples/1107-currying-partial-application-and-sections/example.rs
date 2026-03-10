// Currying, Partial Application, and Sections
// OCaml → Rust translation example

// ---------------------------------------------------------------------------
// Partial application via closures
// ---------------------------------------------------------------------------

pub fn partial_add(x: i32) -> impl Fn(i32) -> i32 {
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
    A: Clone + 'static,
    B: 'static,
    C: 'static,
    F: Fn((A, B)) -> C + Clone + 'static,
{
    move |a: A| {
        let f = f.clone();
        Box::new(move |b: B| f((a.clone(), b)))
    }
}

pub fn uncurry<A, B, C, F, G>(f: F) -> impl Fn((A, B)) -> C
where
    F: Fn(A) -> G,
    G: Fn(B) -> C,
{
    move |(a, b)| f(a)(b)
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
// Labeled-argument partial application
// ---------------------------------------------------------------------------

pub fn scale_and_shift(scale: i32, shift: i32) -> impl Fn(i32) -> i32 {
    move |x| x * scale + shift
}

// ---------------------------------------------------------------------------
// Pipeline
// ---------------------------------------------------------------------------

pub fn apply_pipeline(init: i32, pipeline: &[fn(i32) -> i32]) -> i32 {
    pipeline.iter().fold(init, |acc, f| f(acc))
}

fn main() {
    let add5 = partial_add(5);
    println!("add5(10)        = {}", add5(10));
    println!("double(7)       = {}", double(7));
    println!("halve(20)       = {}", halve(20));

    // Function pipeline: 6 →*2→ 12 →+1→ 13 →/2→ 6
    let result = apply_pipeline(6, &[double, increment, halve]);
    println!("6 |>*2|>+1|>/2 = {}", result);

    // Partial application with two captured parameters
    let celsius_of_fahrenheit = scale_and_shift(5, -160);
    println!("212F (×5−160)   = {}", celsius_of_fahrenheit(212));

    // curry round-trip
    let curried_add = curry(|(x, y): (i32, i32)| x + y);
    println!("curried(3)(4)   = {}", curried_add(3)(4));

    // uncurry round-trip
    let tupled_add = uncurry(|x: i32| move |y: i32| x + y);
    println!("tupled((3,4))   = {}", tupled_add((3, 4)));

    // add_tup demo
    println!("add_tup((5,6))  = {}", add_tup((5, 6)));
}

/* Output:
   add5(10)        = 15
   double(7)       = 14
   halve(20)       = 10
   6 |>*2|>+1|>/2 = 6
   212F (×5−160)   = 900
   curried(3)(4)   = 7
   tupled((3,4))   = 7
   add_tup((5,6))  = 11
*/
