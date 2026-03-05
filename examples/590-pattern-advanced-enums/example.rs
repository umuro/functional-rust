#[derive(Debug,Clone)]
enum Json {
    Null,                                   // unit variant
    Bool(bool),                             // tuple variant
    Num(f64),                               // tuple variant
    Str(String),                            // tuple variant
    Array(Vec<Json>),                       // tuple variant
    Object(Vec<(String, Json)>),            // tuple variant
}

impl Json {
    fn is_null(&self) -> bool { matches!(self, Json::Null) }

    fn depth(&self) -> usize {
        match self {
            Json::Array(xs)  => 1 + xs.iter().map(|x|x.depth()).max().unwrap_or(0),
            Json::Object(kv) => 1 + kv.iter().map(|(_,v)|v.depth()).max().unwrap_or(0),
            _                => 0,
        }
    }

    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(kv) => kv.iter().find(|(k,_)| k==key).map(|(_,v)| v),
            _                => None,
        }
    }
}

impl std::fmt::Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Json::Null        => write!(f, "null"),
            Json::Bool(b)     => write!(f, "{}", b),
            Json::Num(n)      => write!(f, "{}", n),
            Json::Str(s)      => write!(f, "{:?}", s),
            Json::Array(xs)   => {
                write!(f, "[")?;
                for (i,x) in xs.iter().enumerate() { if i>0 {write!(f,",")?;} write!(f,"{}",x)?; }
                write!(f, "]")
            }
            Json::Object(kv)  => {
                write!(f, "{{")?;
                for (i,(k,v)) in kv.iter().enumerate() { if i>0{write!(f,",")?;} write!(f,"{:?}:{}",k,v)?; }
                write!(f, "}}")
            }
        }
    }
}

fn main() {
    let j = Json::Object(vec![
        ("name".into(),   Json::Str("Alice".into())),
        ("age".into(),    Json::Num(30.0)),
        ("scores".into(), Json::Array(vec![Json::Num(95.0), Json::Num(87.0)])),
        ("active".into(), Json::Bool(true)),
        ("notes".into(),  Json::Null),
    ]);
    println!("{}", j);
    println!("depth={}", j.depth());
    println!("name={:?}", j.get("name").map(|v|v.to_string()));
    println!("is_null={}", Json::Null.is_null());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn depth_flat() { assert_eq!(Json::Num(1.0).depth(), 0); }
    #[test] fn depth_nested() {
        let j = Json::Array(vec![Json::Array(vec![Json::Null])]);
        assert_eq!(j.depth(), 2);
    }
}
