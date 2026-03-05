use std::fmt;

#[derive(Debug)]
enum DbError { ConnectionFailed(String), QueryTimeout(f64), NotFound(String) }

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed(h) => write!(f, "Cannot connect to {h}"),
            Self::QueryTimeout(s) => write!(f, "Query timed out after {s:.1}s"),
            Self::NotFound(k) => write!(f, "Record not found: {k}"),
        }
    }
}
impl std::error::Error for DbError {}

fn main() {
    let errors: Vec<DbError> = vec![
        DbError::ConnectionFailed("db.prod".into()),
        DbError::QueryTimeout(30.5),
        DbError::NotFound("user:42".into()),
    ];
    for e in &errors {
        println!("Display: {e}");
        println!("Debug:   {e:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn display_human_readable() {
        let e = DbError::ConnectionFailed("localhost".into());
        assert_eq!(e.to_string(), "Cannot connect to localhost");
    }
    #[test] fn debug_has_variant() {
        assert!(format!("{:?}", DbError::NotFound("x".into())).contains("NotFound"));
    }
    #[test] fn implements_error() {
        let e: Box<dyn std::error::Error> = Box::new(DbError::QueryTimeout(5.0));
        assert!(e.to_string().contains("5.0"));
    }
}
