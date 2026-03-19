//! # Free Monad
//!
//! Build monadic DSLs where the structure is data, interpretation is separate.

/// Free monad over a functor F.
pub enum Free<F, A> {
    Pure(A),
    Suspend(F),
}

/// Simple DSL for key-value operations.

pub enum KvOp<Next> {
    Get(String, Box<dyn Fn(Option<String>) -> Next>),
    Put(String, String, Box<dyn Fn(()) -> Next>),
}

/// A simpler, concrete approach for demonstration.

pub enum KvDsl {
    Get(String),
    Put(String, String),
    Pure(String),
    Bind(Box<KvDsl>, String), // simplified
}

/// Interpret DSL with a HashMap.
pub fn interpret(dsl: &KvDsl, store: &mut std::collections::HashMap<String, String>) -> String {
    match dsl {
        KvDsl::Get(k) => store.get(k).cloned().unwrap_or_default(),
        KvDsl::Put(k, v) => {
            store.insert(k.clone(), v.clone());
            String::new()
        }
        KvDsl::Pure(v) => v.clone(),
        KvDsl::Bind(inner, _) => interpret(inner, store),
    }
}

/// Build a get operation.
pub fn get(key: &str) -> KvDsl {
    KvDsl::Get(key.to_string())
}

/// Build a put operation.
pub fn put(key: &str, value: &str) -> KvDsl {
    KvDsl::Put(key.to_string(), value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_put_get() {
        let mut store = HashMap::new();
        interpret(&put("x", "42"), &mut store);
        let result = interpret(&get("x"), &mut store);
        assert_eq!(result, "42");
    }

    #[test]
    fn test_get_missing() {
        let mut store = HashMap::new();
        let result = interpret(&get("missing"), &mut store);
        assert_eq!(result, "");
    }
}
