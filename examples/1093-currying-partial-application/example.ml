(* Currying and Partial Application *)

(* In OCaml, ALL functions are automatically curried.
   `let add x y = x + y` is syntactic sugar for:
   `let add = fun x -> fun y -> x + y` *)

(* Idiomatic OCaml — currying is the default *)
let add x y = x + y

(* Partial application — just supply fewer arguments *)
let add5 = add 5
let add10 = add 10

(* Float version — OCaml needs a separate operator (+.) *)
let add_float x y = x +. y
let add_half = add_float 0.5

(* Generic curry/uncurry combinators *)
let curry f x y = f (x, y)
let uncurry f (x, y) = f x y

(* A tupled add to demonstrate curry combinator *)
let add_tupled (x, y) = x + y

let () =
  (* Direct call *)
  assert (add 3 4 = 7);

  (* Partial application *)
  assert (add5 3 = 8);
  assert (add5 0 = 5);
  assert (add10 1 = 11);

  (* Float partial application *)
  assert (add_half 1.5 = 2.0);

  (* curry combinator: turn tupled function into curried *)
  let curried = curry add_tupled in
  assert (curried 7 3 = 10);

  (* uncurry combinator: turn curried function into tupled *)
  let uncurried = uncurry add in
  assert (uncurried (3, 4) = 7);

  print_endline "ok"
