//! let-else Pattern
//!
//! Early return when pattern doesn't match.

/// Basic let-else.
pub fn get_first(v: &[i32]) -> i32 {
    let [first, ..] = v else {
        return -1;
    };
    *first
}

/// let-else with Option.
pub fn process_option(opt: Option<i32>) -> i32 {
    let Some(value) = opt else {
        return 0;
    };
    value * 2
}

/// let-else with Result.
pub fn process_result(res: Result<i32, &str>) -> i32 {
    let Ok(value) = res else {
        return -1;
    };
    value + 10
}

/// let-else with struct destructure.
pub struct Config {
    pub value: Option<i32>,
}

pub fn get_config_value(c: &Config) -> i32 {
    let Some(v) = c.value else {
        return 0;
    };
    v
}

/// Multiple let-else in sequence.
pub fn parse_pair(s: &str) -> Option<(i32, i32)> {
    let Some((a, b)) = s.split_once(',') else {
        return None;
    };
    let Ok(x) = a.trim().parse() else {
        return None;
    };
    let Ok(y) = b.trim().parse() else {
        return None;
    };
    Some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first() {
        assert_eq!(get_first(&[1, 2, 3]), 1);
        assert_eq!(get_first(&[]), -1);
    }

    #[test]
    fn test_process_option() {
        assert_eq!(process_option(Some(5)), 10);
        assert_eq!(process_option(None), 0);
    }

    #[test]
    fn test_process_result() {
        assert_eq!(process_result(Ok(5)), 15);
        assert_eq!(process_result(Err("error")), -1);
    }

    #[test]
    fn test_get_config() {
        let c = Config { value: Some(42) };
        assert_eq!(get_config_value(&c), 42);
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair("1, 2"), Some((1, 2)));
        assert_eq!(parse_pair("invalid"), None);
    }
}
