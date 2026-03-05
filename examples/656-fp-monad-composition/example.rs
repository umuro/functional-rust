// Monad Composition in Rust

// OptionT over Result
type OptionResult<A, E> = Result<Option<A>, E>;

fn bind_option_result<A, B, E>(
    ma: OptionResult<A, E>,
    f: impl FnOnce(A) -> OptionResult<B, E>,
) -> OptionResult<B, E> {
    match ma {
        Err(e) => Err(e),
        Ok(None) => Ok(None),
        Ok(Some(a)) => f(a),
    }
}

// Practical: validate and transform
fn parse_positive(s: &str) -> OptionResult<u32, String> {
    match s.parse::<i32>() {
        Err(e) => Err(e.to_string()),
        Ok(n) if n > 0 => Ok(Some(n as u32)),
        Ok(_) => Ok(None), // Not positive
    }
}

fn main() {
    let inputs = vec!["42", "-5", "abc"];
    
    for input in inputs {
        let result = bind_option_result(
            parse_positive(input),
            |n| Ok(Some(n * 2))
        );
        println!("{}: {:?}", input, result);
    }
}
