(* Fixed-point types in OCaml *)

(* Fix-point of a functor *)
type 'f fix = Fix of ('f fix) 'f [@@unboxed]

(* Base functor for Nat *)
type 'a nat_f = Zero | Succ of 'a

(* Fix Nat *)
type nat = nat_f fix

let zero     = Fix Zero
let succ n   = Fix (Succ n)

let to_int (Fix n) =
  let rec go acc = function
    | Zero   -> acc
    | Succ (Fix n) -> go (acc+1) n
  in go 0 n

let () =
  let three = succ (succ (succ zero)) in
  Printf.printf "three = %d\n" (to_int three)
