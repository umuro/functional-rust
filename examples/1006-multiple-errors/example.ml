(* 1006: Multiple Error Types
   Two approaches to unifying multiple error types:
   1. Polymorphic variant / string (like Box<dyn Error>)
   2. Typed enum — exhaustive matching, stricter

   OCaml's polymorphic variants let us unify without boilerplate. *)

(* Individual error types *)
type io_error  = IoError of string
type net_error = NetError of string

let string_of_io_error  (IoError s)  = Printf.sprintf "IO error: %s" s
let string_of_net_error (NetError s) = Printf.sprintf "network error: %s" s

(* Approach 1: string-based (like Box<dyn Error> erasure) *)
let do_io_boxed () : (string, string) result =
  Error (string_of_io_error (IoError "file not found"))

let do_parse_boxed s : (int, string) result =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None   -> Error (Printf.sprintf "parse error: %s" s)

let do_net_boxed () : (string, string) result =
  Error (string_of_net_error (NetError "timeout"))

let process_boxed () =
  let data =
    match do_io_boxed () with
    | Ok d -> Ok d
    | Error _ -> Ok "42"  (* fallback *)
  in
  Result.bind data (fun d ->
    Result.bind (do_parse_boxed d) (fun parsed ->
      let _ = match do_net_boxed () with
        | Ok r -> r
        | Error _ -> "ok"
      in
      Ok parsed))

(* Approach 2: Typed enum — exhaustive, composable *)
type app_error =
  | Io of io_error
  | Parse of string
  | Net of net_error

let string_of_app_error = function
  | Io e    -> string_of_io_error e
  | Parse s -> Printf.sprintf "parse: %s" s
  | Net e   -> string_of_net_error e

let do_io_typed () : (string, io_error) result = Ok "42"

let do_parse_typed s : (int, string) result =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None   -> Error (Printf.sprintf "not an int: %s" s)

let process_typed () =
  Result.bind
    (Result.map_error (fun e -> Io e) (do_io_typed ()))
    (fun data ->
       Result.map_error (fun e -> Parse e) (do_parse_typed data))

let () =
  assert (process_boxed () = Ok 42);

  (match do_io_boxed () with
   | Error s -> assert (String.length s > 0)
   | _ -> assert false);

  assert (process_typed () = Ok 42);

  (match (Io (IoError "test")) with
   | Io _ -> ()
   | _ -> assert false);

  let err = Net (NetError "timeout") in
  assert (string_of_app_error err = "network error: timeout");

  Printf.printf "process_boxed: %s\n"
    (match process_boxed () with Ok n -> string_of_int n | Error e -> e)
