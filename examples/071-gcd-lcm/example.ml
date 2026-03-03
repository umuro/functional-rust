(* GCD and LCM — Euclidean Algorithm *)

let rec gcd a b = if b = 0 then a else gcd b (a mod b)
let lcm a b = if a = 0 || b = 0 then 0 else abs (a * b) / gcd a b

let gcd_list = function
  | [] -> 0
  | h :: t -> List.fold_left gcd h t

let () =
  Printf.printf "gcd(48,18) = %d\n" (gcd 48 18);
  Printf.printf "lcm(12,18) = %d\n" (lcm 12 18);
  Printf.printf "gcd_list = %d\n" (gcd_list [48; 36; 60; 12]);
  assert (gcd 48 18 = 6);
  assert (lcm 12 18 = 36);
  assert (gcd_list [48; 36; 60; 12] = 12);
  Printf.printf "All GCD/LCM tests passed!\n"
