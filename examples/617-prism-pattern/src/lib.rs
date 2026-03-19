//! # Prism Pattern
//! Focus on one variant of an enum.

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Num(f64),
    Str(String),
    Array(Vec<Json>),
}

pub fn preview_num(j: &Json) -> Option<f64> {
    match j {
        Json::Num(n) => Some(*n),
        _ => None,
    }
}

pub fn review_num(n: f64) -> Json {
    Json::Num(n)
}

pub fn over_num(j: &Json, f: impl Fn(f64) -> f64) -> Json {
    match j {
        Json::Num(n) => Json::Num(f(*n)),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prism() {
        let j = Json::Num(5.0);
        assert_eq!(preview_num(&j), Some(5.0));
        let j2 = over_num(&j, |n| n * 2.0);
        assert_eq!(j2, Json::Num(10.0));
    }
}
