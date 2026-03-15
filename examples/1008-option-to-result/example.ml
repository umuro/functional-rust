(* 1008: Option to Result Conversion *)
(* Converting Option to Result with error context *)

(* Approach 1: Pattern matching *)
let find_user_manual users name =
  match List.assoc_opt name users with
  | Some user -> Ok user
  | None -> Error (Printf.sprintf "user not found: %s" name)

(* Approach 2: Helper function (like ok_or) *)
let ok_or error = function
  | Some v -> Ok v
  | None -> Error error

let ok_or_else error_fn = function
  | Some v -> Ok v
  | None -> Error (error_fn ())

let find_user users name =
  List.assoc_opt name users
  |> ok_or_else (fun () -> Printf.sprintf "user not found: %s" name)

(* Approach 3: Chaining Option-to-Result in a pipeline *)
let find_and_validate users name min_age =
  List.assoc_opt name users
  |> ok_or_else (fun () -> Printf.sprintf "user not found: %s" name)
  |> Result.bind (fun (_, age) ->
       if age >= min_age then Ok (name, age)
       else Error (Printf.sprintf "%s is too young (%d < %d)" name age min_age))

let users = [("Alice", ("alice@ex.com", 30)); ("Bob", ("bob@ex.com", 17))]

let test_manual () =
  assert (find_user_manual users "Alice" = Ok ("alice@ex.com", 30));
  assert (find_user_manual users "Unknown" = Error "user not found: Unknown");
  Printf.printf "  Approach 1 (manual): passed\n"

let test_ok_or () =
  assert (ok_or "missing" (Some 42) = Ok 42);
  assert (ok_or "missing" None = Error "missing");
  assert (find_user users "Alice" = Ok ("alice@ex.com", 30));
  Printf.printf "  Approach 2 (ok_or helper): passed\n"

let test_chain () =
  assert (find_and_validate users "Alice" 18 = Ok ("Alice", 30));
  (match find_and_validate users "Bob" 18 with
   | Error msg -> assert (String.length msg > 0)
   | Ok _ -> assert false);
  (match find_and_validate users "Unknown" 18 with
   | Error msg -> assert (msg = "user not found: Unknown")
   | Ok _ -> assert false);
  Printf.printf "  Approach 3 (chained pipeline): passed\n"

let () =
  Printf.printf "Testing option to result:\n";
  test_manual ();
  test_ok_or ();
  test_chain ();
  Printf.printf "✓ All tests passed\n"
