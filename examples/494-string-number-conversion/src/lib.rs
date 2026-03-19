// 494. Number <-> String conversion

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_string() {
        assert_eq!(42i32.to_string(), "42");
        assert_eq!((-7i32).to_string(), "-7");
    }
    #[test]
    fn test_parse_int() {
        assert_eq!("42".parse::<i32>().unwrap(), 42);
        assert!("abc".parse::<i32>().is_err());
    }
    #[test]
    fn test_hex() {
        assert_eq!(format!("{:x}", 255u32), "ff");
        assert_eq!(i64::from_str_radix("ff", 16).unwrap(), 255);
    }
    #[test]
    fn test_float() {
        assert_eq!(format!("{:.2}", 3.14159f64), "3.14");
        assert!("3.14".parse::<f64>().is_ok());
    }
}
