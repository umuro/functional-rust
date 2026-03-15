(* 1024: File Operation Errors *)
(* Handling file I/O errors in OCaml *)

(* Approach 1: Exception-based file I/O *)
let read_file_exn path =
  try
    let ic = open_in path in
    let content = really_input_string ic (in_channel_length ic) in
    close_in ic;
    content
  with
  | Sys_error msg -> failwith (Printf.sprintf "IO error: %s" msg)

(* Approach 2: Result-based file I/O *)
type io_error =
  | FileNotFound of string
  | PermissionDenied of string
  | OtherIO of string

let read_file path =
  try
    let ic = open_in path in
    let content = really_input_string ic (in_channel_length ic) in
    close_in ic;
    Ok content
  with
  | Sys_error msg when String.length msg > 0 ->
    if String.sub msg 0 (min 2 (String.length msg)) = "No" then
      Error (FileNotFound path)
    else
      Error (OtherIO msg)

let write_file path content =
  try
    let oc = open_out path in
    output_string oc content;
    close_out oc;
    Ok ()
  with Sys_error msg -> Error (OtherIO msg)

(* Approach 3: Safe file operations with cleanup *)
let with_file path f =
  let ic = open_in path in
  Fun.protect ~finally:(fun () -> close_in_noerr ic) (fun () -> f ic)

let test_exception () =
  (* Can't really test file ops without temp files, so test the error path *)
  (try
     let _ = read_file_exn "/nonexistent_file_12345" in
     assert false
   with Failure msg -> assert (String.length msg > 0));
  Printf.printf "  Approach 1 (exception-based): passed\n"

let test_result () =
  (match read_file "/nonexistent_file_12345" with
   | Error _ -> ()
   | Ok _ -> assert false);
  Printf.printf "  Approach 2 (result-based): passed\n"

let test_write_read () =
  let tmp = "/tmp/ocaml_test_1024.txt" in
  assert (write_file tmp "hello" = Ok ());
  (match read_file tmp with
   | Ok content -> assert (content = "hello")
   | Error _ -> assert false);
  Sys.remove tmp;
  Printf.printf "  Write/read round-trip: passed\n"

let () =
  Printf.printf "Testing file errors:\n";
  test_exception ();
  test_result ();
  test_write_read ();
  Printf.printf "✓ All tests passed\n"
