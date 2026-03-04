type relation = Equal | Sublist | Superlist | Unequal

(* Idiomatic OCaml: use List.filteri/exists with starts_with *)
let rec starts_with lst prefix = match lst, prefix with
  | _, [] -> true
  | [], _ -> false
  | h1 :: t1, h2 :: t2 -> h1 = h2 && starts_with t1 t2

let rec is_sublist sub lst = match lst with
  | [] -> sub = []
  | _ :: t -> starts_with lst sub || is_sublist sub t

let classify a b =
  if a = b then Equal
  else if is_sublist a b then Sublist
  else if is_sublist b a then Superlist
  else Unequal

let name = function
  | Equal -> "equal" | Sublist -> "sublist"
  | Superlist -> "superlist" | Unequal -> "unequal"

let () =
  assert (classify [1;2;3] [0;1;2;3;4] = Sublist);
  assert (classify [0;1;2;3;4] [1;2;3] = Superlist);
  assert (classify [1;2;3] [1;2;3] = Equal);
  assert (classify [1;2;3] [4;5;6] = Unequal);
  assert (classify [] [1;2;3] = Sublist);
  assert (classify [] [] = Equal);
  print_endline "ok"
