(* Records — Functional Update *)
(* Non-destructive record updates with 'with' *)

type config = {
  host : string;
  port : int;
  debug : bool;
  max_connections : int;
  timeout_ms : int;
}

let default_config = {
  host = "localhost"; port = 8080;
  debug = false; max_connections = 100; timeout_ms = 5000
}

let dev_config = { default_config with debug = true; port = 3000 }
let prod_config = { default_config with
  host = "0.0.0.0"; max_connections = 10000; timeout_ms = 30000
}

let () = Printf.printf "Dev: %s:%d (debug=%b)\n"
  dev_config.host dev_config.port dev_config.debug
let () = Printf.printf "Prod: %s:%d (max=%d)\n"
  prod_config.host prod_config.port prod_config.max_connections
