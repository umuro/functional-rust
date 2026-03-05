(* Predicate combinators in OCaml *)
let pred_and p1 p2 x = p1 x && p2 x
let pred_or  p1 p2 x = p1 x || p2 x
let pred_not p x = not (p x)

let () =
  let is_even x = x mod 2 = 0 in
  let is_positive x = x > 0 in
  let is_small x = x < 100 in

  let is_valid = pred_and (pred_and is_even is_positive) is_small in
  let nums = [-2; 0; 4; 8; 102; -6; 50; 99] in
  let valid = List.filter is_valid nums in
  Printf.printf "valid (even, positive, <100): [%s]\n"
    (String.concat "; " (List.map string_of_int valid));

  let either = pred_or (fun x -> x < 0) (fun x -> x > 50) in
  let extreme = List.filter either nums in
  Printf.printf "extreme (<0 or >50): [%s]\n"
    (String.concat "; " (List.map string_of_int extreme))
