(* Static lifetime analog in OCaml — module-level bindings *)
(* OCaml has no 'static concept — all values are GC-managed *)

(* Module-level bindings are "static" in the sense they live forever *)
let app_name = "MyApp"
let version  = "1.0.0"
let max_connections = 100

(* Lookup table (const equivalent) *)
let error_messages = [
  (404, "Not Found");
  (500, "Internal Server Error");
  (403, "Forbidden");
]

let get_error_msg code =
  List.assoc_opt code error_messages
  |> Option.value ~default:"Unknown Error"

let () =
  Printf.printf "%s v%s\n" app_name version;
  Printf.printf "Max connections: %d\n" max_connections;
  Printf.printf "Error 404: %s\n" (get_error_msg 404);
  Printf.printf "Error 418: %s\n" (get_error_msg 418)
