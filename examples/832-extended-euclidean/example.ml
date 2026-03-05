(* Extended Euclidean Algorithm in OCaml *)

(* Returns (g, x, y) such that a*x + b*y = g = gcd(a, b) *)
(* The recursive version is the cleanest expression of the algorithm *)
let rec extended_gcd (a : int) (b : int) : int * int * int =
  if b = 0 then
    (a, 1, 0)   (* a*1 + 0*0 = a = gcd *)
  else
    let (g, x, y) = extended_gcd b (a mod b) in
    (* Recurrence: if b*x + (a mod b)*y = g
       then b*x + (a - (a/b)*b)*y = g
            a*y + b*(x - (a/b)*y) = g *)
    (g, y, x - (a / b) * y)

(* Iterative version (avoids deep recursion for very large inputs) *)
let extended_gcd_iter (a : int) (b : int) : int * int * int =
  let old_r = ref a and r = ref b in
  let old_s = ref 1 and s = ref 0 in
  let old_t = ref 0 and t = ref 1 in
  while !r <> 0 do
    let q = !old_r / !r in
    let tmp = !r in r := !old_r - q * !r; old_r := tmp;
    let tmp = !s in s := !old_s - q * !s; old_s := tmp;
    let tmp = !t in t := !old_t - q * !t; old_t := tmp;
  done;
  (!old_r, !old_s, !old_t)

(* Modular inverse of a mod m (exists iff gcd(a,m)=1) *)
let mod_inv (a : int) (m : int) : int option =
  let (g, x, _) = extended_gcd (((a mod m) + m) mod m) m in
  if g <> 1 then None
  else Some (((x mod m) + m) mod m)

(* Solve linear Diophantine equation ax + by = c *)
(* Returns Some (x0, y0, dx, dy) where general solution is (x0 + k*dx, y0 - k*dy) *)
let solve_diophantine (a : int) (b : int) (c : int) : (int * int * int * int) option =
  let (g, x0, y0) = extended_gcd a b in
  if c mod g <> 0 then None
  else
    let scale = c / g in
    Some (x0 * scale, y0 * scale, b / g, a / g)

let () =
  let show_gcd a b =
    let (g, x, y) = extended_gcd a b in
    Printf.printf "gcd(%d, %d) = %d: %d*%d + %d*%d = %d  (check: %b)\n"
      a b g a x b y g (a*x + b*y = g)
  in
  show_gcd 35 15;
  show_gcd 48 18;
  show_gcd 101 103;

  Printf.printf "\nModular inverses:\n";
  List.iter (fun (a, m) ->
    match mod_inv a m with
    | Some inv -> Printf.printf "  inv(%d, %d) = %d  (check: %d*%d mod %d = %d)\n"
                    a m inv a inv m (a * inv mod m)
    | None     -> Printf.printf "  inv(%d, %d): no inverse\n" a m
  ) [(3, 7); (10, 17); (2, 4)];

  Printf.printf "\nDiophantine 3x + 5y = 1:\n";
  (match solve_diophantine 3 5 1 with
   | Some (x, y, dx, dy) ->
     Printf.printf "  Particular: x=%d, y=%d  general: x=%d+%d*k, y=%d-%d*k\n" x y x dx y dy
   | None -> Printf.printf "  No solution\n")
