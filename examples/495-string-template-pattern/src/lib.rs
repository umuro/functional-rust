// 495. Template string pattern
use std::collections::HashMap;

fn render(template: &str, vars: &HashMap<&str, &str>) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

fn render_fn<F: Fn(&str) -> Option<String>>(template: &str, lookup: F) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        rest = &rest[start+2..];
        if let Some(end) = rest.find("}}") {
            let key = &rest[..end];
            out.push_str(&lookup(key).unwrap_or_else(|| format!("{{{{{}}}}}", key)));
            rest = &rest[end+2..];
        } else {
            out.push_str("{{"); // unclosed — keep as-is
        }
    }
    out.push_str(rest);
    out
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_render() {
        let mut v=HashMap::new(); v.insert("x","10"); v.insert("y","20");
        assert_eq!(render("{{x}}+{{y}}",&v),"10+20");
    }
    #[test] fn test_missing() {
        let v:HashMap<&str,&str>=HashMap::new();
        assert_eq!(render("{{x}}",&v),"{{x}}"); // placeholder kept
    }
}
