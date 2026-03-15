(* 1026: Custom Display for Nested Errors *)
(* Building a source chain display for nested errors *)

type inner_error =
  | IoFailed of string
  | ParseFailed of string

type middle_error = {
  operation: string;
  inner: inner_error;
}

type outer_error = {
  module_name: string;
  cause: middle_error;
}

let string_of_inner = function
  | IoFailed msg -> Printf.sprintf "IO failed: %s" msg
  | ParseFailed msg -> Printf.sprintf "parse failed: %s" msg

let string_of_middle e =
  Printf.sprintf "%s: %s" e.operation (string_of_inner e.inner)

let string_of_outer e =
  Printf.sprintf "[%s] %s" e.module_name (string_of_middle e.cause)

(* Approach 1: Full chain display *)
let display_chain e =
  let lines = [
    Printf.sprintf "Error: %s" (string_of_outer e);
    Printf.sprintf "  Caused by: %s" (string_of_middle e.cause);
    Printf.sprintf "  Root cause: %s" (string_of_inner e.cause.inner);
  ] in
  String.concat "\n" lines

(* Approach 2: Generic error chain *)
type error_node = {
  message: string;
  source: error_node option;
}

let rec display_error_chain depth node =
  let prefix = String.make (depth * 2) ' ' in
  let label = if depth = 0 then "Error" else "Caused by" in
  let line = Printf.sprintf "%s%s: %s" prefix label node.message in
  match node.source with
  | None -> line
  | Some src -> line ^ "\n" ^ display_error_chain (depth + 1) src

let test_nested_display () =
  let err = {
    module_name = "config";
    cause = {
      operation = "reading settings";
      inner = IoFailed "file not found";
    }
  } in
  let s = string_of_outer err in
  assert (String.length s > 0);
  assert (s = "[config] reading settings: IO failed: file not found");
  Printf.printf "  Nested display: passed\n"

let test_chain_display () =
  let err = {
    message = "[config] reading settings failed";
    source = Some {
      message = "IO failed";
      source = Some {
        message = "file not found: /etc/app.conf";
        source = None;
      }
    }
  } in
  let output = display_error_chain 0 err in
  assert (String.length output > 0);
  let lines = String.split_on_char '\n' output in
  assert (List.length lines = 3);
  Printf.printf "  Chain display: passed\n"

let test_single_error () =
  let err = { message = "simple error"; source = None } in
  assert (display_error_chain 0 err = "Error: simple error");
  Printf.printf "  Single error: passed\n"

let () =
  Printf.printf "Testing error display:\n";
  test_nested_display ();
  test_chain_display ();
  test_single_error ();
  Printf.printf "✓ All tests passed\n"
