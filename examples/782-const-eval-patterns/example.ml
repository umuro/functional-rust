(* const eval limitations — OCaml perspective
   In OCaml, all module-level let-bindings run at load time.
   We show what's easy vs what requires care. *)

(* ✓ Arithmetic — trivial in OCaml *)
let sum_1_to_100 = (100 * 101) / 2  (* = 5050 *)

(* ✓ Recursive (TCO) *)
let rec gcd a b = if b = 0 then a else gcd b (a mod b)
let gcd_48_18 = gcd 48 18   (* = 6 *)

(* ✓ String — but note this allocates at load time *)
let greeting = "Hello, " ^ "world"   (* concat at load *)

(* ✓ List building — allocates *)
let evens = List.init 10 (fun i -> i * 2)

(* ✗ True compile-time (in binary) impossible without external tool or ppx *)
(* OCaml's advantage: no limitation on heap allocation in "const" context *)
(* OCaml's disadvantage: can't guarantee zero-cost / binary embedding *)

(* Workaround: explicit table generation *)
let pow_table =
  Array.init 32 (fun i ->
    let rec go acc n = if n = 0 then acc else go (acc * 2) (n-1)
    in go 1 i)

let () =
  Printf.printf "sum(1..100) = %d\n" sum_1_to_100;
  Printf.printf "gcd(48,18)  = %d\n" gcd_48_18;
  Printf.printf "2^10        = %d\n" pow_table.(10);
  Printf.printf "greeting    = %s\n" greeting
