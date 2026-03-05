//! 315. Exhaustive Result/Option method survey
//!
//! Complete reference of `Result<T,E>` and `Option<T>` methods with examples.

fn main() {
    let ok5: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("bad");
    let some5: Option<i32> = Some(5);
    let none: Option<i32> = None;

    println!("=== RESULT METHODS ===");
    println!("is_ok:        {} {}", ok5.is_ok(), err.is_ok());
    println!("is_err:       {} {}", ok5.is_err(), err.is_err());
    println!("ok():         {:?} {:?}", ok5.ok(), err.ok());
    println!("err():        {:?} {:?}", ok5.err(), err.err());
    println!("map:          {:?}", ok5.map(|x| x * 2));
    println!("map_err:      {:?}", err.map_err(|e| format!("wrapped: {}", e)));
    println!("map_or:       {}", ok5.map_or(0, |x| x + 1));
    println!("map_or_else:  {}", err.map_or_else(|_| 99, |x| x));
    println!("and:          {:?}", ok5.and(Ok::<i32, &str>(10)));
    println!("and (err):    {:?}", err.and(Ok::<i32, &str>(10)));
    println!("or:           {:?}", err.or(Ok(42)));
    println!("or_else:      {:?}", err.or_else(|_| Ok(99)));
    println!("and_then:     {:?}", ok5.and_then(|x| Ok::<i32, &str>(x * 3)));
    println!("unwrap_or:    {}", err.unwrap_or(0));
    println!("unwrap_or_else: {}", err.unwrap_or_else(|_| 42));
    println!("unwrap_or_default: {}", err.unwrap_or_default());

    println!();
    println!("=== OPTION METHODS ===");
    println!("is_some:      {} {}", some5.is_some(), none.is_some());
    println!("is_none:      {} {}", some5.is_none(), none.is_none());
    println!("map:          {:?}", some5.map(|x| x * 2));
    println!("filter:       {:?}", some5.filter(|&x| x > 3));
    println!("filter (fail):{:?}", some5.filter(|&x| x > 10));
    println!("and:          {:?}", some5.and(Some(10)));
    println!("and (none):   {:?}", none.and(Some(10)));
    println!("or:           {:?}", none.or(Some(42)));
    println!("or_else:      {:?}", none.or_else(|| Some(99)));
    println!("and_then:     {:?}", some5.and_then(|x| if x > 3 { Some(x+1) } else { None }));
    println!("unwrap_or:    {}", none.unwrap_or(0));
    println!("unwrap_or_default: {}", none.unwrap_or_default::<i32>());
    println!("map_or:       {}", some5.map_or(0, |x| x + 1));
    println!("ok_or:        {:?}", none.ok_or("missing"));
    println!("flatten:      {:?}", Some(some5).flatten());
    println!("zip:          {:?}", some5.zip(Some("hello")));
    println!("unzip:        {:?}", Some((1i32, "a")).unzip());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_result_map_chain() {
        let r: Result<i32, &str> = Ok(5);
        assert_eq!(r.map(|x| x * 2).map(|x| x + 1), Ok(11));
    }

    #[test]
    fn test_option_and_then_chain() {
        let r = Some(5i32)
            .and_then(|x| if x > 0 { Some(x * 2) } else { None })
            .filter(|&x| x < 20);
        assert_eq!(r, Some(10));
    }

    #[test]
    fn test_result_or_else() {
        let r: Result<i32, &str> = Err("bad");
        assert_eq!(r.or_else(|_| Ok(42)), Ok(42));
    }

    #[test]
    fn test_option_zip() {
        assert_eq!(Some(1i32).zip(Some("a")), Some((1, "a")));
        assert_eq!(Some(1i32).zip(None::<&str>), None);
    }

    #[test]
    fn test_option_flatten() {
        assert_eq!(Some(Some(42i32)).flatten(), Some(42));
        assert_eq!(Some(None::<i32>).flatten(), None);
    }
}
