fn grade(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89  => 'B',
        70..=79  => 'C',
        60..=69  => 'D',
        _        => 'F',
    }
}

fn classify_char(c: char) -> &'static str {
    match c {
        'A'..='Z' => "upper",
        'a'..='z' => "lower",
        '0'..='9' => "digit",
        _         => "other",
    }
}

fn tax(income: u32) -> f64 {
    match income {
        0..=10_000          => 0.10,
        10_001..=40_000     => 0.12,
        40_001..=85_000     => 0.22,
        85_001..=163_300    => 0.24,
        _                   => 0.32,
    }
}

fn main() {
    for s in [95u8,82,74,61,45] { print!("{}->{} ", s, grade(s)); } println!();
    for c in ['A','z','5','!']   { print!("{}:{} ", c, classify_char(c)); } println!();
    for i in [5_000u32, 25_000, 60_000, 100_000] {
        println!("income {} -> {:.0}% tax", i, tax(i)*100.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_grade()      { assert_eq!(grade(95),'A'); assert_eq!(grade(55),'F'); }
    #[test] fn test_char_lower() { assert_eq!(classify_char('x'), "lower"); }
    #[test] fn test_tax()        { assert_eq!(tax(5_000), 0.10); }
}
