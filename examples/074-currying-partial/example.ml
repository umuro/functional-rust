(* 074: Currying and Partial Application *)

(* Approach 1: Natural currying *)
let add x y = x + y
let multiply x y = x * y

(* Partial application — just give fewer arguments *)
let add5 = add 5
let double = multiply 2
let triple = multiply 3

(* Approach 2: Multi-argument currying *)
let make_greeting prefix name suffix =
  Printf.sprintf "%s %s%s" prefix name suffix

let hello_greeting = make_greeting "Hello"
let hello_world = hello_greeting "World"

(* Approach 3: Higher-order with partial application *)
let apply_twice f x = f (f x)
let add10 = apply_twice add5
let quadruple = apply_twice double

let compose f g x = f (g x)
let add5_then_double = compose double add5
let double_then_add5 = compose add5 double

(* Tests *)
let () =
  assert (add5 3 = 8);
  assert (add5 0 = 5);
  assert (double 7 = 14);
  assert (triple 4 = 12);
  assert (hello_world "!" = "Hello World!");
  assert (add10 0 = 10);
  assert (add10 5 = 15);
  assert (quadruple 3 = 12);
  assert (add5_then_double 3 = 16);   (* (3+5)*2 = 16 *)
  assert (double_then_add5 3 = 11);   (* 3*2+5 = 11 *)
  Printf.printf "✓ All tests passed\n"
