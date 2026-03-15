(* 1022: Sentinel Values vs Result *)
(* Migrating from sentinel values (-1, null, "") to Option/Result *)

(* Approach 1: Sentinel values — the bad old way *)
let find_index_sentinel lst target =
  let rec aux i = function
    | [] -> -1  (* sentinel: "not found" *)
    | x :: _ when x = target -> i
    | _ :: rest -> aux (i + 1) rest
  in aux 0 lst

let get_config_sentinel key =
  if key = "port" then "8080"
  else ""  (* sentinel: "missing" *)

(* Approach 2: Option — explicit absence *)
let find_index lst target =
  let rec aux i = function
    | [] -> None
    | x :: _ when x = target -> Some i
    | _ :: rest -> aux (i + 1) rest
  in aux 0 lst

let get_config key =
  if key = "port" then Some "8080"
  else None

(* Approach 3: Result — absence with reason *)
let find_index_result lst target =
  let rec aux i = function
    | [] -> Error (Printf.sprintf "%s not in list" target)
    | x :: _ when x = target -> Ok i
    | _ :: rest -> aux (i + 1) rest
  in aux 0 lst

let get_config_result key =
  if key = "port" then Ok "8080"
  else Error (Printf.sprintf "key not found: %s" key)

let test_sentinel () =
  assert (find_index_sentinel [1; 2; 3] 2 = 1);
  assert (find_index_sentinel [1; 2; 3] 9 = -1);
  (* Bug: what if -1 is a valid value? Sentinel is ambiguous *)
  assert (get_config_sentinel "port" = "8080");
  assert (get_config_sentinel "missing" = "");
  (* Bug: what if "" is a valid config value? *)
  Printf.printf "  Approach 1 (sentinel values): passed\n"

let test_option () =
  assert (find_index [1; 2; 3] 2 = Some 1);
  assert (find_index [1; 2; 3] 9 = None);
  assert (get_config "port" = Some "8080");
  assert (get_config "missing" = None);
  Printf.printf "  Approach 2 (Option): passed\n"

let test_result () =
  assert (find_index_result ["a";"b";"c"] "b" = Ok 1);
  (match find_index_result ["a";"b"] "z" with
   | Error msg -> assert (String.length msg > 0)
   | Ok _ -> assert false);
  assert (get_config_result "port" = Ok "8080");
  Printf.printf "  Approach 3 (Result): passed\n"

let () =
  Printf.printf "Testing sentinel vs result:\n";
  test_sentinel ();
  test_option ();
  test_result ();
  Printf.printf "✓ All tests passed\n"
