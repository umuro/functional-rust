(* 940: GCD and LCM — Euclidean Algorithm

   Classic recursive algorithm, beautifully concise in OCaml.
   OCaml's TCO means the recursive gcd is stack-safe. *)

(* ── GCD: Euclid's algorithm ─────────────────────────────────────────────── *)

(* Recursive — tail position, so OCaml optimises to a loop *)
let rec gcd a b =
  if b = 0 then a else gcd b (a mod b)

(* LCM: |a × b| / gcd(a, b) — divide first to avoid overflow *)
let lcm a b =
  if a = 0 || b = 0 then 0
  else a / gcd a b * b

(* GCD of a list via fold *)
let gcd_list nums =
  match nums with
  | [] -> 0
  | hd :: tl -> List.fold_left gcd hd tl

(* LCM of a list *)
let lcm_list nums =
  match nums with
  | [] -> 0
  | hd :: tl -> List.fold_left lcm hd tl

(* ── Extended Euclidean: returns (g, s, t) where a*s + b*t = g ──────────── *)

let rec extended_gcd a b =
  if b = 0 then (a, 1, 0)
  else
    let (g, s, t) = extended_gcd b (a mod b) in
    (g, t, s - (a / b) * t)

(* ── Iterative GCD (avoids any potential stack concerns for very large n) ─── *)

let gcd_iterative a b =
  let a = ref a and b = ref b in
  while !b <> 0 do
    let temp = !b in
    b := !a mod !b;
    a := temp
  done;
  !a

(* ── Co-primality ────────────────────────────────────────────────────────── *)

let are_coprime a b = gcd a b = 1

(* Euler's totient φ(n) — count integers 1..n coprime to n *)
let totient n =
  List.length (List.filter (are_coprime n) (List.init (n - 1) (fun i -> i + 1)))

let () =
  assert (gcd 48 18 = 6);
  assert (gcd 100 75 = 25);
  assert (gcd 17 13 = 1);     (* coprime *)

  assert (gcd 5 0 = 5);
  assert (gcd 0 5 = 5);
  assert (gcd 0 0 = 0);

  assert (lcm 12 18 = 36);
  assert (lcm 4 6 = 12);
  assert (lcm 0 5 = 0);

  assert (gcd_list [48; 36; 60; 12] = 12);
  assert (gcd_list [] = 0);
  assert (lcm_list [2; 3; 4; 5] = 60);

  (* iterative matches recursive *)
  assert (gcd_iterative 48 18 = gcd 48 18);
  assert (gcd_iterative 100 75 = gcd 100 75);

  (* extended GCD: 48*s + 18*t = 6 *)
  let (g, s, t) = extended_gcd 48 18 in
  assert (g = 6);
  assert (48 * s + 18 * t = g);

  (* coprimality *)
  assert (are_coprime 17 13);
  assert (not (are_coprime 12 18));

  (* Euler's totient φ(12) = 4 — {1, 5, 7, 11} are coprime to 12 *)
  assert (totient 12 = 4);
  (* φ(prime p) = p - 1 *)
  assert (totient 7 = 6);

  print_endline "940-gcd-lcm: all tests passed"
