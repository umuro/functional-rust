(* 1005: Error Chaining *)
(* Adding context to errors as they propagate *)

type inner_error = NotFound | Corrupted of string
type app_error = { context: string; cause: inner_error }

let string_of_inner = function
  | NotFound -> "not found"
  | Corrupted s -> Printf.sprintf "corrupted: %s" s

let string_of_app_error e =
  Printf.sprintf "%s: %s" e.context (string_of_inner e.cause)

(* Approach 1: Manual context wrapping *)
let read_file path =
  if path = "/missing" then Error NotFound
  else if path = "/bad" then Error (Corrupted "invalid utf-8")
  else Ok "data"

let load_with_context path =
  match read_file path with
  | Error e -> Error { context = Printf.sprintf "loading %s" path; cause = e }
  | Ok v -> Ok v

(* Approach 2: Generic context helper (like Rust's map_err) *)
let with_context ctx result =
  match result with
  | Error e -> Error { context = ctx; cause = e }
  | Ok v -> Ok v

let load_with_helper path =
  read_file path |> with_context (Printf.sprintf "loading %s" path)

let test_manual () =
  (match load_with_context "/missing" with
   | Error e ->
     assert (e.context = "loading /missing");
     assert (e.cause = NotFound)
   | Ok _ -> assert false);
  assert (load_with_context "/ok" = Ok "data");
  Printf.printf "  Approach 1 (manual context): passed\n"

let test_helper () =
  (match load_with_helper "/bad" with
   | Error e ->
     assert (String.length (string_of_app_error e) > 0);
     assert (e.cause = Corrupted "invalid utf-8")
   | Ok _ -> assert false);
  assert (load_with_helper "/ok" = Ok "data");
  Printf.printf "  Approach 2 (context helper): passed\n"

let () =
  Printf.printf "Testing error chaining:\n";
  test_manual ();
  test_helper ();
  Printf.printf "✓ All tests passed\n"
