//! 306. ok_or and ok_or_else
//!
//! `ok_or(err)` converts `Option<T>` to `Result<T, E>`, providing an error for `None`.

fn lookup<'a>(map: &'a std::collections::HashMap<&str, &str>, key: &str)
    -> Result<&'a str, String>
{
    map.get(key).copied().ok_or_else(|| format!("key '{}' not found", key))
}

fn main() {
    let some42: Option<i32> = Some(42);
    let none: Option<i32> = None;

    // ok_or: eager error
    println!("Some(42).ok_or('missing') = {:?}", some42.ok_or("missing"));
    println!("None.ok_or('missing')     = {:?}", none.ok_or("missing"));

    // ok_or_else: lazy error (only evaluated if None)
    println!("ok_or_else: {:?}", none.ok_or_else(|| format!("error at {}", 42)));

    // Reverse: Result -> Option via .ok() and .err()
    let ok: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("bad");
    println!("Ok(5).ok()  = {:?}", ok.ok());
    println!("Err.ok()    = {:?}", err.ok());
    println!("Ok(5).err() = {:?}", ok.err());

    // Practical: HashMap lookup with descriptive errors
    let mut config = std::collections::HashMap::new();
    config.insert("host", "localhost");
    config.insert("port", "8080");

    match lookup(&config, "host") {
        Ok(v) => println!("host = {}", v),
        Err(e) => println!("Error: {}", e),
    }
    match lookup(&config, "db_url") {
        Ok(v) => println!("db_url = {}", v),
        Err(e) => println!("Error: {}", e),
    }

    // Chaining ok_or with ?
    fn get_port(config: &std::collections::HashMap<&str, &str>) -> Result<u16, String> {
        let s = config.get("port").copied().ok_or_else(|| "port not set".to_string())?;
        s.parse::<u16>().map_err(|e| format!("invalid port: {}", e))
    }
    println!("port = {:?}", get_port(&config));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ok_or_some() {
        assert_eq!(Some(5i32).ok_or("missing"), Ok(5));
    }

    #[test]
    fn test_ok_or_none() {
        assert_eq!(None::<i32>.ok_or("missing"), Err("missing"));
    }

    #[test]
    fn test_ok_or_else_lazy() {
        let mut called = false;
        let _: Result<i32, &str> = Some(5).ok_or_else(|| { called = true; "err" });
        assert!(!called);
    }

    #[test]
    fn test_result_ok() {
        let r: Result<i32, &str> = Ok(42);
        assert_eq!(r.ok(), Some(42));
    }

    #[test]
    fn test_result_err() {
        let r: Result<i32, &str> = Err("bad");
        assert_eq!(r.err(), Some("bad"));
    }
}
