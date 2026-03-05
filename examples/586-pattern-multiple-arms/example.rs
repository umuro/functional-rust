#[derive(Debug)]
enum Token { Plus,Minus,Star,Slash,Eq,Ne,Lt,Le,Gt,Ge,LParen,RParen,Num(i64),Ident(String) }

fn token_type(t: &Token) -> &'static str {
    match t {
        Token::Plus|Token::Minus|Token::Star|Token::Slash => "arithmetic",
        Token::Eq|Token::Ne|Token::Lt|Token::Le|Token::Gt|Token::Ge => "comparison",
        Token::LParen|Token::RParen => "bracket",
        Token::Num(_)               => "number",
        Token::Ident(_)             => "identifier",
    }
}

fn precedence(t: &Token) -> i32 {
    match t {
        Token::Plus|Token::Minus       => 1,
        Token::Star|Token::Slash       => 2,
        Token::Eq|Token::Ne|
        Token::Lt|Token::Le|
        Token::Gt|Token::Ge            => 0,
        _                              => -1,
    }
}

// HTTP status codes — consolidate semantically
fn status_category(code: u16) -> &'static str {
    match code {
        100..=199 => "informational",
        200..=299 => "success",
        300..=399 => "redirection",
        400..=499 => "client error",
        500..=599 => "server error",
        _         => "unknown",
    }
}

fn main() {
    use Token::*;
    let toks = vec![Plus,Star,Eq,Lt,LParen,Num(42),Ident("x".into())];
    for t in &toks { println!("{:?}: type={} prec={}", t, token_type(t), precedence(t)); }
    for code in [200u16,201,301,404,500,999] {
        println!("{} -> {}", code, status_category(code));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn arith() { assert_eq!(token_type(&Token::Plus), "arithmetic"); }
    #[test] fn cmp()   { assert_eq!(token_type(&Token::Eq),   "comparison"); }
    #[test] fn prec_mul()  { assert_eq!(precedence(&Token::Star), 2); }
    #[test] fn status_ok() { assert_eq!(status_category(200), "success"); }
}
