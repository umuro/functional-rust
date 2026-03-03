(* GCD and LCM — Euclidean Algorithm *)

(* Version 1: Basic recursive *)
let rec gcd a b = if b = 0 then a else gcd b (a mod b)
let lcm a b = if a = 0 || b = 0 then 0 else abs (a * b) / gcd a b

(* Version 2: GCD over a list using fold *)
let gcd_list = function
  | [] -> 0
  | h :: t -> List.fold_left gcd h t

let () =
  assert (gcd 48 18 = 6);
  assert (lcm 12 18 = 36);
  assert (gcd_list [48; 36; 60; 12] = 12);
  Printf.printf "gcd(48,18) = %d\n" (gcd 48 18);
  Printf.printf "lcm(12,18) = %d\n" (lcm 12 18)
