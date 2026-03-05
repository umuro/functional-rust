//! 302. Option/Result transpose patterns
//!
//! `Option::transpose()` converts `Option<Result<T,E>>` into `Result<Option<T>,E>`.

fn lookup_and_parse(
    map: &std::collections::HashMap<&str, &str>,
    key: &str,
) -> Result<Option<i32>, std::num::ParseIntError> {
    // map.get returns Option<&&str>
    // .map(|s| s.parse()) returns Option<Result<i32, ParseIntError>>
    // .transpose() converts to Result<Option<i32>, ParseIntError>
    map.get(key).map(|s| s.parse::<i32>()).transpose()
}

fn main() {
    // Option::transpose() conversions
    let some_ok: Option<Result<i32, &str>> = Some(Ok(42));
    let some_err: Option<Result<i32, &str>> = Some(Err("bad"));
    let none: Option<Result<i32, &str>> = None;

    println!("Some(Ok(42)).transpose()  = {:?}", some_ok.transpose());   // Ok(Some(42))
    println!("Some(Err).transpose()     = {:?}", some_err.transpose());  // Err("bad")
    println!("None.transpose()          = {:?}", none.transpose());      // Ok(None)

    // Practical: optional config key parsing
    let mut config = std::collections::HashMap::new();
    config.insert("port", "8080");
    config.insert("timeout", "xyz");

    let port = lookup_and_parse(&config, "port");
    println!("port: {:?}", port); // Ok(Some(8080))

    let missing = lookup_and_parse(&config, "missing_key");
    println!("missing: {:?}", missing); // Ok(None)

    let bad = lookup_and_parse(&config, "timeout");
    println!("timeout (bad): {:?}", bad); // Err(ParseIntError)

    // Collecting: filter Nones, propagate errors
    let inputs: Vec<Option<&str>> = vec![Some("1"), None, Some("2"), None, Some("bad")];
    let results: Result<Vec<_>, _> = inputs.into_iter()
        .filter_map(|opt| opt.map(|s| s.parse::<i32>()).transpose())
        .collect();
    println!("Collect with transpose: {:?}", results);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_some_ok_transpose() {
        let v: Option<Result<i32, &str>> = Some(Ok(5));
        assert_eq!(v.transpose(), Ok(Some(5)));
    }

    #[test]
    fn test_some_err_transpose() {
        let v: Option<Result<i32, &str>> = Some(Err("fail"));
        assert_eq!(v.transpose(), Err("fail"));
    }

    #[test]
    fn test_none_transpose() {
        let v: Option<Result<i32, &str>> = None;
        assert_eq!(v.transpose(), Ok(None));
    }

    #[test]
    fn test_collect_with_transpose() {
        let inputs: Vec<Option<&str>> = vec![Some("1"), None, Some("2")];
        let nums: Result<Vec<i32>, _> = inputs.into_iter()
            .filter_map(|opt| opt.map(|s| s.parse::<i32>()).transpose())
            .collect();
        assert_eq!(nums.unwrap(), vec![1, 2]);
    }
}
