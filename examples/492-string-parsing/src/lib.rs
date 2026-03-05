//! # String Parsing — FromStr and Parse

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Point { pub x: i32, pub y: i32 }

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Expected x,y".into());
        }
        Ok(Point {
            x: parts[0].trim().parse().map_err(|e| format!("{}", e))?,
            y: parts[1].trim().parse().map_err(|e| format!("{}", e))?,
        })
    }
}

pub fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

pub fn parse_list(s: &str) -> Vec<i32> {
    s.split(',').filter_map(|p| p.trim().parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_parse() {
        let p: Point = "10, 20".parse().unwrap();
        assert_eq!(p, Point { x: 10, y: 20 });
    }

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int("42"), Some(42));
        assert_eq!(parse_int("abc"), None);
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(parse_list("1, 2, 3"), vec![1, 2, 3]);
    }
}
