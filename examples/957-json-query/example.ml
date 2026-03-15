(* 957: JSON Query by Path *)

type json =
  | Null
  | Bool of bool
  | Number of float
  | Str of string
  | Array of json list
  | Object of (string * json) list

(* Approach 1: Path query returning Option *)

let rec get path json =
  match path, json with
  | [], j -> Some j
  | key :: rest, Object pairs ->
    (match List.assoc_opt key pairs with
     | Some v -> get rest v
     | None -> None)
  | idx :: rest, Array items ->
    (match int_of_string_opt idx with
     | Some i when i >= 0 && i < List.length items ->
       get rest (List.nth items i)
     | _ -> None)
  | _ -> None

(* Approach 2: Extract typed values *)

let get_string path json =
  match get path json with
  | Some (Str s) -> Some s
  | _ -> None

let get_number path json =
  match get path json with
  | Some (Number n) -> Some n
  | _ -> None

let get_bool path json =
  match get path json with
  | Some (Bool b) -> Some b
  | _ -> None

let get_array path json =
  match get path json with
  | Some (Array items) -> Some items
  | _ -> None

(* Approach 3: Query with default *)

let get_or default path json =
  match get path json with
  | Some v -> v
  | None -> default

let () =
  let json = Object [
    ("users", Array [
      Object [("name", Str "Alice"); ("age", Number 30.0); ("active", Bool true)];
      Object [("name", Str "Bob");   ("age", Number 25.0); ("active", Bool false)];
    ]);
    ("count", Number 2.0);
    ("meta", Object [("version", Str "1.0"); ("tag", Null)]);
  ] in

  (* Basic path queries *)
  assert (get ["count"] json = Some (Number 2.0));
  assert (get ["users"; "0"; "name"] json = Some (Str "Alice"));
  assert (get ["users"; "1"; "name"] json = Some (Str "Bob"));
  assert (get ["users"; "0"; "active"] json = Some (Bool true));
  assert (get ["users"; "1"; "active"] json = Some (Bool false));
  assert (get ["meta"; "version"] json = Some (Str "1.0"));
  assert (get ["meta"; "tag"] json = Some Null);

  (* Missing paths return None *)
  assert (get ["missing"] json = None);
  assert (get ["users"; "5"; "name"] json = None);
  assert (get ["users"; "0"; "missing"] json = None);

  (* Typed extractors *)
  assert (get_string ["users"; "0"; "name"] json = Some "Alice");
  assert (get_number ["count"] json = Some 2.0);
  assert (get_bool ["users"; "0"; "active"] json = Some true);

  (* Empty path returns whole document *)
  assert (get [] json = Some json);

  Printf.printf "✓ All tests passed\n"
