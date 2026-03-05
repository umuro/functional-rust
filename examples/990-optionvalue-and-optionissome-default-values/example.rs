/// Return the value inside an Option, or a default if None.
/// Mirrors OCaml's `Option.value ~default:x opt`.
pub fn option_value<T>(opt: Option<T>, default: T) -> T {
    opt.unwrap_or(default)
}

/// Return true if the Option holds a value.
/// Mirrors OCaml's `Option.is_some opt`.
pub fn option_is_some<T>(opt: &Option<T>) -> bool {
    opt.is_some()
}

/// Functional-style: extract with a lazy default (closure).
pub fn option_value_lazy<T, F: FnOnce() -> T>(opt: Option<T>, default_fn: F) -> T {
    opt.unwrap_or_else(default_fn)
}

/// Map an Option to a concrete value, falling back to a default.
pub fn describe_port(port: Option<u16>) -> String {
    port.map(|p| format!("port {p}"))
        .unwrap_or_else(|| "default port".to_string())
}

fn main() {
    let config_port: Option<u16> = None;
    let config_host: Option<&str> = Some("localhost");

    let port = option_value(config_port, 8080);
    let host = option_value(config_host, "0.0.0.0");

    println!("Server: {host}:{port}");
    println!(
        "Port set: {}, Host set: {}",
        option_is_some(&config_port),
        option_is_some(&config_host)
    );

    // Lazy default — closure only called when None
    let lazy_port = option_value_lazy(config_port, || 9090_u16);
    println!("Lazy port fallback: {lazy_port}");

    println!("{}", describe_port(None));
    println!("{}", describe_port(Some(3000)));
}

/* Output:
   Server: localhost:8080
   Port set: false, Host set: true
   Lazy port fallback: 9090
   default port
   port 3000
*/
