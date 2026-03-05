(* Fast Modular Exponentiation in OCaml *)

(* Recursive binary method — elegant and clear *)
let rec pow_mod_rec (base : int) (exp : int) (m : int) : int =
  if exp = 0 then 1 mod m
  else if exp land 1 = 1 then
    (* Odd exponent: a^n = a * a^(n-1) *)
    let rest = pow_mod_rec base (exp - 1) m in
    Int64.(to_int (rem (mul (of_int base) (of_int rest)) (of_int m)))
  else
    (* Even exponent: a^n = (a^(n/2))^2 *)
    let half = pow_mod_rec base (exp / 2) m in
    Int64.(to_int (rem (mul (of_int half) (of_int half)) (of_int m)))

(* Iterative binary method — more efficient (no stack) *)
let pow_mod (base : int) (exp : int) (m : int) : int =
  let result = ref 1 in
  let base = ref (base mod m) in
  let exp = ref exp in
  while !exp > 0 do
    if !exp land 1 = 1 then
      result := Int64.(to_int (rem (mul (of_int !result) (of_int !base)) (of_int m)));
    base := Int64.(to_int (rem (mul (of_int !base) (of_int !base)) (of_int m)));
    exp := !exp lsr 1
  done;
  !result

(* Matrix exponentiation for Fibonacci — shows generality of the method *)
(* M^n via binary exponentiation where M is a 2x2 matrix *)
type matrix = { a: int; b: int; c: int; d: int }

let mat_mul m1 m2 modp =
  let mm a b = Int64.(to_int (rem (mul (of_int a) (of_int b)) (of_int modp))) in
  { a = (mm m1.a m2.a + mm m1.b m2.c) mod modp;
    b = (mm m1.a m2.b + mm m1.b m2.d) mod modp;
    c = (mm m1.c m2.a + mm m1.d m2.c) mod modp;
    d = (mm m1.c m2.b + mm m1.d m2.d) mod modp }

let mat_id = { a=1; b=0; c=0; d=1 }

let rec mat_pow m exp modp =
  if exp = 0 then mat_id
  else if exp land 1 = 1 then mat_mul m (mat_pow m (exp-1) modp) modp
  else let half = mat_pow m (exp/2) modp in mat_mul half half modp

(* F(n) mod p using matrix exponentiation in O(log n) *)
let fib_mod n p =
  if n = 0 then 0
  else
    let m = mat_pow { a=1; b=1; c=1; d=0 } (n-1) p in
    m.a

let () =
  Printf.printf "2^10 mod 1000 = %d  (expected 24)\n" (pow_mod 2 10 1000);
  Printf.printf "3^100 mod 1_000_000_007 = %d\n" (pow_mod 3 100 1_000_000_007);
  Printf.printf "2^1000 mod 1009 = %d\n" (pow_mod 2 1000 1009);
  Printf.printf "recursive: 2^10 mod 1000 = %d\n" (pow_mod_rec 2 10 1000);
  Printf.printf "fib(10) mod 100 = %d  (expected 55)\n" (fib_mod 10 100);
  Printf.printf "fib(100) mod 1_000_000_007 = %d\n" (fib_mod 100 1_000_000_007)
