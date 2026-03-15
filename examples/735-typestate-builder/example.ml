(* 735: Typestate Builder — OCaml approach with labeled arguments
   OCaml uses labeled/optional arguments for similar effect, but
   compile-time enforcement of required fields needs GADTs. *)

(* In OCaml, we can use phantom types + GADT to mimic typestate builder *)
type unset = Unset
type set = Set

(* Simple simulation: we use options and fail at build time *)
(* A truly type-safe builder in OCaml would use GADT witnesses *)
type ('name, 'port) http_builder = {
  name: string option;
  port: int option;
  timeout_ms: int;
}

let new_builder () : (unset, unset) http_builder =
  { name = None; port = None; timeout_ms = 5000 }

(* Note: OCaml can't enforce 'name = set here without GADTs *)
let set_name b name = { b with name = Some name }
let set_port b port = { b with port = Some port }
let set_timeout b t = { b with timeout_ms = t }

type http_client = { name: string; port: int; timeout_ms: int }

let build b =
  match b.name, b.port with
  | Some name, Some port -> { name; port; timeout_ms = b.timeout_ms }
  | None, _ -> failwith "name is required"   (* runtime, not compile time *)
  | _, None -> failwith "port is required"

let () =
  let client =
    new_builder ()
    |> (fun b -> set_name b "api.example.com")
    |> (fun b -> set_port b 8080)
    |> (fun b -> set_timeout b 3000)
    |> build
  in
  Printf.printf "Client: %s:%d timeout=%dms\n"
    client.name client.port client.timeout_ms
