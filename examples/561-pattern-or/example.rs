#[derive(Debug)]
enum Color { Red, Green, Blue, Yellow, Purple }

fn classify(c: &Color) -> &'static str {
    match c {
        Color::Red | Color::Yellow  => "warm",
        Color::Green | Color::Blue  => "cool",
        Color::Purple               => "mixed",
    }
}

fn is_primary(c: &Color) -> bool {
    matches!(c, Color::Red | Color::Green | Color::Blue)
}

fn describe_number(n: i32) -> &'static str {
    match n {
        0 | 1         => "tiny",
        2 | 3 | 4     => "small",
        5..=9         => "medium",
        _             => "large",
    }
}

fn main() {
    use Color::*;
    for c in &[Red, Green, Blue, Yellow, Purple] {
        println!("{:?} -> {}", c, classify(c));
    }
    println!("Red primary? {}", is_primary(&Color::Red));
    println!("Yellow primary? {}", is_primary(&Color::Yellow));
    for n in [0, 3, 7, 42] {
        println!("{} is {}", n, describe_number(n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn warm() { assert_eq!(classify(&Color::Red), "warm"); }
    #[test] fn cool() { assert_eq!(classify(&Color::Blue), "cool"); }
    #[test] fn primary() { assert!(is_primary(&Color::Red)); assert!(!is_primary(&Color::Yellow)); }
}
