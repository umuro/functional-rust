#[derive(Debug,Clone)]
enum Dir { N, S, E, W }

fn describe(d: &Dir) -> &'static str {
    match d {
        Dir::N => "north",
        Dir::S => "south",
        Dir::E => "east",
        Dir::W => "west",
        // No _ needed: all covered → compile-time guarantee!
    }
}

fn horizontal(d: &Dir) -> bool {
    match d { Dir::E | Dir::W => true, _ => false }
}

// Library enum: adding variants is safe with #[non_exhaustive]
#[non_exhaustive]
#[derive(Debug)]
enum StatusCode { Ok, NotFound, Unauthorized, ServerError }

fn status_text(c: &StatusCode) -> &'static str {
    match c {
        StatusCode::Ok           => "OK",
        StatusCode::NotFound     => "Not Found",
        StatusCode::Unauthorized => "Unauthorized",
        StatusCode::ServerError  => "Internal Server Error",
        _                        => "Unknown", // required by #[non_exhaustive]
    }
}

fn classify(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0             => "zero",
        1..=i32::MAX  => "positive",
    }
}

fn main() {
    for d in [Dir::N,Dir::S,Dir::E,Dir::W] { println!("{:?}={}", d, describe(&d)); }
    for c in [StatusCode::Ok, StatusCode::NotFound] { println!("{:?}={}", c, status_text(&c)); }
    for n in [-5,0,7] { println!("{}={}", n, classify(n)); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_desc()     { assert_eq!(describe(&Dir::N), "north"); }
    #[test] fn test_classify() { assert_eq!(classify(0), "zero"); assert_eq!(classify(-1), "negative"); }
}
