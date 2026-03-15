(* 074: Currying and Partial Application
   OCaml functions are curried by default — every function takes one argument *)

(* --- Approach 1: All functions are already curried --- *)

(* No special syntax needed: add is already curried *)
let add x y = x + y
let multiply x y = x * y

(* Partial application: supply just the first arg *)
let add5     = add 5       (* int -> int *)
let double   = multiply 2  (* int -> int *)
let triple   = multiply 3

(* --- Approach 2: Higher-order functions --- *)

let apply_twice f x = f (f x)

(* compose: (b -> c) -> (a -> b) -> (a -> c) *)
let compose f g = fun x -> f (g x)

(* --- Approach 3: Pipelines using partial application --- *)

let greet prefix name suffix =
  Printf.sprintf "%s %s%s" prefix name suffix

(* flip reverses argument order — useful for partial application of right arg *)
let flip f x y = f y x

let () =
  Printf.printf "add5 3 = %d\n" (add5 3);
  Printf.printf "add5 0 = %d\n" (add5 0);
  Printf.printf "double 7 = %d\n" (double 7);
  Printf.printf "triple 4 = %d\n" (triple 4);

  Printf.printf "apply_twice add5 0 = %d\n" (apply_twice add5 0);
  Printf.printf "apply_twice add5 5 = %d\n" (apply_twice add5 5);
  Printf.printf "apply_twice double 3 = %d\n" (apply_twice double 3);

  (* compose: add5 then double = (x+5)*2 *)
  let add5_then_double = compose double add5 in
  Printf.printf "add5_then_double 3 = %d\n" (add5_then_double 3);

  let double_then_add5 = compose add5 double in
  Printf.printf "double_then_add5 3 = %d\n" (double_then_add5 3);

  Printf.printf "greet Hello World! = %s\n" (greet "Hello" "World" "!");

  (* Partial application with flip to fix a right-hand argument *)
  let add_to_10 = flip add 10 in
  Printf.printf "add_to_10 5 = %d\n" (add_to_10 5)
