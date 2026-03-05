(* Chinese Remainder Theorem in OCaml *)

(* Extended GCD: returns (g, x, y) where a*x + b*y = g *)
let rec extended_gcd (a : int) (b : int) : int * int * int =
  if b = 0 then (a, 1, 0)
  else
    let (g, x, y) = extended_gcd b (a mod b) in
    (g, y, x - (a / b) * y)

(* Combine two congruences: x ≡ a1 (mod m1), x ≡ a2 (mod m2) *)
(* Returns Some (remainder, modulus) or None if no solution *)
let crt_combine (a1 : int) (m1 : int) (a2 : int) (m2 : int)
    : (int * int) option =
  let (g, p, _) = extended_gcd m1 m2 in
  if (a2 - a1) mod g <> 0 then None
  else begin
    let lcm = m1 / g * m2 in
    (* x = a1 + m1 * ((a2 - a1) / g * p mod (m2/g)) *)
    let m2g = m2 / g in
    let diff = ((a2 - a1) / g) mod m2g in
    let x = (a1 + m1 * ((diff * p mod m2g + m2g) mod m2g)) mod lcm in
    let x = (x + lcm) mod lcm in
    Some (x, lcm)
  end

(* Solve a system of congruences: x ≡ a_i (mod m_i) for all i *)
let crt (congruences : (int * int) list) : (int * int) option =
  List.fold_left (fun acc (a, m) ->
    match acc with
    | None -> None
    | Some (r, lcm) -> crt_combine r lcm a m
  ) (Some (0, 1)) congruences

let () =
  (* x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7) → x = 23 *)
  let system = [(2, 3); (3, 5); (2, 7)] in
  (match crt system with
   | Some (x, m) -> Printf.printf "x ≡ 2(mod 3), x ≡ 3(mod 5), x ≡ 2(mod 7): x = %d (mod %d)\n" x m
   | None -> Printf.printf "No solution\n");

  (* x ≡ 0 (mod 4), x ≡ 6 (mod 10) — non-coprime moduli, has solution *)
  (match crt [(0, 4); (6, 10)] with
   | Some (x, m) -> Printf.printf "x ≡ 0(mod 4), x ≡ 6(mod 10): x = %d (mod %d)\n" x m
   | None -> Printf.printf "No solution\n");

  (* x ≡ 1 (mod 4), x ≡ 6 (mod 10) — no solution since 6-1=5 not div by gcd(4,10)=2 *)
  (match crt [(1, 4); (6, 10)] with
   | Some (x, m) -> Printf.printf "x ≡ 1(mod 4), x ≡ 6(mod 10): x = %d (mod %d)\n" x m
   | None -> Printf.printf "x ≡ 1(mod 4), x ≡ 6(mod 10): No solution\n")
