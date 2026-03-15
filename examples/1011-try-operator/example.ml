(* 1011: The ? (Try) Operator
   Rust's ? desugars to early-return on Err + From conversion.
   In OCaml we achieve the same with Result.bind (>>=) chains or
   the let* syntax (available with Result monad or a let-bind helper). *)

type app_error =
  | NotFound
  | ParseFailed of string
  | TooLarge of int

let string_of_app_error = function
  | NotFound        -> "not found"
  | ParseFailed s   -> Printf.sprintf "parse failed: %s" s
  | TooLarge n      -> Printf.sprintf "too large: %d" n

let read_data key =
  if key = "missing" then Error NotFound
  else Ok "42"

let parse_data s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None   -> Error (ParseFailed (Printf.sprintf "not an int: %s" s))

let validate n =
  if n > 100 then Error (TooLarge n)
  else Ok n

(* Approach 1: explicit Result.bind chain (mirrors ? in Rust) *)
let process_bind key =
  Result.bind (read_data key) (fun s ->
    Result.bind (parse_data s) (fun n ->
      validate n))

(* Approach 2: using let* via a local bind operator *)
let ( let* ) = Result.bind

let process_let_star key =
  let* s = read_data key in
  let* n = parse_data s in
  validate n

(* Approach 3: inline composition *)
let process_inline key =
  read_data key
  |> Result.bind parse_data
  |> Result.bind validate

let () =
  assert (process_bind "ok" = Ok 42);
  assert (process_bind "missing" = Error NotFound);

  (* All three are equivalent *)
  assert (process_bind "ok" = process_let_star "ok");
  assert (process_bind "missing" = process_let_star "missing");
  assert (process_bind "ok" = process_inline "ok");
  assert (process_bind "missing" = process_inline "missing");

  (* ParseFailed conversion *)
  (match parse_data "abc" with
   | Error (ParseFailed _) -> ()
   | _ -> assert false);

  (* TooLarge *)
  let r =
    let* n =
      match int_of_string_opt "200" with
      | Some n -> Ok n
      | None -> Error (ParseFailed "not an int")
    in
    validate n
  in
  assert (r = Error (TooLarge 200));

  Printf.printf "process ok: %s\n"
    (match process_let_star "ok" with
     | Ok n -> string_of_int n
     | Error e -> string_of_app_error e)
