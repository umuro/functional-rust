(* Option and Result: safe error handling without exceptions *)

(* ── Option: safe lookups ────────────────────────────────── *)

let safe_div a b =
  if b = 0.0 then None else Some (a /. b)

let head = function
  | [] -> None
  | h :: _ -> Some h

let last lst =
  let rec aux = function
    | [] -> None
    | [x] -> Some x
    | _ :: t -> aux t
  in aux lst

(* ── Option combinators ──────────────────────────────────── *)

let find_and_double lst pred =
  List.find_opt pred lst |> Option.map (fun x -> x * 2)

let nth_or_default lst n default =
  match List.nth_opt lst n with
  | Some v -> v
  | None -> default

(* ── Result type (OCaml 4.03+) ───────────────────────────── *)

type math_error = Division_by_zero_err | Negative_sqrt | Overflow_err

let checked_div a b =
  if b = 0 then Error Division_by_zero_err
  else Ok (a / b)

let checked_sqrt x =
  if x < 0.0 then Error Negative_sqrt
  else Ok (sqrt x)

(* ── Monadic chaining with Result.bind ───────────────────── *)

let sqrt_of_division a b =
  safe_div a b
  |> Option.to_result ~none:Division_by_zero_err
  |> Result.bind (fun q -> checked_sqrt q)

(* ── Recursive association list lookup ───────────────────── *)

let rec assoc_opt key = function
  | [] -> None
  | (k, v) :: _ when k = key -> Some v
  | _ :: rest -> assoc_opt key rest

(* ── Tests ────────────────────────────────────────────────── *)
let () =
  assert (safe_div 10.0 2.0 = Some 5.0);
  assert (safe_div 1.0 0.0 = None);
  assert (head [1;2;3] = Some 1);
  assert (head [] = None);
  assert (last [1;2;3] = Some 3);
  assert (find_and_double [1;2;3;4] (fun x -> x > 2) = Some 6);
  assert (nth_or_default [10;20;30] 1 0 = 20);
  assert (nth_or_default [10;20;30] 5 99 = 99);
  assert (checked_div 10 2 = Ok 5);
  assert (checked_div 10 0 = Error Division_by_zero_err);
  assert (assoc_opt 2 [(1,"one");(2,"two")] = Some "two");
  assert (assoc_opt 9 [(1,"one")] = None);
  print_endline "✓ All option/result tests passed"
