(* 1014: Recover from Panic *)
(* OCaml try/with for catching exceptions *)

(* Approach 1: try/with — catch specific exceptions *)
let risky_divide a b =
  if b = 0 then failwith "division by zero"
  else a / b

let safe_divide a b =
  try Ok (risky_divide a b)
  with Failure msg -> Error msg

(* Approach 2: Catch all exceptions *)
let catch_all f =
  try Ok (f ())
  with
  | Failure msg -> Error (Printf.sprintf "Failure: %s" msg)
  | Invalid_argument msg -> Error (Printf.sprintf "Invalid: %s" msg)
  | exn -> Error (Printf.sprintf "Unknown: %s" (Printexc.to_string exn))

(* Approach 3: Using Fun.protect for cleanup *)
let with_resource f =
  let resource = "opened" in
  Fun.protect
    ~finally:(fun () -> Printf.printf "    cleanup: resource closed\n")
    (fun () -> f resource)

let test_try_with () =
  assert (safe_divide 10 2 = Ok 5);
  (match safe_divide 10 0 with
   | Error msg -> assert (msg = "division by zero")
   | Ok _ -> assert false);
  Printf.printf "  Approach 1 (try/with): passed\n"

let test_catch_all () =
  assert (catch_all (fun () -> 42) = Ok 42);
  (match catch_all (fun () -> failwith "boom") with
   | Error msg -> assert (String.length msg > 0)
   | Ok _ -> assert false);
  (match catch_all (fun () -> invalid_arg "bad") with
   | Error msg -> assert (String.length msg > 0)
   | Ok _ -> assert false);
  Printf.printf "  Approach 2 (catch all): passed\n"

let test_protect () =
  let result = try Ok (with_resource (fun r -> String.length r))
               with _ -> Error "failed" in
  assert (result = Ok 6);
  Printf.printf "  Approach 3 (Fun.protect): passed\n"

let () =
  Printf.printf "Testing recover from panic:\n";
  test_try_with ();
  test_catch_all ();
  test_protect ();
  Printf.printf "✓ All tests passed\n"
