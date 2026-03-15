(* 957: JSON Query by Path

   get ["users"; "0"; "name"] json → Some value
   Numeric strings index into arrays; other strings look up object keys. *)

type json =
  | Null
  | Bool   of bool
  | Number of float
  | Str    of string
  | Array  of json list
  | Object of (string * json) list

(* ── Path query returning Option ─────────────────────────────────────────── *)

let rec get path json =
  match path with
  | []             -> Some json
  | key :: rest ->
    match json with
    | Object pairs ->
      (match List.assoc_opt key pairs with
       | None -> None
       | Some v -> get rest v)
    | Array items ->
      (match int_of_string_opt key with
       | None -> None
       | Some idx ->
         if idx < 0 || idx >= List.length items then None
         else get rest (List.nth items idx))
    | _ -> None

(* ── Typed extractors ────────────────────────────────────────────────────── *)

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
  | Some (Array a) -> Some a
  | _ -> None

(* ── Query with default ──────────────────────────────────────────────────── *)

let get_or default path json =
  match get path json with
  | Some v -> v
  | None   -> default

(* ── Update at path (functional: returns new JSON) ──────────────────────── *)

let rec set path value json =
  match path with
  | [] -> value
  | [key] ->
    (match json with
     | Object pairs ->
       let pairs' = List.filter (fun (k, _) -> k <> key) pairs in
       Object ((key, value) :: pairs')
     | _ -> Object [(key, value)])
  | key :: rest ->
    (match json with
     | Object pairs ->
       let existing = Option.value ~default:Null (List.assoc_opt key pairs) in
       let updated  = set rest value existing in
       let pairs'   = List.filter (fun (k, _) -> k <> key) pairs in
       Object ((key, updated) :: pairs')
     | _ -> Object [(key, set rest value Null)])

let () =
  let json =
    Object [
      ("users", Array [
        Object [
          ("name",   Str "Alice");
          ("age",    Number 30.0);
          ("active", Bool true);
        ];
        Object [
          ("name",   Str "Bob");
          ("age",    Number 25.0);
          ("active", Bool false);
        ];
      ]);
      ("count", Number 2.0);
      ("meta", Object [
        ("version", Str "1.0");
        ("tag",     Null);
      ]);
    ]
  in

  (* basic queries *)
  assert (get ["count"] json = Some (Number 2.0));
  assert (get ["users"; "0"; "name"] json = Some (Str "Alice"));
  assert (get ["users"; "1"; "name"] json = Some (Str "Bob"));
  assert (get ["meta"; "tag"] json = Some Null);

  (* missing paths *)
  assert (get ["missing"] json = None);
  assert (get ["users"; "5"; "name"] json = None);
  assert (get ["users"; "0"; "missing"] json = None);

  (* typed extractors *)
  assert (get_string ["users"; "0"; "name"] json = Some "Alice");
  assert (get_number ["count"] json = Some 2.0);
  assert (get_bool ["users"; "0"; "active"] json = Some true);
  assert (get_bool ["users"; "1"; "active"] json = Some false);

  (* empty path returns root *)
  assert (get [] json = Some json);

  (* get_or *)
  assert (get_or (Str "default") ["missing"] json = Str "default");
  assert (get_or Null ["count"] json = Number 2.0);

  (* get_array *)
  assert (match get_array ["users"] json with
          | Some lst -> List.length lst = 2
          | None -> false);

  (* set: functional update *)
  let updated = set ["count"] (Number 99.0) json in
  assert (get ["count"] updated = Some (Number 99.0));
  assert (get ["count"] json    = Some (Number 2.0));  (* original unchanged *)

  print_endline "957-json-query: all tests passed"
