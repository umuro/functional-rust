(* 934: Church Numerals — Functions as Numbers

   Lambda calculus encoding of natural numbers.
   A Church numeral N is: fun f x -> f (f (... (f x) ...))  (* f applied N times *)

   OCaml has a uniform function representation, so Church numerals are natural
   and clean — no Box<dyn Fn> gymnastics required. *)

(* ── Church numerals as higher-order functions ───────────────────────────── *)

(* A Church numeral: takes a function f and a base x, applies f N times *)
type 'a church = { apply : 'b. ('b -> 'b) -> 'b -> 'b }

(* Zero: apply f zero times *)
let zero  = { apply = fun _f x -> x }

(* One: apply f once *)
let one   = { apply = fun f x -> f x }

(* Two: apply f twice *)
let two   = { apply = fun f x -> f (f x) }

(* Successor: given n, apply f one more time *)
let succ n = { apply = fun f x -> f (n.apply f x) }

(* Add: apply f m times, then n times *)
let add m n = { apply = fun f x -> m.apply f (n.apply f x) }

(* Multiply: apply (n.apply f) m times *)
let mul m n = { apply = fun f x -> m.apply (n.apply f) x }

(* Convert Church numeral to integer *)
let to_int n = n.apply (fun x -> x + 1) 0

(* Convert integer to Church numeral *)
let rec of_int k =
  if k <= 0 then zero
  else succ (of_int (k - 1))

(* Is zero: returns true iff the numeral is zero *)
let is_zero n = n.apply (fun _ -> false) true

(* ── Alternative: inlined function representation (simpler) ─────────────── *)

(* Without the rank-2 type wrapper — uses int as the carrier type *)
let czero  f x = ignore f; x
let cone   f x = f x
let ctwo   f x = f (f x)
let csucc n f x = f (n f x)
let cadd m n f x = m f (n f x)
let cmul m n f x = m (n f) x
let cto_int n = n (fun x -> x + 1) 0

let () =
  (* Church numerals via rank-2 encoding *)
  assert (to_int zero = 0);
  assert (to_int one  = 1);
  assert (to_int two  = 2);
  assert (to_int (succ zero) = 1);
  assert (to_int (succ one)  = 2);
  assert (to_int (add one two) = 3);
  assert (to_int (add (of_int 3) (of_int 4)) = 7);
  assert (to_int (mul two (of_int 3)) = 6);
  assert (to_int (of_int 10) = 10);
  assert (is_zero zero);
  assert (not (is_zero one));

  (* Inline function representation *)
  assert (cto_int czero = 0);
  assert (cto_int cone  = 1);
  assert (cto_int ctwo  = 2);
  assert (cto_int (csucc czero) = 1);
  assert (cto_int (cadd cone ctwo) = 3);
  assert (cto_int (cmul ctwo (csucc ctwo)) = 6);

  (* Church arithmetic: 2 + 3 = 5 *)
  let three = csucc ctwo in
  assert (cto_int (cadd ctwo three) = 5);

  (* Church multiplication: 2 × 4 = 8 *)
  let four = cadd ctwo ctwo in
  assert (cto_int (cmul ctwo four) = 8);

  print_endline "934-church-numerals: all tests passed"
