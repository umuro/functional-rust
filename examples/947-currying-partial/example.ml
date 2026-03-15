(* 947: Currying, Partial Application, and Sections

   OCaml functions are curried by default.
   `let add x y = x + y` already allows `add 5` as a partial application.
   No closures needed — it's built into the function call syntax. *)

(* ── Curried functions — the OCaml default ───────────────────────────────── *)

(* add is automatically curried: add : int -> int -> int *)
let add x y = x + y

(* Partial application: fix the first argument *)
let add5 = add 5         (* add5 : int -> int *)

(* Equivalently with an explicit lambda *)
let add_curried x = fun y -> x + y

(* ── Operator sections via partial application ───────────────────────────── *)

let double    = ( * ) 2    (* partially apply multiplication *)
let increment = ( + ) 1
let halve x   = x / 2

(* ── compose and pipe ─────────────────────────────────────────────────────── *)

let compose f g x = f (g x)   (* (f ∘ g)(x) = f(g(x)) *)
let ( >> ) g f = compose f g  (* left-to-right composition *)
let ( |> ) = Stdlib.( |> )    (* pipe: x |> f = f x — built into OCaml *)

(* ── Pipeline ────────────────────────────────────────────────────────────── *)

let pipeline initial funcs =
  List.fold_left (fun acc f -> f acc) initial funcs

(* ── curry / uncurry converters ─────────────────────────────────────────── *)

(* curry: convert a pair-taking function to a curried function *)
let curry f x y = f (x, y)

(* uncurry: convert a curried function to a pair-taking function *)
let uncurry f (x, y) = f x y

(* ── Scale-and-shift utility ─────────────────────────────────────────────── *)

let scale_and_shift scale shift x = x * scale + shift

let celsius_of_fahrenheit f = scale_and_shift 5 (-160) f  (* integer approx *)

(* ── flip: swap argument order ───────────────────────────────────────────── *)

let flip f x y = f y x

let () =
  (* add5 via partial application *)
  assert (add5 10 = 15);
  assert (add_curried 3 7 = 10);

  (* operator sections *)
  assert (double 7 = 14);
  assert (increment 9 = 10);
  assert (halve 20 = 10);

  (* pipeline *)
  let result = pipeline 6 [double; increment; halve] in
  (* 6*2=12, 12+1=13, 13/2=6 *)
  assert (result = 6);

  (* |> pipe — built-in OCaml operator *)
  let result2 = 6 |> double |> increment |> halve in
  assert (result2 = 6);

  (* compose with >> *)
  let double_then_inc = double >> increment in
  assert (double_then_inc 5 = 11);  (* 5*2+1 *)

  (* curry / uncurry *)
  let add_pair (x, y) = x + y in
  let curried_add = curry add_pair in
  assert (curried_add 3 4 = 7);

  let uncurried_add = uncurry add in
  assert (uncurried_add (3, 4) = 7);

  (* celsius *)
  assert (celsius_of_fahrenheit 212 = 900);  (* integer arithmetic *)

  (* flip: reverse argument order *)
  let sub x y = x - y in
  let sub_flipped = flip sub in
  assert (sub 10 3 = 7);
  assert (sub_flipped 10 3 = -7);  (* 3 - 10 *)

  (* List.map with partial application *)
  let add3 = add 3 in
  assert (List.map add3 [1; 2; 3; 4] = [4; 5; 6; 7]);

  (* List.filter with partial application *)
  let gt5 = flip ( < ) 5 in   (* x > 5 *)
  assert (List.filter gt5 [3; 6; 2; 8; 1] = [6; 8]);

  print_endline "947-currying-partial: all tests passed"
