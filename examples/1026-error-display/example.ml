(* 1026: Error Display — Walking Nested Error Source Chains
   Rust's Error::source() walks a chain of wrapped errors.
   In OCaml we model a nested error chain explicitly as a record
   and walk it recursively. *)

(* Root cause error *)
type io_error =
  | FileNotFound of string
  | PermissionDenied of string

let string_of_io_error = function
  | FileNotFound p     -> Printf.sprintf "file not found: %s" p
  | PermissionDenied p -> Printf.sprintf "permission denied: %s" p

(* Middle layer: wraps io_error *)
type config_error = {
  operation: string;
  config_source: io_error;
}

let string_of_config_error e =
  Printf.sprintf "%s failed" e.operation

(* Outer layer: wraps config_error *)
type app_error = {
  module_name: string;
  app_source: config_error;
}

let string_of_app_error e =
  Printf.sprintf "[%s] error" e.module_name

(* Approach 1: Walk the chain manually *)
let display_error_chain (e : app_error) =
  let lines = [
    Printf.sprintf "Error: %s" (string_of_app_error e);
    Printf.sprintf "  Caused by: %s" (string_of_config_error e.app_source);
    Printf.sprintf "    Caused by: %s" (string_of_io_error e.app_source.config_source);
  ] in
  String.concat "\n" lines

(* Approach 2: Collect all messages into a list *)
let error_sources (e : app_error) = [
  string_of_app_error e;
  string_of_config_error e.app_source;
  string_of_io_error e.app_source.config_source;
]

(* Approach 3: Single-line display with arrows *)
let display_inline e =
  String.concat " -> " (error_sources e)

let make_error () = {
  module_name = "config";
  app_source = {
    operation = "reading settings";
    config_source = FileNotFound "/etc/app.conf";
  }
}

let () =
  let err = make_error () in

  (* display_error_chain has 3 lines *)
  let chain = display_error_chain err in
  assert (try let _ = Str.search_forward (Str.regexp "Error:") chain 0 in true
          with Not_found -> false);
  assert (try let _ = Str.search_forward (Str.regexp "Caused by") chain 0 in true
          with Not_found -> false);
  assert (try let _ = Str.search_forward (Str.regexp "file not found") chain 0 in true
          with Not_found -> false);
  let lines = String.split_on_char '\n' chain in
  assert (List.length lines = 3);

  (* error_sources *)
  let sources = error_sources err in
  assert (List.length sources = 3);
  assert (List.nth sources 0 = "[config] error");
  assert (List.nth sources 1 = "reading settings failed");
  assert (try let _ = Str.search_forward (Str.regexp "file not found") (List.nth sources 2) 0 in true
          with Not_found -> false);

  (* display_inline *)
  let inline = display_inline err in
  assert (try let _ = Str.search_forward (Str.regexp " -> ") inline 0 in true
          with Not_found -> false);
  assert (String.sub inline 0 7 = "[config");

  (* Level checks *)
  assert (string_of_app_error err = "[config] error");
  assert (string_of_config_error err.app_source = "reading settings failed");
  assert (try let _ = Str.search_forward (Str.regexp "file not found") (string_of_io_error err.app_source.config_source) 0 in true
          with Not_found -> false);

  (* Single-level error chain *)
  let io = FileNotFound "test.txt" in
  assert (string_of_io_error io = "file not found: test.txt");

  Printf.printf "Error chain: %s\n" (display_inline err)
