// 497. Case conversion patterns
fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn to_camel_case(s: &str) -> String {
    s.split('_')
        .enumerate()
        .flat_map(|(i, word)| {
            let mut chars = word.chars();
            if i == 0 {
                chars.map(|c| c).collect::<String>()
            } else {
                let first = chars
                    .next()
                    .map(|c| c.to_uppercase().to_string())
                    .unwrap_or_default();
                let rest: String = chars.collect();
                first + &rest
            }
            .chars()
            .collect::<Vec<_>>()
        })
        .collect()
}

fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut cs = word.chars();
            cs.next()
                .map(|c| c.to_uppercase().collect::<String>() + cs.as_str())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_upper() {
        assert_eq!("hello".to_uppercase(), "HELLO");
    }
    #[test]
    fn test_lower() {
        assert_eq!("HELLO".to_lowercase(), "hello");
    }
    #[test]
    fn test_snake() {
        assert_eq!(to_snake_case("MyFunc"), "my_func");
    }
    #[test]
    fn test_camel() {
        assert_eq!(to_camel_case("my_func_name"), "myFuncName");
    }
    #[test]
    fn test_title() {
        assert_eq!(to_title_case("hello world"), "Hello World");
    }
}
