(* 004: Option and Result *)
(* Safe handling of missing values and errors *)

(* Approach 1: Option basics *)
let safe_div a b =
  if b = 0 then None else Some (a / b)

let safe_head = function
  | [] -> None
  | x :: _ -> Some x

let find_even lst =
  List.find_opt (fun x -> x mod 2 = 0) lst

(* Approach 2: Chaining with map and bind *)
let double_head lst =
  safe_head lst |> Option.map (fun x -> x * 2)

let safe_div_then_add a b c =
  safe_div a b |> Option.map (fun q -> q + c)

let chain_lookups lst1 lst2 =
  safe_head lst1
  |> Option.bind (fun idx ->
    if idx >= 0 && idx < List.length lst2
    then Some (List.nth lst2 idx)
    else None)

(* Approach 3: Result for richer errors *)
type error = DivByZero | NegativeInput | EmptyList

let safe_div_r a b =
  if b = 0 then Error DivByZero else Ok (a / b)

let safe_sqrt x =
  if x < 0.0 then Error NegativeInput
  else Ok (sqrt x)

let safe_head_r = function
  | [] -> Error EmptyList
  | x :: _ -> Ok x

let compute lst =
  match safe_head_r lst with
  | Error e -> Error e
  | Ok x -> safe_div_r (x * 10) 3

(* Tests *)
let () =
  assert (safe_div 10 3 = Some 3);
  assert (safe_div 10 0 = None);
  assert (safe_head [1; 2; 3] = Some 1);
  assert (safe_head [] = None);
  assert (double_head [5; 10] = Some 10);
  assert (double_head [] = None);
  assert (safe_div_then_add 10 3 5 = Some 8);
  assert (safe_div_then_add 10 0 5 = None);
  assert (chain_lookups [1] [10; 20; 30] = Some 20);
  assert (chain_lookups [] [10; 20] = None);
  assert (safe_div_r 10 2 = Ok 5);
  assert (safe_div_r 10 0 = Error DivByZero);
  assert (compute [5; 10] = Ok 16);
  assert (compute [] = Error EmptyList);
  Printf.printf "✓ All tests passed\n"
