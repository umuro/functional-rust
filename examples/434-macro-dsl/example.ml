(* DSL design with macros in OCaml *)

(* Configuration DSL *)
type config_value =
  | Str of string
  | Int of int
  | Bool of bool
  | List of config_value list

type config = (string * config_value) list

(* DSL-like function to build config *)
let ( --> ) key value = (key, value)

let config_str s = Str s
let config_int n = Int n
let config_bool b = Bool b
let config_list items = List (List.map (fun s -> Str s) items)

(* Simulate a config DSL *)
let app_config : config = [
  "host" --> config_str "localhost";
  "port" --> config_int 8080;
  "debug" --> config_bool true;
  "allowed_origins" --> config_list ["http://localhost:3000"; "http://localhost:8080"];
]

let rec show_value = function
  | Str s -> Printf.sprintf "%S" s
  | Int n -> string_of_int n
  | Bool b -> string_of_bool b
  | List vs -> "[" ^ String.concat ", " (List.map show_value vs) ^ "]"

let () =
  Printf.printf "Config:\n";
  List.iter (fun (k, v) ->
    Printf.printf "  %s = %s\n" k (show_value v)
  ) app_config
