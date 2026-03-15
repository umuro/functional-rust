(* 071: GCD and LCM
   Euclidean algorithm — naturally recursive in OCaml *)

(* --- Approach 1: Recursive GCD (Euclidean) --- *)

let rec gcd a b =
  if b = 0 then abs a else gcd b (a mod b)

let lcm a b =
  if a = 0 || b = 0 then 0
  else abs (a * b) / gcd a b

(* --- Approach 2: Iterative GCD using loop-style recursion --- *)

(* OCaml tail recursion IS iteration; no explicit loop needed *)
let gcd_iter a b =
  let rec aux a b =
    if b = 0 then a else aux b (a mod b)
  in
  aux (abs a) (abs b)

(* GCD/LCM of a list using fold *)
let gcd_list = function
  | [] -> 0
  | xs -> List.fold_left gcd (List.hd xs) (List.tl xs)

let lcm_list = function
  | [] -> 1
  | xs -> List.fold_left lcm (List.hd xs) (List.tl xs)

(* --- Approach 3: Extended Euclidean (returns g, x, y s.t. a*x + b*y = g) --- *)

let rec extended_gcd a b =
  if b = 0 then (a, 1, 0)
  else
    let (g, x, y) = extended_gcd b (a mod b) in
    (g, y, x - (a / b) * y)

let () =
  Printf.printf "gcd 12 8 = %d\n" (gcd 12 8);
  Printf.printf "gcd 17 5 = %d\n" (gcd 17 5);
  Printf.printf "lcm 4 6  = %d\n" (lcm 4 6);
  Printf.printf "gcd_iter (-12) 8 = %d\n" (gcd_iter (-12) 8);
  Printf.printf "gcd_list [12;18;24] = %d\n" (gcd_list [12;18;24]);
  Printf.printf "lcm_list [4;6;8] = %d\n" (lcm_list [4;6;8]);
  let (g, x, y) = extended_gcd 35 15 in
  Printf.printf "extended_gcd 35 15: g=%d, 35*%d + 15*%d = %d\n" g x y (35*x + 15*y)
