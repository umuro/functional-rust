(* 071: GCD and LCM *)

(* Approach 1: Recursive GCD *)
let rec gcd a b =
  if b = 0 then abs a
  else gcd b (a mod b)

let lcm a b =
  if a = 0 || b = 0 then 0
  else abs (a * b) / gcd a b

(* Approach 2: GCD of a list *)
let gcd_list = function
  | [] -> 0
  | x :: xs -> List.fold_left gcd x xs

let lcm_list = function
  | [] -> 1
  | x :: xs -> List.fold_left lcm x xs

(* Approach 3: Extended GCD — returns (g, x, y) where ax + by = g *)
let rec extended_gcd a b =
  if b = 0 then (a, 1, 0)
  else
    let g, x, y = extended_gcd b (a mod b) in
    (g, y, x - (a / b) * y)

(* Tests *)
let () =
  assert (gcd 12 8 = 4);
  assert (gcd 17 5 = 1);
  assert (gcd 0 5 = 5);
  assert (gcd 100 0 = 100);
  assert (lcm 4 6 = 12);
  assert (lcm 3 5 = 15);
  assert (lcm 0 5 = 0);
  assert (gcd_list [12; 18; 24] = 6);
  assert (lcm_list [4; 6; 8] = 24);
  let (g, x, y) = extended_gcd 35 15 in
  assert (g = 5);
  assert (35 * x + 15 * y = 5);
  Printf.printf "✓ All tests passed\n"
