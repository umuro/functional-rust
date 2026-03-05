(* Church encoding in OCaml *)

(* Church Booleans *)
let church_true  t _ = t
let church_false _ f = f
let church_not b = b church_false church_true
let church_and p q = p q church_false
let church_or  p q = p church_true q
let church_if c t f = c t f

(* Church Numerals *)
let zero f x = x
let succ n f x = f (n f x)
let church_add m n f x = m f (n f x)
let church_mul m n f = m (n f)
let to_int n = n (fun x -> x+1) 0

let one   = succ zero
let two   = succ one
let three = succ two

(* Church Pairs *)
let church_pair a b f = f a b
let church_fst p = p (fun a _ -> a)
let church_snd p = p (fun _ b -> b)

let () =
  Printf.printf "2+3 = %d\n" (to_int (church_add two three));
  Printf.printf "2*3 = %d\n" (to_int (church_mul two three));
  Printf.printf "T and F = %b\n" (church_if (church_and church_true church_false) true false);
  let p = church_pair 42 "hello" in
  Printf.printf "fst=%d snd=%s\n" (church_fst p) (church_snd p)
