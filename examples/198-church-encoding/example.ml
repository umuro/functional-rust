(* Church encoding: represent natural numbers as iteration functions.
   A Church numeral n is a function that applies f to x exactly n times. *)

(* Church numerals: type is ('a -> 'a) -> 'a -> 'a *)
let zero  f x = x
let one   f x = f x
let two   f x = f (f x)
let three f x = f (f (f x))

(* Arithmetic *)
let succ n f x = f (n f x)
let add  m n f x = m f (n f x)
let mul  m n f x = m (n f) x
let pow  m n     = n m            (* m^n *)

(* Convert to int for display *)
let to_int n = n (fun x -> x + 1) 0

(* Church booleans *)
let ctrue  t _f = t
let cfalse _t f = f
let cif b t f = b t f
let cand b1 b2 = b1 b2 cfalse
let cor  b1 b2 = b1 ctrue b2
let cnot b     = b cfalse ctrue

let to_bool b = b true false

let () =
  let four = succ three in
  let five = add two three in
  let six  = mul two three in
  let nine = pow three two in

  Printf.printf "3 + 2 = %d\n" (to_int (add three two));
  Printf.printf "4     = %d\n" (to_int four);
  Printf.printf "5     = %d\n" (to_int five);
  Printf.printf "6     = %d\n" (to_int six);
  Printf.printf "3^2   = %d\n" (to_int nine);

  Printf.printf "true and false = %b\n" (to_bool (cand ctrue cfalse));
  Printf.printf "true or  false = %b\n" (to_bool (cor  ctrue cfalse));
  Printf.printf "not true       = %b\n" (to_bool (cnot ctrue))
