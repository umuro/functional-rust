(* 1016: Error Context *)
(* Adding context/backtrace to errors manually *)

type error_with_context = {
  message: string;
  context: string list;  (* stack of context strings *)
}

let make_error msg = { message = msg; context = [] }

let add_context ctx err =
  { err with context = ctx :: err.context }

let display_error err =
  let chain = String.concat " -> " (List.rev err.context) in
  if chain = "" then err.message
  else Printf.sprintf "%s: %s" chain err.message

(* Approach 1: Manual context threading *)
let read_file path =
  if path = "/missing" then Error (make_error "file not found")
  else Ok "42"

let parse_config content =
  match int_of_string_opt content with
  | None -> Error (make_error "invalid number")
  | Some n -> Ok n

let load_setting path =
  match read_file path with
  | Error e -> Error (add_context "reading config" e)
  | Ok content ->
    match parse_config content with
    | Error e -> Error (add_context "parsing config" e)
    | Ok n -> Ok n

let init_system path =
  match load_setting path with
  | Error e -> Error (add_context "system init" e)
  | Ok n -> Ok n

(* Approach 2: Result pipe with context *)
let ( >>| ) r ctx =
  match r with
  | Ok v -> Ok v
  | Error e -> Error (add_context ctx e)

let load_setting_pipe path =
  (read_file path >>| "reading config")
  |> Result.bind (fun content ->
       parse_config content >>| "parsing config")

let test_manual () =
  (match init_system "/missing" with
   | Error e ->
     let msg = display_error e in
     assert (String.length msg > 0);
     assert (List.length e.context = 2)
   | Ok _ -> assert false);
  assert (init_system "/ok" = Ok 42);
  Printf.printf "  Approach 1 (manual context): passed\n"

let test_pipe () =
  (match load_setting_pipe "/missing" with
   | Error e -> assert (List.length e.context = 1)
   | Ok _ -> assert false);
  assert (load_setting_pipe "/ok" = Ok 42);
  Printf.printf "  Approach 2 (pipe with context): passed\n"

let () =
  Printf.printf "Testing error context:\n";
  test_manual ();
  test_pipe ();
  Printf.printf "✓ All tests passed\n"
