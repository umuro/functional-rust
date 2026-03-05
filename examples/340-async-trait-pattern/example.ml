(* OCaml: Async trait pattern with module types *)

module type ASYNC_STORE = sig
  type t
  val get : t -> string -> string option Lwt.t
  val set : t -> string -> string -> unit Lwt.t
end

(* In-memory implementation *)
module MemStore : ASYNC_STORE with type t = (string, string) Hashtbl.t = struct
  type t = (string, string) Hashtbl.t
  let get tbl key = Lwt.return (Hashtbl.find_opt tbl key)
  let set tbl key value = Hashtbl.replace tbl key value; Lwt.return ()
end

(* Failing implementation *)
module FailStore : ASYNC_STORE with type t = unit = struct
  type t = unit
  let get () _ = Lwt.fail_with "connection refused"
  let set () _ _ = Lwt.fail_with "read-only"
end

(* Without Lwt, sync simulation: *)
let () =
  let tbl = Hashtbl.create 16 in
  Hashtbl.add tbl "key" "value";
  match Hashtbl.find_opt tbl "key" with
  | Some v -> Printf.printf "Got: %s\n" v
  | None -> Printf.printf "Not found\n"
