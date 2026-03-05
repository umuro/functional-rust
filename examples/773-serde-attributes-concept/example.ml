(* Serde attributes concept in OCaml — manual implementation *)

(* We demonstrate: rename, skip, flatten by writing them by hand *)

(* ── Domain types ─────────────────────────────────────────────────────────────── *)
type address = { street: string; city: string }

(* Fields: user_id (rename→"id"), name, password (skip), address (flatten) *)
type user = {
  user_id   : int;    (* serialize as "id" *)
  name      : string;
  password  : string; (* skip on serialize *)
  address   : address;(* flatten: merge into parent *)
}

(* ── Serialize ────────────────────────────────────────────────────────────────── *)
let serialize_user u =
  (* rename: user_id → "id" *)
  let pairs = [
    ("id",     string_of_int u.user_id);     (* renamed *)
    ("name",   u.name);
    (* password: skipped *)
    (* flatten address: merge its fields *)
    ("street", u.address.street);            (* flattened *)
    ("city",   u.address.city);              (* flattened *)
  ] in
  List.map (fun (k, v) -> k ^ "=" ^ v) pairs |> String.concat "|"

(* ── Deserialize ─────────────────────────────────────────────────────────────── *)
let fields s =
  String.split_on_char '|' s
  |> List.filter_map (fun p ->
    match String.split_on_char '=' p with
    | [k; v] -> Some (k, v)
    | _ -> None)

let deserialize_user s =
  let f = fields s in
  let get k = match List.assoc_opt k f with Some v -> v | None -> "" in
  { user_id  = (try int_of_string (get "id") with _ -> 0);  (* rename *)
    name     = get "name";
    password = "";                                           (* skip: default empty *)
    address  = { street = get "street"; city = get "city" }; (* unflatten *)
  }

let () =
  let u = { user_id = 42; name = "Alice"; password = "secret";
             address = { street = "Main St"; city = "Berlin" } } in
  let wire = serialize_user u in
  Printf.printf "Wire  : %s\n" wire;
  let u2 = deserialize_user wire in
  Printf.printf "name  : %s\n" u2.name;
  Printf.printf "id    : %d\n" u2.user_id;
  Printf.printf "city  : %s\n" u2.address.city;
  Printf.printf "pass  : '%s' (skipped → default)\n" u2.password
