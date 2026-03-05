//! 301. Converting Result<Option<T>> into Option<Result<T>>
//!
//! `Result::transpose()` swaps `Result` and `Option` layers.

fn maybe_parse(s: Option<&str>) -> Result<Option<i32>, std::num::ParseIntError> {
    match s {
        None => Ok(None),
        Some(s) => s.parse::<i32>().map(Some),
    }
}

fn main() {
    // Result::transpose()
    let ok_some: Result<Option<i32>, &str> = Ok(Some(42));
    let ok_none: Result<Option<i32>, &str> = Ok(None);
    let err:     Result<Option<i32>, &str> = Err("bad");

    println!("Ok(Some(42)).transpose() = {:?}", ok_some.transpose());
    println!("Ok(None).transpose()     = {:?}", ok_none.transpose());
    println!("Err(...).transpose()     = {:?}", err.transpose());

    // Option::transpose()
    let some_ok: Option<Result<i32, &str>> = Some(Ok(5));
    let some_err: Option<Result<i32, &str>> = Some(Err("fail"));
    let none: Option<Result<i32, &str>> = None;

    println!("Some(Ok(5)).transpose()   = {:?}", some_ok.transpose());
    println!("Some(Err).transpose()     = {:?}", some_err.transpose());
    println!("None.transpose()          = {:?}", none.transpose());

    // Practical: parse optional config value
    let config_val: Option<&str> = Some("42");
    let parsed: Option<Result<i32, _>> = config_val.map(|s| s.parse::<i32>());
    let transposed: Result<Option<i32>, _> = parsed.transpose();
    println!("Config parse transposed: {:?}", transposed);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_result_transpose_ok_some() {
        let r: Result<Option<i32>, &str> = Ok(Some(42));
        assert_eq!(r.transpose(), Some(Ok(42)));
    }

    #[test]
    fn test_result_transpose_ok_none() {
        let r: Result<Option<i32>, &str> = Ok(None);
        assert_eq!(r.transpose(), None);
    }

    #[test]
    fn test_result_transpose_err() {
        let r: Result<Option<i32>, &str> = Err("bad");
        assert_eq!(r.transpose(), Some(Err("bad")));
    }

    #[test]
    fn test_option_transpose() {
        let o: Option<Result<i32, &str>> = Some(Ok(5));
        assert_eq!(o.transpose(), Ok(Some(5)));
    }
}
