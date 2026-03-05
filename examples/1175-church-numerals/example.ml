(* Church Numerals *)
(* Encode natural numbers as higher-order functions *)

(* Church encoding of naturals *)
let zero _f x = x
let succ n f x = f (n f x)
let one = succ zero
let two = succ one
let three = succ two

let add m n f x = m f (n f x)
let mul m n f = m (n f)

let to_int n = n (fun x -> x + 1) 0

let five = add two three
let six = mul two three

let () =
  Printf.printf "2 + 3 = %d\n" (to_int five);
  Printf.printf "2 * 3 = %d\n" (to_int six);
  Printf.printf "3 + 3 = %d\n" (to_int (add three three))
