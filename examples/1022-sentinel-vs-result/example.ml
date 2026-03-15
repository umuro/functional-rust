(* 1022: Sentinel Values vs Result
   C uses -1 / "" as sentinels for "not found". OCaml and Rust prefer
   Option for "maybe absent" and Result for "absent with a reason".
   Demonstrates the migration pattern: sentinel → Option → Result *)

(* Approach 1: Sentinel values — the C way (avoid in OCaml) *)
let find_index_sentinel haystack needle =
  let found = ref (-1) in
  List.iteri (fun i v -> if v = needle then found := i) haystack;
  !found  (* -1 = "not found" *)

let get_config_sentinel key =
  match key with
  | "port" -> "8080"
  | _      -> ""  (* "" = "missing" — ambiguous! *)

(* Approach 2: Option — explicit absence (PREFERRED for lookups) *)
let find_index haystack needle =
  let rec loop i = function
    | [] -> None
    | x :: _ when x = needle -> Some i
    | _ :: rest -> loop (i + 1) rest
  in
  loop 0 haystack

let get_config key =
  match key with
  | "port" -> Some "8080"
  | _      -> None

(* Approach 3: Result — absence with reason *)
let find_index_result haystack needle =
  match find_index haystack needle with
  | Some i -> Ok i
  | None   -> Error (Printf.sprintf "%s not in list" needle)

let get_config_result key =
  match key with
  | "port" -> Ok "8080"
  | _      -> Error (Printf.sprintf "key not found: %s" key)

(* Migration wrapper: sentinel int → Option *)
let migrate_sentinel v =
  if v = -1 then None else Some v

let () =
  assert (find_index_sentinel [1; 2; 3] 2 = 1);
  assert (find_index_sentinel [1; 2; 3] 9 = -1);

  assert (find_index [1; 2; 3] 2 = Some 1);
  assert (find_index [1; 2; 3] 9 = None);

  assert (find_index_result ["a"; "b"; "c"] "b" = Ok 1);
  (match find_index_result ["a"; "b"] "z" with
   | Error msg -> assert (String.length msg > 0)
   | _ -> assert false);

  (* Ambiguity with sentinels *)
  assert (get_config_sentinel "missing" = "");
  assert (get_config "missing" = None);  (* clear: absent *)

  assert (get_config_result "port" = Ok "8080");
  assert (Result.is_error (get_config_result "unknown"));

  (* Migration pattern *)
  assert (migrate_sentinel (find_index_sentinel [1; 2] 2) = Some 1);
  assert (migrate_sentinel (find_index_sentinel [1; 2] 9) = None);

  Printf.printf "find_index [1;2;3] 2 = %s\n"
    (match find_index [1; 2; 3] 2 with
     | Some i -> string_of_int i
     | None -> "not found")
