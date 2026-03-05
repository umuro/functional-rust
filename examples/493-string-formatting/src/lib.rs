//! # String Formatting — Display and Format

use std::fmt;

pub struct Money { pub cents: i64 }

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.cents / 100;
        let cents = (self.cents % 100).abs();
        if self.cents < 0 {
            write!(f, "-${}.{:02}", -dollars, cents)
        } else {
            write!(f, "${}.{:02}", dollars, cents)
        }
    }
}

pub fn format_number(n: i64) -> String {
    format!("{:>10}", n)
}

pub fn format_hex(n: u32) -> String {
    format!("{:#x}", n)
}

pub fn format_binary(n: u8) -> String {
    format!("{:08b}", n)
}

pub fn format_float(f: f64, precision: usize) -> String {
    format!("{:.prec$}", f, prec = precision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money() {
        assert_eq!(Money { cents: 1234 }.to_string(), "$12.34");
        assert_eq!(Money { cents: -500 }.to_string(), "-$5.00");
    }

    #[test]
    fn test_formats() {
        assert_eq!(format_hex(255), "0xff");
        assert_eq!(format_binary(5), "00000101");
        assert_eq!(format_float(3.14159, 2), "3.14");
    }
}
