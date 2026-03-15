(* 056: Result as Monad *)
(* Chain fallible operations with bind *)

(* Approach 1: Explicit pattern matching *)
let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "Not a number: %s" s)

let safe_div a b =
  if b = 0 then Error "Division by zero"
  else Ok (a / b)

let compute_explicit s1 s2 =
  match parse_int s1 with
  | Error e -> Error e
  | Ok a ->
    match parse_int s2 with
    | Error e -> Error e
    | Ok b -> safe_div a b

(* Approach 2: Using Result.bind *)
let compute_bind s1 s2 =
  parse_int s1
  |> Result.bind (fun a ->
    parse_int s2
    |> Result.bind (fun b ->
      safe_div a b))

(* Approach 3: Pipeline with map and bind *)
let add_one r = Result.map (fun x -> x + 1) r
let double r = Result.map (fun x -> x * 2) r

let pipeline s =
  parse_int s
  |> Result.bind (fun n -> safe_div n 2)
  |> Result.map (fun n -> n + 1)
  |> Result.map (fun n -> n * 2)

(* Tests *)
let () =
  assert (parse_int "42" = Ok 42);
  assert (parse_int "abc" = Error "Not a number: abc");
  assert (compute_explicit "10" "3" = Ok 3);
  assert (compute_explicit "10" "0" = Error "Division by zero");
  assert (compute_explicit "abc" "3" = Error "Not a number: abc");
  assert (compute_bind "10" "3" = Ok 3);
  assert (compute_bind "10" "0" = Error "Division by zero");
  assert (pipeline "10" = Ok 12);
  assert (pipeline "abc" = Error "Not a number: abc");
  Printf.printf "✓ All tests passed\n"
