(* 062: Records
   Named fields, construction, functional update, destructuring *)

(* --- Approach 1: Basic record type --- *)

type point = { x: float; y: float }

let origin = { x = 0.0; y = 0.0 }

let distance p1 p2 =
  let dx = p1.x -. p2.x in
  let dy = p1.y -. p2.y in
  sqrt (dx *. dx +. dy *. dy)

(* --- Approach 2: Functional record update with { r with field = value } --- *)

type config = {
  host    : string;
  port    : int;
  debug   : bool;
  timeout : int;
}

let default_config = {
  host    = "localhost";
  port    = 8080;
  debug   = false;
  timeout = 30;
}

(* OCaml record update syntax mirrors Rust's struct update *)
let dev_config  = { default_config with debug = true; port = 3000 }
let prod_config = { default_config with host = "prod.example.com"; timeout = 60 }

(* --- Approach 3: Pattern-matching on records (destructuring) --- *)

let describe_config { host; port; debug; _ } =
  Printf.sprintf "%s:%d%s" host port (if debug then " [DEBUG]" else "")

let is_local { host; _ } =
  host = "localhost" || host = "127.0.0.1"

let () =
  let p = { x = 3.0; y = 4.0 } in
  Printf.printf "distance origin (3,4) = %.1f\n" (distance origin p);

  Printf.printf "dev:  %s\n" (describe_config dev_config);
  Printf.printf "prod: %s\n" (describe_config prod_config);

  Printf.printf "default is_local: %b\n" (is_local default_config);
  Printf.printf "prod is_local:    %b\n" (is_local prod_config)
