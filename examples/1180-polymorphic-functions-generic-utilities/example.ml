(* Polymorphic Functions — Generic Utilities *)
(* Write functions that work for any type *)

let compose f g x = f (g x)
let flip f x y = f y x
let const x _y = x
let tap f x = f x; x

let twice f x = f (f x)
let thrice f x = f (f (f x))

let () =
  let inc = ( + ) 1 in
  Printf.printf "twice inc 5 = %d\n" (twice inc 5);
  Printf.printf "thrice double 3 = %d\n" (thrice (( * ) 2) 3);

  let exclaim = (fun s -> s ^ "!") in
  Printf.printf "%s\n" (thrice exclaim "wow")
