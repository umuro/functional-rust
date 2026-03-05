// 499. Escaping and unescaping strings
fn escape_html(s: &str) -> String {
    s.chars().flat_map(|c| match c {
        '<'  => "&lt;".chars().collect::<Vec<_>>(),
        '>'  => "&gt;".chars().collect(),
        '&'  => "&amp;".chars().collect(),
        '"'  => "&quot;".chars().collect(),
        '\'' => "&#39;".chars().collect(),
        c    => vec![c],
    }).collect()
}

fn unescape_html(s: &str) -> String {
    s.replace("&lt;","<").replace("&gt;",">")
     .replace("&amp;","&").replace("&quot;",""").replace("&#39;","'")
}

fn escape_control(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            '\\' => out.push_str("\\\\"),
            '"'  => out.push_str("\\\""),
            c    => out.push(c),
        }
    }
    out
}

fn unescape_control(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        if c == '\\' {
            match iter.next() {
                Some('n')  => out.push('\n'),
                Some('t')  => out.push('\t'),
                Some('r')  => out.push('\r'),
                Some('\\') => out.push('\\'),
                Some('"')  => out.push('"'),
                Some(c)    => { out.push('\\'); out.push(c); }
                None       => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_html_escape()   { assert_eq!(escape_html("<b>hi</b>"),"&lt;b&gt;hi&lt;/b&gt;"); }
    #[test] fn test_html_unescape() { assert_eq!(unescape_html("&lt;b&gt;"),"<b>"); }
    #[test] fn test_roundtrip_html(){ let s="<div>&amp;</div>"; assert_eq!(unescape_html(&escape_html(s)),s); }
    #[test] fn test_control_esc()   { assert_eq!(escape_control("a\nb"),"a\\nb"); }
    #[test] fn test_control_unesc() { assert_eq!(unescape_control("a\\nb"),"a\nb"); }
}
