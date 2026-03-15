(* 056: Result as Monad
   Chain fallible operations with Result.bind (>>=) *)

type calc_error =
  | ParseError of string
  | DivByZero

let parse_int s =
  match int_of_string_opt s with
  | None   -> Error (ParseError (Printf.sprintf "not an integer: %s" s))
  | Some n -> Ok n

let safe_div a b =
  if b = 0 then Error DivByZero else Ok (a / b)

(* --- Approach 1: Using Result.bind (monadic bind / >>=) --- *)

let ( >>= ) = Result.bind

let compute_bind s1 s2 =
  parse_int s1 >>= fun a ->
  parse_int s2 >>= fun b ->
  safe_div a b

(* --- Approach 2: Let-syntax style (sequential, explicit) --- *)

let compute_let s1 s2 =
  match parse_int s1 with
  | Error e -> Error e
  | Ok a ->
    match parse_int s2 with
    | Error e -> Error e
    | Ok b -> safe_div a b

(* --- Approach 3: Chained pipeline using map and bind --- *)

let pipeline s =
  parse_int s
  >>= (fun n -> safe_div n 2)
  |> Result.map (fun n -> n + 1)
  |> Result.map (fun n -> n * 2)

let () =
  let show_result = function
    | Ok v    -> string_of_int v
    | Error DivByZero  -> "DivByZero"
    | Error (ParseError msg) -> "ParseError: " ^ msg
  in
  Printf.printf "compute_bind \"10\" \"3\" = %s\n" (show_result (compute_bind "10" "3"));
  Printf.printf "compute_bind \"10\" \"0\" = %s\n" (show_result (compute_bind "10" "0"));
  Printf.printf "compute_bind \"abc\" \"3\" = %s\n" (show_result (compute_bind "abc" "3"));
  Printf.printf "pipeline \"10\" = %s\n" (show_result (pipeline "10"));
  Printf.printf "pipeline \"abc\" = %s\n" (show_result (pipeline "abc"))
