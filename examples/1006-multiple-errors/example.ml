(* 1006: Multiple Error Types *)
(* Unifying multiple error types into one *)

type io_error = FileNotFound | ReadError of string
type parse_error = BadFormat | Overflow
type net_error = Timeout | ConnectionRefused

(* Approach 1: Unified variant type (like Rust enum) *)
type app_error =
  | Io of io_error
  | Parse of parse_error
  | Net of net_error

let do_io () = Error (Io FileNotFound)
let do_parse () = Error (Parse BadFormat)
let do_net () = Error (Net Timeout)

let process_all () =
  match do_io () with
  | Error _ as e -> e
  | Ok data ->
    match do_parse () with
    | Error _ as e -> e
    | Ok parsed ->
      match do_net () with
      | Error _ as e -> e
      | Ok _ -> Ok (data, parsed)

(* Approach 2: Polymorphic variants — open, extensible *)
let do_io_poly () : (string, [> `FileNotFound | `ReadError of string]) result =
  Error `FileNotFound

let do_parse_poly () : (int, [> `BadFormat | `Overflow]) result =
  Ok 42

let process_poly () =
  match do_io_poly () with
  | Error _ as e -> e
  | Ok _data ->
    match do_parse_poly () with
    | Error _ as e -> e
    | Ok v -> Ok v

let test_unified () =
  (match do_io () with
   | Error (Io FileNotFound) -> ()
   | _ -> assert false);
  (match do_parse () with
   | Error (Parse BadFormat) -> ()
   | _ -> assert false);
  (match do_net () with
   | Error (Net Timeout) -> ()
   | _ -> assert false);
  Printf.printf "  Approach 1 (unified variant): passed\n"

let test_poly () =
  (match do_io_poly () with
   | Error `FileNotFound -> ()
   | _ -> assert false);
  assert (do_parse_poly () = Ok 42);
  Printf.printf "  Approach 2 (polymorphic variants): passed\n"

let () =
  Printf.printf "Testing multiple error types:\n";
  test_unified ();
  test_poly ();
  Printf.printf "✓ All tests passed\n"
