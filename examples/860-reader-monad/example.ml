(* Example 061: Reader Monad *)
(* Dependency injection via implicit environment passing *)

type ('r, 'a) reader = Reader of ('r -> 'a)

let run_reader (Reader f) env = f env
let return_ x = Reader (fun _ -> x)
let bind m f = Reader (fun env ->
  let a = run_reader m env in
  run_reader (f a) env)
let ( >>= ) = bind
let ask = Reader (fun env -> env)
let asks f = Reader (fun env -> f env)

(* Approach 1: Configuration-based computation *)
type config = { db_host: string; db_port: int; debug: bool }

let get_connection_string =
  asks (fun c -> Printf.sprintf "%s:%d" c.db_host c.db_port)

let get_log_prefix =
  asks (fun c -> if c.debug then "[DEBUG] " else "[INFO] ")

let format_message msg =
  get_log_prefix >>= fun prefix ->
  get_connection_string >>= fun conn ->
  return_ (Printf.sprintf "%s%s (connected to %s)" prefix msg conn)

(* Approach 2: Nested reader computations *)
let greet name =
  asks (fun (lang : string) ->
    if lang = "en" then "Hello, " ^ name
    else if lang = "nl" then "Hallo, " ^ name
    else "Hi, " ^ name)

(* Approach 3: Combining multiple environment reads *)
type app_env = { user: string; locale: string; version: int }

let welcome =
  asks (fun e -> e.user) >>= fun user ->
  asks (fun e -> e.locale) >>= fun locale ->
  asks (fun e -> e.version) >>= fun ver ->
  return_ (Printf.sprintf "Welcome %s! Locale: %s, v%d" user locale ver)

let () =
  let cfg = { db_host = "localhost"; db_port = 5432; debug = true } in
  let msg = run_reader (format_message "Starting") cfg in
  assert (msg = "[DEBUG] Starting (connected to localhost:5432)");

  let cfg2 = { cfg with debug = false } in
  let msg2 = run_reader (format_message "Starting") cfg2 in
  assert (msg2 = "[INFO] Starting (connected to localhost:5432)");

  assert (run_reader (greet "Umur") "nl" = "Hallo, Umur");
  assert (run_reader (greet "Umur") "en" = "Hello, Umur");

  let env = { user = "Umur"; locale = "nl_NL"; version = 3 } in
  let w = run_reader welcome env in
  assert (w = "Welcome Umur! Locale: nl_NL, v3");

  Printf.printf "✓ All tests passed\n"
