(* Church Numerals — Functions as Numbers *)
(* Lambda calculus: a numeral N applies f N times to x *)

let zero _f x = x
let one f x = f x
let succ n f x = f (n f x)
let add m n f x = m f (n f x)
let mul m n f = m (n f)
let to_int n = n (fun x -> x + 1) 0

let two = succ one
let three = add one two
let six = mul two three
let nine = mul three three

let () =
  Printf.printf "0=%d 1=%d 2=%d 3=%d 6=%d 9=%d\n"
    (to_int zero) (to_int one) (to_int two)
    (to_int three) (to_int six) (to_int nine);
  assert (to_int zero = 0);
  assert (to_int three = 3);
  assert (to_int six = 6);
  assert (to_int nine = 9);
  Printf.printf "All Church numeral tests passed!\n"
