// 473. FromStr and parse()
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
struct ColorErr(String);
impl fmt::Display for ColorErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Color {
    type Err = ColorErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Vec<&str> = s.split(',').collect();
        if p.len() != 3 {
            return Err(ColorErr(s.to_string()));
        }
        let u = |x: &str| x.trim().parse::<u8>().map_err(|_| ColorErr(s.to_string()));
        Ok(Color {
            r: u(p[0])?,
            g: u(p[1])?,
            b: u(p[2])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_int() {
        assert_eq!("42".parse::<i32>().unwrap(), 42);
    }
    #[test]
    fn test_err() {
        assert!("abc".parse::<i32>().is_err());
    }
    #[test]
    fn test_color() {
        let c: Color = "10,20,30".parse().unwrap();
        assert_eq!(
            c,
            Color {
                r: 10,
                g: 20,
                b: 30
            }
        );
    }
    #[test]
    fn test_bad() {
        assert!("1,2".parse::<Color>().is_err());
    }
}
