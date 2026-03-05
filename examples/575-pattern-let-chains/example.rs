// Requires Rust 1.88+ for stable let chains

fn process(s: &str) -> Option<i32> {
    if let Ok(n) = s.parse::<i32>()
        && n > 0
        && n % 2 == 0
    {
        Some(n * 2)
    } else {
        None
    }
}

struct Config { host: Option<String>, port: Option<u16> }

fn make_addr(cfg: &Config) -> Option<String> {
    if let Some(ref host) = cfg.host
        && let Some(port) = cfg.port
        && !host.is_empty()
        && port > 0
    {
        Some(format!("{}:{}", host, port))
    } else {
        None
    }
}

// Nested let chains in loops
fn first_positive_even(data: &[&str]) -> Option<i32> {
    for &s in data {
        if let Ok(n) = s.parse::<i32>()
            && n > 0
            && n % 2 == 0
        {
            return Some(n);
        }
    }
    None
}

fn main() {
    for s in ["4","-2","3","abc","8"] {
        match process(s) {
            Some(v) => println!("{} -> {}", s, v),
            None    => println!("{} -> invalid", s),
        }
    }
    let c1 = Config { host: Some("localhost".into()), port: Some(8080) };
    let c2 = Config { host: Some("".into()),           port: Some(8080) };
    let c3 = Config { host: None,                      port: Some(80)   };
    println!("{:?}", make_addr(&c1));
    println!("{:?}", make_addr(&c2));
    println!("{:?}", make_addr(&c3));
    println!("{:?}", first_positive_even(&["x","3","-4","6","8"]));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_process() {
        assert_eq!(process("4"),   Some(8));
        assert_eq!(process("-2"),  None);
        assert_eq!(process("3"),   None);
        assert_eq!(process("abc"), None);
    }
}
