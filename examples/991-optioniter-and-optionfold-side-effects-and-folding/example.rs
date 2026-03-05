/// Option.iter equivalent: run a side effect only if Some.
pub fn greet_if_present(name: Option<&str>) -> Vec<String> {
    let mut log = Vec::new();
    name.iter()
        .for_each(|n| log.push(format!("Hello, {}!", n)));
    log
}

/// Option.iter with `if let` — the most natural Rust idiom.
pub fn greet_if_present_iflet(name: Option<&str>) -> Vec<String> {
    let mut log = Vec::new();
    if let Some(n) = name {
        log.push(format!("Hello, {}!", n));
    }
    log
}

/// Option.fold equivalent via map_or_else.
pub fn greeting(name: Option<&str>) -> String {
    name.map_or_else(|| "Hello, stranger!".to_owned(), |n| format!("Hello, {}!", n))
}

/// Explicit match — mirrors OCaml's pattern match.
pub fn greeting_match(name: Option<&str>) -> String {
    match name {
        None => "Hello, stranger!".to_owned(),
        Some(n) => format!("Hello, {}!", n),
    }
}

/// Generic option_fold: generalised Option.fold.
pub fn option_fold<T, U>(opt: Option<T>, none: U, some: impl FnOnce(T) -> U) -> U {
    opt.map_or_else(|| none, some)
}

fn main() {
    let maybe_name: Option<&str> = Some("Alice");
    let no_name: Option<&str> = None;

    // Option::iter — zero or one iterations
    for msg in greet_if_present(maybe_name) {
        println!("{msg}");
    }
    let silent = greet_if_present(no_name);
    println!("(None produced {} messages)", silent.len());

    // Option::map_or_else — the Rust fold
    println!("{}", greeting(maybe_name));
    println!("{}", greeting(no_name));

    // Generic fold
    let doubled = option_fold(Some(21), 0, |x| x * 2);
    println!("option_fold(Some(21), 0, *2) = {doubled}");
    let default = option_fold(None::<i32>, 0, |x| x * 2);
    println!("option_fold(None, 0, *2)     = {default}");
}

/* Output:
   Hello, Alice!
   (None produced 0 messages)
   Hello, Alice!
   Hello, stranger!
   option_fold(Some(21), 0, *2) = 42
   option_fold(None, 0, *2)     = 0
*/
