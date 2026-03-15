(* 062: Records *)
(* Named fields, creation, update, pattern matching *)

(* Approach 1: Basic record *)
type point = { x: float; y: float }

let origin = { x = 0.0; y = 0.0 }
let p1 = { x = 3.0; y = 4.0 }

let distance p1 p2 =
  let dx = p1.x -. p2.x in
  let dy = p1.y -. p2.y in
  sqrt (dx *. dx +. dy *. dy)

(* Approach 2: Functional update *)
type config = {
  host: string;
  port: int;
  debug: bool;
  timeout: int;
}

let default_config = {
  host = "localhost";
  port = 8080;
  debug = false;
  timeout = 30;
}

let dev_config = { default_config with debug = true; port = 3000 }
let prod_config = { default_config with host = "prod.example.com"; timeout = 60 }

(* Approach 3: Pattern matching on records *)
let describe_config { host; port; debug; _ } =
  Printf.sprintf "%s:%d%s" host port (if debug then " [DEBUG]" else "")

let is_local { host; _ } =
  host = "localhost" || host = "127.0.0.1"

(* Tests *)
let () =
  assert (origin.x = 0.0);
  assert (abs_float (distance origin p1 -. 5.0) < 0.001);
  assert (dev_config.debug = true);
  assert (dev_config.port = 3000);
  assert (dev_config.host = "localhost");
  assert (prod_config.timeout = 60);
  assert (describe_config dev_config = "localhost:3000 [DEBUG]");
  assert (is_local default_config);
  assert (not (is_local prod_config));
  Printf.printf "✓ All tests passed\n"
