(* 1024: File Operation Errors
   OCaml's Unix module and Sys module provide file operations.
   Exceptions like Sys_error carry the OS error message.
   We classify exceptions into typed error variants. *)

(* Approach 1: Basic file operations — exceptions become Result via try/with *)
let read_file path =
  try Ok (In_channel.input_all (In_channel.open_text path))
  with Sys_error msg -> Error msg

let write_file path content =
  try
    Out_channel.with_open_text path (fun oc ->
      Out_channel.output_string oc content);
    Ok ()
  with Sys_error msg -> Error msg

(* Approach 2: Classify the error *)
type file_error =
  | FileNotFound of string
  | PermissionDenied of string
  | OtherFileError of string

let string_of_file_error = function
  | FileNotFound p        -> Printf.sprintf "file not found: %s" p
  | PermissionDenied p    -> Printf.sprintf "permission denied: %s" p
  | OtherFileError msg    -> Printf.sprintf "file error: %s" msg

let classify_sys_error path msg =
  (* Sys_error messages contain the path and OS description *)
  if String.length msg > 0 &&
     (try let _ = Str.search_forward (Str.regexp "No such file") msg 0 in true
      with Not_found -> false)
  then FileNotFound path
  else if
     (try let _ = Str.search_forward (Str.regexp "Permission denied") msg 0 in true
      with Not_found -> false)
  then PermissionDenied path
  else OtherFileError msg

let read_file_typed path =
  try Ok (In_channel.input_all (In_channel.open_text path))
  with Sys_error msg -> Error (classify_sys_error path msg)

(* Read if exists — returns None for missing files *)
let read_if_exists path =
  if Sys.file_exists path then
    (try Ok (Some (In_channel.input_all (In_channel.open_text path)))
     with Sys_error msg -> Error msg)
  else Ok None

let () =
  (* Read nonexistent — should error *)
  (match read_file "/nonexistent_file_12345" with
   | Error _ -> ()
   | Ok _    -> assert false);

  (* Write/read roundtrip *)
  let tmp = "/tmp/ocaml_test_1024.txt" in
  assert (write_file tmp "hello ocaml" = Ok ());
  assert (read_file tmp = Ok "hello ocaml");
  Sys.remove tmp;

  (* Typed error *)
  (match read_file_typed "/nonexistent_12345" with
   | Error (FileNotFound _) -> ()
   | Error e -> ignore (string_of_file_error e)
   | Ok _ -> assert false);

  (* read_if_exists *)
  assert (read_if_exists "/nonexistent_12345" = Ok None);

  let tmp2 = "/tmp/ocaml_test_1024b.txt" in
  assert (write_file tmp2 "exists" = Ok ());
  assert (read_if_exists tmp2 = Ok (Some "exists"));
  Sys.remove tmp2;

  Printf.printf "File error tests passed\n"
