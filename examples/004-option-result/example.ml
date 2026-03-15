(* 004: Option and Result
   Safe handling of missing values and errors *)

(* --- Approach 1: Option basics --- *)

let safe_div a b =
  if b = 0 then None else Some (a / b)

let safe_head = function
  | [] -> None
  | x :: _ -> Some x

let find_even xs =
  List.find_opt (fun x -> x mod 2 = 0) xs

(* --- Approach 2: Chaining with Option.map and Option.bind --- *)

let double_head xs =
  Option.map (fun x -> x * 2) (safe_head xs)

let safe_div_then_add a b c =
  Option.map (fun q -> q + c) (safe_div a b)

let chain_lookups v1 v2 =
  (* use first element of v1 as index into v2 *)
  Option.bind (safe_head v1) (fun idx ->
    if idx >= 0 && idx < List.length v2
    then Some (List.nth v2 idx)
    else None)

(* --- Approach 3: Result for richer errors --- *)

type my_error =
  | DivByZero
  | NegativeInput
  | EmptyList

let safe_div_r a b =
  if b = 0 then Error DivByZero else Ok (a / b)

let safe_sqrt x =
  if x < 0.0 then Error NegativeInput else Ok (sqrt x)

let safe_head_r = function
  | [] -> Error EmptyList
  | x :: _ -> Ok x

(* Monadic bind: Result.bind chains fallible operations *)
let compute xs =
  Result.bind (safe_head_r xs) (fun x ->
    safe_div_r (x * 10) 3)

let () =
  Printf.printf "safe_div 10 3 = %s\n"
    (match safe_div 10 3 with Some v -> string_of_int v | None -> "None");
  Printf.printf "safe_div 10 0 = %s\n"
    (match safe_div 10 0 with Some v -> string_of_int v | None -> "None");
  Printf.printf "double_head [5;10] = %s\n"
    (match double_head [5;10] with Some v -> string_of_int v | None -> "None");
  Printf.printf "compute [5;10] = %s\n"
    (match compute [5;10] with Ok v -> string_of_int v | Error _ -> "Error");
  Printf.printf "compute [] = %s\n"
    (match compute [] with Ok v -> string_of_int v | Error EmptyList -> "EmptyList" | Error _ -> "Error")
