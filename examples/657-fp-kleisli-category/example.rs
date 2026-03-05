// Kleisli Category in Rust

// Kleisli composition (>=>) for Option
fn fish<A, B, C>(
    f: impl FnOnce(A) -> Option<B>,
    g: impl FnOnce(B) -> Option<C>,
) -> impl FnOnce(A) -> Option<C> {
    move |a| f(a).and_then(g)
}

// Validators as Kleisli arrows
fn validate_positive(x: i32) -> Option<i32> {
    if x > 0 { Some(x) } else { None }
}

fn validate_even(x: i32) -> Option<i32> {
    if x % 2 == 0 { Some(x) } else { None }
}

fn double(x: i32) -> Option<i32> {
    Some(x * 2)
}

fn main() {
    // Compose: positive? >=> even? >=> double
    let pipeline = fish(fish(validate_positive, validate_even), double);
    
    println!("4: {:?}", pipeline(4));   // Some(8)
    println!("3: {:?}", fish(fish(validate_positive, validate_even), double)(3));   // None
    println!("-2: {:?}", fish(fish(validate_positive, validate_even), double)(-2)); // None
}
