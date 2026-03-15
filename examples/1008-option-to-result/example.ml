(* 1008: Option to Result Conversion
   Option.to_result (available in OCaml 4.08+) converts Option to Result.
   The equivalent of Rust's ok_or / ok_or_else is built-in.
   Result.to_option converts back. *)

(* Build a simple "user database" using a Hashtbl *)
let build_users () =
  let tbl = Hashtbl.create 4 in
  Hashtbl.add tbl "Alice" ("alice@ex.com", 30);
  Hashtbl.add tbl "Bob"   ("bob@ex.com",   17);
  tbl

(* Approach 1: Option.to_result — eager error value *)
let find_user_eager users name =
  Option.to_result
    ~none:(Printf.sprintf "user not found: %s" name)
    (Hashtbl.find_opt users name)

(* Approach 2: Lazy error using match *)
let find_user_lazy users name =
  match Hashtbl.find_opt users name with
  | Some v -> Ok v
  | None   -> Error (Printf.sprintf "user not found: %s" name)

(* Approach 3: Chain Option -> Result into a pipeline *)
let find_and_validate users name min_age =
  let open Result in
  bind
    (Option.to_result ~none:(Printf.sprintf "user not found: %s" name)
       (Hashtbl.find_opt users name))
    (fun (email, age) ->
       if age >= min_age then Ok (email, age)
       else Error (Printf.sprintf "%s is too young (%d < %d)" name age min_age))

let () =
  let users = build_users () in

  (* ok_or equivalent *)
  assert (Result.is_ok (find_user_eager users "Alice"));
  (match find_user_eager users "Alice" with Ok (_, age) -> assert (age = 30) | _ -> assert false);
  assert (find_user_eager users "Unknown" = Error "user not found: Unknown");

  (* ok_or_else equivalent *)
  assert (Result.is_ok (find_user_lazy users "Bob"));
  assert (Result.is_error (find_user_lazy users "Nobody"));

  (* pipeline *)
  (match find_and_validate users "Alice" 18 with
   | Ok (email, age) ->
     assert (email = "alice@ex.com");
     assert (age = 30)
   | _ -> assert false);

  (match find_and_validate users "Bob" 18 with
   | Error msg -> assert (String.length msg > 0)
   | _ -> assert false);

  (* Direct Option ↔ Result conversions *)
  assert (Option.to_result ~none:"missing" (Some 42) = Ok 42);
  assert (Option.to_result ~none:"missing" None = Error "missing");
  assert (Result.to_option (Ok 42) = Some 42);
  assert (Result.to_option (Error "fail" : (int, string) result) = None);

  Printf.printf "Alice found: %b\n" (Result.is_ok (find_user_eager users "Alice"))
