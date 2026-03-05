use std::any::Any;

#[derive(Debug,Clone)]
enum Value { Int(i64), Float(f64), Str(String), Bool(bool) }

impl Value {
    fn type_name(&self) -> &'static str {
        match self { Value::Int(_)=>"int", Value::Float(_)=>"float",
                     Value::Str(_)=>"str", Value::Bool(_)=>"bool" }
    }
    fn to_f64(&self) -> Option<f64> {
        match self {
            Value::Int(n)   => Some(*n as f64),   // 'as' numeric cast
            Value::Float(f) => Some(*f),
            Value::Str(s)   => s.parse().ok(),
            _               => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Int(n)  => write!(f, "{}", n),
            Value::Float(v)=> write!(f, "{}", v),
            Value::Str(s)  => write!(f, "{:?}", s),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

fn describe_any(v: &dyn Any) -> &'static str {
    if      v.downcast_ref::<i32>()    .is_some() { "i32" }
    else if v.downcast_ref::<f64>()    .is_some() { "f64" }
    else if v.downcast_ref::<String>() .is_some() { "String" }
    else if v.downcast_ref::<bool>()   .is_some() { "bool" }
    else { "unknown" }
}

fn main() {
    let vals = vec![Value::Int(42), Value::Float(3.14), Value::Str("hi".into()), Value::Bool(true)];
    for v in &vals { println!("{} : {}", v, v.type_name()); }
    for v in &vals { if let Some(f)=v.to_f64() { println!("f64: {:.2}", f); } }

    // Numeric 'as' casts
    let x: i32 = 300;
    println!("300i32 as u8={} as f64={}", x as u8, x as f64);

    // Any downcast
    let things: Vec<Box<dyn Any>> = vec![Box::new(42i32), Box::new(3.14f64), Box::new(String::from("hi")), Box::new(true)];
    for t in &things { println!("Any: {}", describe_any(t.as_ref())); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn type_names() { assert_eq!(Value::Int(1).type_name(), "int"); }
    #[test] fn to_f64()     { assert_eq!(Value::Int(5).to_f64(), Some(5.0)); }
}
