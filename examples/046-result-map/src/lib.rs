#![allow(clippy::all)]
// Result::map / map_err: transform the success or error channel independently.
pub fn transform(r: Result<i32, String>) -> Result<String, String> {
    r.map(|x| (x * 2).to_string()).map_err(|e| format!("Error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_ok() {
        assert_eq!(transform(Ok(21)), Ok("42".to_string()));
    }

    #[test]
    fn test_transform_err() {
        assert_eq!(transform(Err("bad input".to_string())), Err("Error: bad input".to_string()));
    }

    #[test]
    fn test_map_leaves_err_untouched_by_map() {
        let r: Result<i32, String> = Err("x".to_string());
        assert_eq!(r.map(|x| x * 2), Err("x".to_string()));
    }

    #[test]
    fn test_map_err_leaves_ok_untouched() {
        let r: Result<i32, String> = Ok(5);
        assert_eq!(r.map_err(|e: String| format!("Error: {}", e)), Ok(5));
    }
}
