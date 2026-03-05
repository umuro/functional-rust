(* Miller-Rabin Primality Test in OCaml *)

(* Modular multiplication using Int64 to avoid overflow *)
let mulmod a b m =
  Int64.(to_int (rem (mul (of_int a) (of_int b)) (of_int m)))

(* Fast modular exponentiation *)
let rec pow_mod base exp m =
  if exp = 0 then 1
  else if exp land 1 = 1 then mulmod base (pow_mod base (exp-1) m) m
  else
    let half = pow_mod base (exp/2) m in
    mulmod half half m

(* Single Miller-Rabin witness test: is 'a' a witness to n's primality? *)
(* Returns true if n is probably prime w.r.t. witness a *)
let miller_witness (n : int) (d : int) (s : int) (a : int) : bool =
  let x = ref (pow_mod a d n) in
  if !x = 1 || !x = n - 1 then true
  else begin
    let composite = ref true in
    for _ = 1 to s - 1 do
      x := mulmod !x !x n;
      if !x = n - 1 then composite := false
    done;
    not !composite
  end

(* Deterministic Miller-Rabin for n < 3.2*10^18 *)
let is_prime (n : int) : bool =
  if n < 2 then false
  else if n = 2 || n = 3 || n = 5 || n = 7 then true
  else if n mod 2 = 0 || n mod 3 = 0 then false
  else begin
    (* Factor n-1 = 2^s * d, d odd *)
    let d = ref (n - 1) and s = ref 0 in
    while !d mod 2 = 0 do d := !d / 2; incr s done;
    (* Witnesses sufficient for n < 3.2*10^18 *)
    let witnesses = [2; 3; 5; 7; 11; 13; 17; 19; 23; 29; 31; 37] in
    List.for_all (fun a ->
      if a >= n then true  (* witness >= n means n is prime by definition *)
      else miller_witness n !d !s a
    ) witnesses
  end

let () =
  let primes   = [2; 3; 5; 17; 97; 997; 1_000_003; 999_999_937] in
  let composites = [4; 9; 15; 100; 1001; 1_000_000] in
  List.iter (fun n ->
    Printf.printf "is_prime(%d) = %b  (expected true)\n" n (is_prime n)
  ) primes;
  List.iter (fun n ->
    Printf.printf "is_prime(%d) = %b  (expected false)\n" n (is_prime n)
  ) composites;
  (* Carmichael number: 561 = 3*11*17, fools Fermat test *)
  Printf.printf "is_prime(561) = %b  (Carmichael, expected false)\n" (is_prime 561)
