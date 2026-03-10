(* Curried: int -> int -> int *)
let add x y = x + y
let add5 = add 5             (* partial application *)

(* Tupled: NOT the OCaml default *)
let add_tup (x, y) = x + y

(* Converters between the two styles *)
let curry   f x y = f (x, y)
let uncurry f (x, y) = f x y

(* Operator sections via partial application *)
let double    = ( * ) 2
let increment = ( + ) 1
let halve     = Fun.flip ( / ) 2   (* flip swaps argument order *)

(* Labeled arguments allow any-order partial application *)
let scale_and_shift ~scale ~shift x = x * scale + shift
let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)

let () =
  (* partial application *)
  assert (add5 10 = 15);
  assert (add5 0  = 5);
  assert (add_tup (3, 4) = 7);
  (* curry / uncurry round-trip *)
  assert (curry add_tup 3 4 = 7);
  assert (uncurry add5 (0, 10) = 10);
  (* operator sections *)
  assert (double 7    = 14);
  assert (increment 9 = 10);
  assert (halve 20    = 10);
  assert (halve 7     = 3);   (* integer division *)
  (* pipeline *)
  let pipeline = [double; increment; halve] in
  let result = List.fold_left (fun acc f -> f acc) 6 pipeline in
  assert (result = 6);        (* 6→12→13→6 *)
  (* labeled-argument partial application *)
  assert (celsius_of_fahrenheit 32  = 0);   (* 32*5 - 160 = 0 *)
  assert (celsius_of_fahrenheit 212 = 900); (* 212*5 - 160 = 900 *)
  Printf.printf "add5 10   = %d\n" (add5 10);
  Printf.printf "double 7  = %d\n" (double 7);
  Printf.printf "halve 20  = %d\n" (halve 20);
  Printf.printf "6 |> *2 |> +1 |> /2 = %d\n" result;
  Printf.printf "212F in celsius_of_fahrenheit = %d\n" (celsius_of_fahrenheit 212);
  print_endline "ok"
