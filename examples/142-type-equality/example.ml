(* Type equality witnesses: a value of type ('a, 'b) eq proves a = b.
   Useful in GADTs to unify type variables. *)

(* Leibniz equality: a = b iff for all F, F a = F b *)
type ('a, 'b) eq = { cast : 'r. 'r 'a -> 'r 'b }

(* Or simpler with GADTs: *)
type (_, _) teq = Refl : ('a, 'a) teq

(* Using Refl to cast values *)
let cast_with : type a b. (a, b) teq -> a -> b = fun Refl x -> x

(* Symmetry: if a = b then b = a *)
let sym : type a b. (a, b) teq -> (b, a) teq = fun Refl -> Refl

(* Transitivity: if a = b and b = c then a = c *)
let trans : type a b c. (a, b) teq -> (b, c) teq -> (a, c) teq =
  fun Refl Refl -> Refl

let () =
  let eq_int : (int, int) teq = Refl in
  let n : int = 42 in
  let m : int = cast_with eq_int n in
  assert (n = m);
  Printf.printf "cast_with Refl: %d\n" m;

  (* Symmetry *)
  let _sym_eq : (int, int) teq = sym eq_int in
  Printf.printf "Type equality proofs work\n"
