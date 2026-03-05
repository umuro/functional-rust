const MIN_AGE: u32 = 18;
const MAX_AGE: u32 = 65;
const HTTP:    u16 = 80;
const HTTPS:   u16 = 443;
const ADMIN:   u16 = 8080;

fn classify_age(age: u32) -> &'static str {
    match age {
        0             => "newborn",
        1..=MIN_AGE   => "minor",
        MIN_AGE..=MAX_AGE => "adult",
        _             => "senior",
    }
}

fn describe_port(p: u16) -> &'static str {
    match p {
        HTTP  => "HTTP",
        HTTPS => "HTTPS",
        ADMIN => "Admin",
        1..=1023      => "well-known",
        1024..=49151  => "registered",
        _             => "dynamic",
    }
}

// Associated consts
struct Cfg;
impl Cfg {
    const TIMEOUT: u32 = 30;
}

fn classify_timeout(t: u32) -> &'static str {
    match t {
        0               => "none",
        Cfg::TIMEOUT    => "default",
        1..=10          => "fast",
        _               => "slow",
    }
}

fn main() {
    for a in [0u32,1,18,40,65,80] { println!("age {}: {}", a, classify_age(a)); }
    for p in [80u16,443,8080,3000,50000] { println!("port {}: {}", p, describe_port(p)); }
    for t in [0u32,5,30,60] { println!("timeout {}: {}", t, classify_timeout(t)); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn age18()   { assert_eq!(classify_age(18), "adult"); }
    #[test] fn port80()  { assert_eq!(describe_port(80), "HTTP"); }
    #[test] fn timeout() { assert_eq!(classify_timeout(30), "default"); }
}
