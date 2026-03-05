(* Builder pattern with closures in OCaml *)
type server_config = {
  host: string;
  port: int;
  max_connections: int;
  timeout_ms: int;
  on_connect: string -> unit;
}

let default_config = {
  host = "localhost";
  port = 8080;
  max_connections = 100;
  timeout_ms = 5000;
  on_connect = (fun addr -> Printf.printf "Connected: %s\n" addr);
}

let build_server configure =
  configure default_config

let () =
  let cfg = build_server (fun c -> { c with
    port = 9090;
    max_connections = 200;
    on_connect = (fun addr -> Printf.printf "Custom handler: %s\n" addr);
  }) in
  Printf.printf "Server: %s:%d (max=%d, timeout=%dms)\n"
    cfg.host cfg.port cfg.max_connections cfg.timeout_ms;
  cfg.on_connect "192.168.1.1"
