(* Versioned serialization with migration in OCaml *)

(* ── V1 schema: name + age *)
type user_v1 = { name: string; age: int }

(* ── V2 schema: name + age + email (new field) *)
type user_v2 = { name: string; age: int; email: string }

(* ── Migration: V1 → V2 *)
let migrate_v1_to_v2 (u: user_v1) : user_v2 =
  { name  = u.name;
    age   = u.age;
    email = u.name ^ "@example.com" }  (* synthesized default *)

(* ── Versioned union *)
type versioned_user =
  | V1User of user_v1
  | V2User of user_v2

(* ── Serialize *)
let serialize_v2 u =
  Printf.sprintf "version=2|name=%s|age=%d|email=%s" u.name u.age u.email

let serialize_v1 u =
  Printf.sprintf "version=1|name=%s|age=%d" u.name u.age

(* ── Deserialize with migration *)
let field pairs key =
  match List.assoc_opt key pairs with
  | Some v -> Ok v
  | None   -> Error ("missing field: " ^ key)

let parse_pairs s =
  String.split_on_char '|' s
  |> List.filter_map (fun p ->
    match String.split_on_char '=' p with
    | [k; v] -> Some (k, v)
    | _ -> None)

let deserialize s =
  let pairs = parse_pairs s in
  match field pairs "version" with
  | Error e -> Error e
  | Ok "1" ->
    (match field pairs "name", field pairs "age" with
     | Ok name, Ok age_s ->
       (try
         let u1 = V1User { name; age = int_of_string age_s } in
         Ok u1
        with Failure e -> Error e)
     | Error e, _ | _, Error e -> Error e)
  | Ok "2" ->
    (match field pairs "name", field pairs "age", field pairs "email" with
     | Ok name, Ok age_s, Ok email ->
       (try Ok (V2User { name; age = int_of_string age_s; email })
        with Failure e -> Error e)
     | Error e, _, _ | _, Error e, _ | _, _, Error e -> Error e)
  | Ok v -> Error ("unsupported version: " ^ v)

(* Normalize to V2 (migrating if needed) *)
let to_v2 = function
  | V1User u1 -> migrate_v1_to_v2 u1
  | V2User u2 -> u2

let () =
  (* Write in old format, read as new *)
  let old_data = serialize_v1 { name = "Alice"; age = 30 } in
  Printf.printf "Old wire: %s\n" old_data;
  match deserialize old_data with
  | Ok v ->
    let u2 = to_v2 v in
    Printf.printf "Migrated: name=%s age=%d email=%s\n" u2.name u2.age u2.email
  | Error e -> Printf.printf "Error: %s\n" e
