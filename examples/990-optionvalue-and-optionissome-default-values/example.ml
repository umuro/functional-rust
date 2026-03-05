(* Option.value and Option.is_some — Default Values *)
(* Extract values from options with defaults *)

let config_port = None
let config_host = Some "localhost"

let port = Option.value ~default:8080 config_port
let host = Option.value ~default:"0.0.0.0" config_host

let () = Printf.printf "Server: %s:%d\n" host port
let () = Printf.printf "Port set: %b, Host set: %b\n"
  (Option.is_some config_port) (Option.is_some config_host)
