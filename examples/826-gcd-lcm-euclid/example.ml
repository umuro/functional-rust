(* GCD, LCM, and Euclidean Algorithm in OCaml *)

(* The classic one-liner — functional elegance at its finest *)
let rec gcd (a : int) (b : int) : int =
  if b = 0 then a else gcd b (a mod b)

(* LCM via GCD — divide first to avoid overflow *)
let lcm (a : int) (b : int) : int =
  if a = 0 || b = 0 then 0
  else (a / gcd a b) * b

(* GCD of a list — neutral element is 0 since gcd(a,0)=a *)
let list_gcd (xs : int list) : int =
  List.fold_left gcd 0 xs

let list_lcm (xs : int list) : int =
  List.fold_left lcm 1 xs

(* Trace the steps of Euclidean algorithm *)
let gcd_trace (a : int) (b : int) : unit =
  Printf.printf "gcd(%d, %d):\n" a b;
  let a = ref a and b = ref b in
  while !b <> 0 do
    Printf.printf "  gcd(%d, %d) → rem = %d\n" !a !b (!a mod !b);
    let r = !a mod !b in
    a := !b;
    b := r
  done;
  Printf.printf "  = %d\n" !a

(* Binary GCD (Stein's algorithm) — avoids modulo *)
let rec binary_gcd (a : int) (b : int) : int =
  match (a, b) with
  | (0, b) -> b
  | (a, 0) -> a
  | (a, b) when a mod 2 = 0 && b mod 2 = 0 -> 2 * binary_gcd (a / 2) (b / 2)
  | (a, b) when a mod 2 = 0 -> binary_gcd (a / 2) b
  | (a, b) when b mod 2 = 0 -> binary_gcd a (b / 2)
  | (a, b) -> binary_gcd (abs (a - b)) (min a b)

let () =
  gcd_trace 48 18;
  Printf.printf "\ngcd(48, 18) = %d  (expected 6)\n" (gcd 48 18);
  Printf.printf "lcm(4, 6)   = %d  (expected 12)\n" (lcm 4 6);
  Printf.printf "lcm(12, 18) = %d  (expected 36)\n" (lcm 12 18);
  Printf.printf "gcd [12;18;24] = %d  (expected 6)\n" (list_gcd [12;18;24]);
  Printf.printf "lcm [2;3;4]    = %d  (expected 12)\n" (list_lcm [2;3;4]);
  Printf.printf "binary_gcd(48, 18) = %d\n" (binary_gcd 48 18)
