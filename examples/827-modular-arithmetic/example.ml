(* Modular Arithmetic in OCaml *)

(* Modular addition *)
let mod_add a b m = (a + b) mod m

(* Modular subtraction — ensure non-negative *)
let mod_sub a b m = ((a - b) mod m + m) mod m

(* Modular multiplication — use Int64 to avoid overflow for large values *)
let mod_mul a b m =
  Int64.(to_int (rem (mul (of_int a) (of_int b)) (of_int m)))

(* Fast modular exponentiation: a^exp mod m *)
let rec pow_mod (a : int) (exp : int) (m : int) : int =
  if exp = 0 then 1
  else if exp mod 2 = 0 then
    let half = pow_mod a (exp / 2) m in
    mod_mul half half m
  else
    mod_mul a (pow_mod a (exp - 1) m) m

(* Modular inverse via Fermat's little theorem (m must be prime) *)
let mod_inv_fermat (a : int) (p : int) : int =
  pow_mod a (p - 2) p

(* Extended Euclidean Algorithm: returns (gcd, x, y) s.t. ax + by = gcd *)
let rec extended_gcd (a : int) (b : int) : int * int * int =
  if b = 0 then (a, 1, 0)
  else
    let (g, x, y) = extended_gcd b (a mod b) in
    (g, y, x - (a / b) * y)

(* Modular inverse via Extended Euclidean (works for any coprime a, m) *)
let mod_inv (a : int) (m : int) : int option =
  let (g, x, _) = extended_gcd (((a mod m) + m) mod m) m in
  if g <> 1 then None
  else Some (((x mod m) + m) mod m)

let () =
  let m = 1_000_000_007 in
  Printf.printf "mod_add(999_999_999, 9, %d) = %d\n" m (mod_add 999_999_999 9 m);
  Printf.printf "mod_sub(3, 7, 10) = %d  (expected 6)\n" (mod_sub 3 7 10);
  Printf.printf "mod_mul(123456, 789012, %d) = %d\n" m (mod_mul 123456 789012 m);
  Printf.printf "pow_mod(2, 10, %d) = %d  (expected 1024)\n" m (pow_mod 2 10 m);
  Printf.printf "mod_inv_fermat(3, 7) = %d  (expected 5, since 3*5=15≡1 mod 7)\n"
    (mod_inv_fermat 3 7);
  (match mod_inv 3 7 with
   | Some inv -> Printf.printf "mod_inv(3, 7) = %d\n" inv
   | None -> Printf.printf "mod_inv(3, 7): no inverse\n");
  (* Verify: 3 * inv ≡ 1 mod 7 *)
  let inv = Option.get (mod_inv 3 7) in
  Printf.printf "3 * %d mod 7 = %d  (should be 1)\n" inv (mod_mul 3 inv 7)
