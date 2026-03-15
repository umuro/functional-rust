(* 1007: Result Combinators
   Result.bind = and_then (flatMap), Result.map, Result.map_error, Result.value.
   OCaml's stdlib Result module provides these directly. *)

let parse_int s =
  match int_of_string_opt (String.trim s) with
  | Some n -> Ok n
  | None   -> Error (Printf.sprintf "not an int: %s" s)

let double_if_positive n =
  if n > 0 then Ok (n * 2)
  else Error "must be positive"

(* Approach 1: Chaining with Result.bind (= and_then) *)
let process_chain s =
  Result.bind (parse_int s) double_if_positive
  |> Result.map string_of_int

(* Approach 2: map, map_error, or_else, value *)
let process_with_fallback s =
  let r =
    Result.bind (parse_int s) double_if_positive
    |> Result.map string_of_int
    |> Result.map_error (fun e -> Printf.sprintf "FALLBACK: %s" e)
  in
  match r with
  | Ok v    -> v
  | Error v -> v

(* or_else: recover from error with a default *)
let process_or_else s =
  match Result.bind (parse_int s) double_if_positive with
  | Ok n    -> Ok n
  | Error _ -> Ok 0  (* fallback to 0 *)

let () =
  assert (process_chain "5" = Ok "10");
  assert (process_chain "-3" = Error "must be positive");
  assert (Result.is_error (process_chain "abc"));

  (* map *)
  assert (Result.map (fun n -> n * 2) (Ok 5) = Ok 10);

  (* map_error *)
  assert (Result.map_error String.uppercase_ascii (Error "low") = Error "LOW");

  (* or_else *)
  assert (process_or_else "-1" = Ok 0);
  assert (process_or_else "5" = Ok 10);

  (* value (like unwrap_or_else) *)
  assert (Result.value (Error "fail") ~default:99 = 99);
  assert (Result.value (Ok 42) ~default:99 = 42);

  assert (process_with_fallback "5" = "10");
  assert (String.sub (process_with_fallback "abc") 0 8 = "FALLBACK");

  Printf.printf "process_chain 5: %s\n"
    (match process_chain "5" with Ok s -> s | Error e -> e)
