pub fn safe_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 { None } else { Some(x / y) }
}

pub fn safe_head(list: &[i32]) -> Option<i32> {
    list.first().copied()
}

// Idiomatic: Option::and_then (bind) + Option::map (fmap)
pub fn compute_idiomatic(lst: &[i32]) -> Option<i32> {
    safe_head(lst)
        .and_then(|x| safe_div(100, x))
        .map(|r| r * 2)
}

// Explicit bind — shows what and_then desugars to
fn bind<T, U>(opt: Option<T>, f: impl FnOnce(T) -> Option<U>) -> Option<U> {
    match opt {
        None => None,
        Some(x) => f(x),
    }
}

pub fn compute_explicit(lst: &[i32]) -> Option<i32> {
    let divided = bind(safe_head(lst), |x| safe_div(100, x));
    divided.map(|r| r * 2)
}

// Question-mark operator — ergonomic monadic short-circuit
pub fn compute_question_mark(lst: &[i32]) -> Option<i32> {
    let x = safe_head(lst)?;
    let r = safe_div(100, x)?;
    Some(r * 2)
}

fn main() {
    let show = |opt: Option<i32>| match opt {
        None => "None".to_string(),
        Some(x) => x.to_string(),
    };

    println!("--- Idiomatic (and_then + map) ---");
    println!("compute([5,3,1]) = {}", show(compute_idiomatic(&[5, 3, 1])));
    println!("compute([0,1])   = {}", show(compute_idiomatic(&[0, 1])));
    println!("compute([])      = {}", show(compute_idiomatic(&[])));

    println!("\n--- Explicit bind ---");
    println!("compute([5,3,1]) = {}", show(compute_explicit(&[5, 3, 1])));
    println!("compute([0,1])   = {}", show(compute_explicit(&[0, 1])));
    println!("compute([])      = {}", show(compute_explicit(&[])));

    println!("\n--- Question-mark operator ---");
    println!("compute([5,3,1]) = {}", show(compute_question_mark(&[5, 3, 1])));
    println!("compute([0,1])   = {}", show(compute_question_mark(&[0, 1])));
    println!("compute([])      = {}", show(compute_question_mark(&[])));
}

/* Output:
   --- Idiomatic (and_then + map) ---
   compute([5,3,1]) = 40
   compute([0,1])   = None
   compute([])      = None

   --- Explicit bind ---
   compute([5,3,1]) = 40
   compute([0,1])   = None
   compute([])      = None

   --- Question-mark operator ---
   compute([5,3,1]) = 40
   compute([0,1])   = None
   compute([])      = None
*/
