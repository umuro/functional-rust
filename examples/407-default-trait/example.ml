(* Default trait concept in OCaml *)

(* Zero-value initialization *)
type server_config = {
  host: string;
  port: int;
  max_connections: int;
  debug: bool;
  timeout_seconds: float;
}

let default_config = {
  host = "localhost";
  port = 8080;
  max_connections = 100;
  debug = false;
  timeout_seconds = 30.0;
}

(* Struct update *)
let make_config ?host ?port ?debug () =
  { default_config with
    host = Option.value host ~default:default_config.host;
    port = Option.value port ~default:default_config.port;
    debug = Option.value debug ~default:default_config.debug;
  }

let print_config c =
  Printf.printf "Config { host=%s, port=%d, max=%d, debug=%b, timeout=%.1f }\n"
    c.host c.port c.max_connections c.debug c.timeout_seconds

let () =
  print_config default_config;
  let custom = make_config ~port:9090 ~debug:true () in
  print_config custom
