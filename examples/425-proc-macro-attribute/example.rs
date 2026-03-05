// Attribute macros #[my_attr] — concept and simulation

// ===========================================================
// REAL PROC MACRO (requires separate proc-macro crate):
//
// #[proc_macro_attribute]
// pub fn log_calls(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as ItemFn);
//     let fn_name = &input.sig.ident;
//     let fn_args = &input.sig.inputs;
//     let fn_body = &input.block;
//
//     quote! {
//         fn #fn_name(#fn_args) {
//             println!("[CALL] {}", stringify!(#fn_name));
//             let result = { #fn_body };
//             println!("[RETURN] {}", stringify!(#fn_name));
//             result
//         }
//     }
// }
// ===========================================================

// Simulation: higher-order function wrappers

// Simulate #[log_calls]
fn logged<F: Fn(i32) -> i32>(name: &'static str, f: F) -> impl Fn(i32) -> i32 {
    move |x| {
        println!("[CALL] {}({})", name, x);
        let result = f(x);
        println!("[RETURN] {} -> {}", name, result);
        result
    }
}

// Simulate #[memoize] for single i32 argument
fn memoized<F: Fn(u64) -> u64>(f: F) -> impl FnMut(u64) -> u64 {
    use std::collections::HashMap;
    let mut cache: HashMap<u64, u64> = HashMap::new();
    move |x| {
        if let Some(&cached) = cache.get(&x) {
            return cached;
        }
        let result = f(x);
        cache.insert(x, result);
        result
    }
}

// Simulate #[retry(times=3)]
fn with_retry<T, E: std::fmt::Debug, F: Fn() -> Result<T, E>>(
    times: u32,
    f: F,
) -> Result<T, E> {
    let mut last_err = None;
    for attempt in 1..=times {
        match f() {
            Ok(val) => return Ok(val),
            Err(e) => {
                println!("Attempt {} failed: {:?}", attempt, e);
                last_err = Some(e);
            }
        }
    }
    Err(last_err.unwrap())
}

// Simulate #[route(GET, "/users")]
struct Route {
    method: &'static str,
    path: &'static str,
    handler: fn() -> String,
}

macro_rules! route {
    ($method:ident, $path:literal, $handler:expr) => {
        Route {
            method: stringify!($method),
            path: $path,
            handler: $handler,
        }
    };
}

fn get_users() -> String { "[{"id":1,"name":"Alice"}]".to_string() }
fn get_health() -> String { "{"status":"ok"}".to_string() }

fn main() {
    // Logged wrapper
    let double = logged("double", |x| x * 2);
    println!("Result: {}", double(21));

    // Retry wrapper
    let mut attempts = 0;
    let result = with_retry(3, || {
        attempts += 1;
        if attempts < 3 { Err("not ready") } else { Ok(42) }
    });
    println!("After retry: {:?}", result);

    // Routes
    let routes = vec![
        route!(GET, "/users", get_users),
        route!(GET, "/health", get_health),
    ];

    println!("
Routes:");
    for r in &routes {
        println!("  {} {} -> {}", r.method, r.path, (r.handler)());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_success() {
        let mut n = 0;
        let r = with_retry(3, || {
            n += 1;
            if n < 2 { Err("fail") } else { Ok(n) }
        });
        assert!(r.is_ok());
    }

    #[test]
    fn test_retry_exhausted() {
        let r = with_retry(3, || Err::<i32, &str>("always fail"));
        assert!(r.is_err());
    }
}
