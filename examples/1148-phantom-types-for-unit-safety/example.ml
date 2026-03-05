(* Phantom Types for Unit Safety *)
(* Use phantom types to prevent unit confusion *)

type meters
type seconds
type _ quantity = Q of float

let meters (x : float) : meters quantity = Q x
let seconds (x : float) : seconds quantity = Q x

let add (Q a : 'a quantity) (Q b : 'a quantity) : 'a quantity = Q (a +. b)
let scale (Q a : 'a quantity) (f : float) : 'a quantity = Q (a *. f)
let value (Q x : _ quantity) = x

let d1 = meters 100.0
let d2 = meters 50.0
let total = add d1 d2  (* OK: same units *)
let () = Printf.printf "Total: %.1f meters\n" (value total)

(* let bad = add d1 (seconds 5.0)  (* Type error! *) *)
