(* Singleton types: each type has exactly one value.
   Used for type-level tags and phantom types. *)

(* Unit type — the canonical singleton *)
let () =
  let u : unit = () in
  Printf.printf "unit value: %s\n" (if u = () then "()" else "impossible");

  (* Phantom type tags as singletons *)
  ()

(* GADT encoding of type-level naturals as singletons *)
type zero  = ZERO
type 'n succ = SUCC

type _ nat =
  | Zero : zero nat
  | Succ : 'n nat -> 'n succ nat

let zero     = Zero
let one      = Succ Zero
let two      = Succ (Succ Zero)

let rec to_int : type n. n nat -> int = function
  | Zero   -> 0
  | Succ n -> 1 + to_int n

let () =
  Printf.printf "zero = %d\n" (to_int zero);
  Printf.printf "one  = %d\n" (to_int one);
  Printf.printf "two  = %d\n" (to_int two)

(* Singleton bool: type-level true/false *)
type tru = TRU
type fls = FLS

type _ bool_s =
  | True  : tru bool_s
  | False : fls bool_s

let () =
  let _t : tru bool_s = True  in
  let _f : fls bool_s = False in
  Printf.printf "Singleton types compile and run\n"
