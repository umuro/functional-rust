(* Discrete Logarithm: Baby-Step Giant-Step in OCaml *)

let mulmod a b m =
  Int64.(to_int (rem (mul (of_int a) (of_int b)) (of_int m)))

let rec pow_mod base exp m =
  if exp = 0 then 1 mod m
  else if exp land 1 = 1 then mulmod base (pow_mod base (exp-1) m) m
  else let h = pow_mod base (exp/2) m in mulmod h h m

(* Baby-step giant-step: find x s.t. a^x ≡ b (mod p), 0 ≤ x < p *)
(* Returns Some x or None if no solution *)
let bsgs (a : int) (b : int) (p : int) : int option =
  let m = int_of_float (ceil (sqrt (float_of_int p))) in

  (* Baby steps: table[a^j mod p] = j, for j = 0..m-1 *)
  let table = Hashtbl.create m in
  let aj = ref 1 in
  for j = 0 to m - 1 do
    Hashtbl.replace table !aj j;
    aj := mulmod !aj a p
  done;

  (* Giant steps: check b * (a^m)^i mod p, for i = 1..m *)
  let am = pow_mod a m p in
  let giant = ref b in
  let result = ref None in
  let i = ref 1 in
  while !i <= m && !result = None do
    giant := mulmod !giant am p;
    (match Hashtbl.find_opt table !giant with
     | Some j ->
       let x = !i * m - j in
       if x >= 0 then result := Some x
     | None -> ());
    incr i
  done;
  !result

(* Brute-force for verification *)
let discrete_log_brute (a : int) (b : int) (p : int) : int option =
  let x = ref 0 and cur = ref 1 in
  let found = ref None in
  while !x < p && !found = None do
    if !cur = b then found := Some !x;
    cur := mulmod !cur a p;
    incr x
  done;
  !found

let () =
  let cases = [
    (2, 22, 29);    (* 2^x ≡ 22 (mod 29) *)
    (3, 1, 7);      (* 3^x ≡ 1 (mod 7), x=6 *)
    (5, 3, 23);     (* 5^x ≡ 3 (mod 23) *)
    (2, 3, 7);      (* 2^x ≡ 3 (mod 7) *)
  ] in
  List.iter (fun (a, b, p) ->
    let bsgs_result = bsgs a b p in
    let brute_result = discrete_log_brute a b p in
    Printf.printf "%d^x ≡ %d (mod %d): bsgs=%s, brute=%s\n"
      a b p
      (match bsgs_result with Some x -> string_of_int x | None -> "None")
      (match brute_result with Some x -> string_of_int x | None -> "None")
  ) cases
