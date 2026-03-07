/// Idiomatic Rust: Direct boolean logic with clearest precedence
pub fn is_leap_year(year: u32) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}

fn main() {
    println!("2000 is leap: {}", is_leap_year(2000));
    println!("1900 is leap: {}", is_leap_year(1900));
    println!("2004 is leap: {}", is_leap_year(2004));
    println!("2001 is leap: {}", is_leap_year(2001));
}
