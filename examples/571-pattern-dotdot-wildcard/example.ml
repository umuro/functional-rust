(* Wildcards in OCaml *)
type config = { host: string; port: int; timeout: float; debug: bool }

let connect { host; port; _ } =
  Printf.printf "Connecting to %s:%d\n" host port

type response = Ok of int * string * float | Err of string

let get_val = function
  | Ok (v, _, _) -> Some v
  | Err _        -> None

let () =
  let cfg = { host="localhost"; port=8080; timeout=30.0; debug=true } in
  connect cfg;
  List.iter (fun r ->
    match get_val r with
    | Some v -> Printf.printf "Got: %d\n" v
    | None   -> Printf.printf "Error\n"
  ) [Ok(42,"msg",1.0); Err "oops"]
